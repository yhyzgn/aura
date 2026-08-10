#!/usr/bin/env python3
"""Repository-local CTX bootstrap, migration, audit, finalize, and validate tool.

This implementation is intentionally standard-library only. It supports the CTX
contract needed by this repository: create canonical `.context/` assets, migrate
legacy `prompt.md`, `.memory/`, and `.prompt/` content units with source markers,
archive old roots, and validate the resulting pointers.
"""

from __future__ import annotations

import argparse
import datetime as _dt
import hashlib
import json
import os
import re
import shutil
import sys
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Iterable

ROOT_MARKERS = ("Cargo.toml", "crates", "apps")
LEGACY_ROOTS = ("prompt.md", ".memory", ".prompt")
STATE_VERSION = 2
TODAY = _dt.date.today().isoformat()

CURRENT_PLAN = ".context/plans/001_ctx_takeover.md"
CURRENT_TASK = ".context/tasks/001_ctx_takeover.md"

HEADER_RE = re.compile(r"^(#{1,6})\s+.+$", re.MULTILINE)
MARKER_RE = re.compile(
    r'<!-- ctx-migration source="(?P<source>[^"]+)" unit="(?P<unit>[^"]+)" sha256="(?P<sha>[0-9a-f]{64})" -->'
)


@dataclass
class Unit:
    unit: str
    source_path: str
    start_line: int
    end_line: int
    sha256: str
    content: str
    category: str
    destination: str
    status: str = "migrated"
    reason: str = "Preserved during CTX takeover as verified legacy context."


@dataclass
class SourceRecord:
    path: str
    archived_path: str | None
    sha256: str
    units: list[dict]


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_text(text: str) -> str:
    return sha256_bytes(text.encode("utf-8"))


def rel(path: Path, root: Path) -> str:
    return path.relative_to(root).as_posix()


def safe_write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def append_if_missing(path: Path, text: str) -> None:
    if not path.exists():
        safe_write(path, text)


def repo_kind(root: Path) -> str:
    return "existing" if any((root / marker).exists() for marker in ROOT_MARKERS) else "new"


def legacy_paths(root: Path, include_archive: bool = False) -> list[Path]:
    paths: list[Path] = []
    for item in LEGACY_ROOTS:
        p = root / item
        if p.is_file():
            paths.append(p)
        elif p.is_dir():
            paths.extend(sorted(f for f in p.rglob("*") if f.is_file()))
    if include_archive:
        archive = root / ".context/archive/legacy-sources"
        for item in LEGACY_ROOTS:
            p = archive / item
            if p.is_file():
                paths.append(p)
            elif p.is_dir():
                paths.extend(sorted(f for f in p.rglob("*") if f.is_file()))
    return sorted(set(paths))


def canonical_source_path(path: Path, root: Path) -> str:
    raw = rel(path, root)
    prefix = ".context/archive/legacy-sources/"
    if raw.startswith(prefix):
        return raw[len(prefix):]
    return raw


def unit_id(source: str, index: int, content: str) -> str:
    base = re.sub(r"[^a-zA-Z0-9]+", "-", source).strip("-").lower() or "unit"
    return f"{base}-{index:04d}-{sha256_text(content)[:12]}"


def split_markdown_units(source: str, text: str) -> list[tuple[int, int, str]]:
    lines = text.splitlines(keepends=True)
    if not text.strip():
        return []
    matches = list(HEADER_RE.finditer(text))
    if not matches:
        units: list[tuple[int, int, str]] = []
        start = 0
        idx = 0
        for part in re.split(r"(\n\s*\n)", text):
            if not part.strip():
                start += part.count("\n")
                continue
            idx += 1
            start_line = start + 1
            end_line = start + part.count("\n") + (0 if part.endswith("\n") else 1)
            units.append((start_line, end_line, part if part.endswith("\n") else part + "\n"))
            start += part.count("\n")
        return units or [(1, len(lines), text if text.endswith("\n") else text + "\n")]

    starts = [text[:m.start()].count("\n") for m in matches]
    units = []
    for i, start_line_zero in enumerate(starts):
        end_line_zero = starts[i + 1] if i + 1 < len(starts) else len(lines)
        content = "".join(lines[start_line_zero:end_line_zero])
        units.append((start_line_zero + 1, end_line_zero, content))
    prefix = "".join(lines[: starts[0]])
    if prefix.strip():
        units.insert(0, (1, starts[0], prefix))
    return units


def classify(source: str, content: str) -> tuple[str, str]:
    if source == "prompt.md":
        return "plan", ".context/plans/000_legacy_phase_history.md"
    if source.startswith(".prompt/"):
        return "plan", ".context/plans/000_legacy_phase_history.md"
    if source == ".memory/decisions.md":
        return "convention", ".context/system/conventions.md"
    if source == ".memory/inventory.md":
        return "system_fact", ".context/system/overview.md"
    if source == ".memory/state.md":
        return "system_fact", ".context/system/overview.md"
    if source == ".memory/sessions.md":
        return "task", ".context/tasks/000_legacy_session_history.md"
    if "red line" in content.lower() or "风险" in content or "forbidden" in content.lower():
        return "risk", ".context/system/risks.md"
    return "system_fact", ".context/system/overview.md"


def collect_units(root: Path, include_archive: bool = False) -> tuple[list[SourceRecord], list[Unit]]:
    records: list[SourceRecord] = []
    units: list[Unit] = []
    for path in legacy_paths(root, include_archive=include_archive):
        source = canonical_source_path(path, root)
        try:
            data = path.read_bytes()
            text = data.decode("utf-8")
        except UnicodeDecodeError:
            continue
        file_sha = sha256_bytes(data)
        source_units = []
        for index, (start, end, content) in enumerate(split_markdown_units(source, text), start=1):
            category, destination = classify(source, content)
            u = Unit(
                unit=unit_id(source, index, content),
                source_path=source,
                start_line=start,
                end_line=end,
                sha256=sha256_text(content),
                content=content,
                category=category,
                destination=destination,
            )
            units.append(u)
            source_units.append(
                {
                    "unit": u.unit,
                    "start_line": start,
                    "end_line": end,
                    "sha256": u.sha256,
                    "status": u.status,
                    "category": u.category,
                    "destinations": [u.destination],
                    "reason": u.reason,
                }
            )
        archived = None
        raw = rel(path, root)
        if raw.startswith(".context/archive/legacy-sources/"):
            archived = raw
        records.append(SourceRecord(path=source, archived_path=archived, sha256=file_sha, units=source_units))
    return records, units


def base_files(root: Path) -> None:
    append_if_missing(
        root / ".context/README.md",
        f"""# Liora CTX Context

## Canonical reading order

1. `AGENTS.md`
2. `.context/README.md`
3. Current plan: `{CURRENT_PLAN}`
4. Current task: `{CURRENT_TASK}`
5. `.context/system/overview.md`
6. `.context/system/conventions.md`
7. `.context/system/risks.md`

## Directory responsibilities

- `system/`: stable verified facts, conventions, and risks.
- `plans/`: bounded phase plans and migrated phase history.
- `tasks/`: bounded execution tasks and migrated session history.
- `archive/`: byte-preserving legacy source archive after migration finalization; not a primary reading source.

## Lifecycle

- Plan status: `草稿 -> 进行中 -> 暂停 -> 已完成 | 已取代`.
- Task status: `待开始 -> 进行中 -> 已完成 | 阻塞 | 已取代`.
- Update stable facts in `system/`; update plan/task files when scope, acceptance, validation, or status changes.

## Current pointers

The authoritative current plan/task pointers live in `AGENTS.md` and currently point to `{CURRENT_PLAN}` and `{CURRENT_TASK}`.
""",
    )
    append_if_missing(
        root / ".context/system/overview.md",
        """# System Overview

## Verified project summary

Liora is a Rust edition 2024 Cargo workspace for a pure Rust + GPUI native enterprise UI component library. Evidence: `Cargo.toml`, `crates/`, `apps/liora-gallery`, `apps/liora-docs`, `README.md`.

## Workspace topology

- `crates/liora`: one-stop SDK facade.
- `crates/liora-core`: global config, initialization, locale/codegen support, utility traits.
- `crates/liora-theme`: design tokens, theme modes, font configuration.
- `crates/liora-icons` and `crates/liora-icons-*`: icon primitive, asset loading, bundled typed icon libraries.
- `crates/liora-icons-optimizer`: build-time SVG bundle optimizer.
- `crates/liora-components`: reusable GPUI components.
- `crates/liora-tray`, `liora-packager`, `liora-updater`: reusable app-shell/packaging/update SDK crates.
- `apps/liora-gallery`: native component gallery.
- `apps/liora-docs`: native documentation application.

## Build, test, and release commands

Verified command set from `README.md`, `.github/workflows/ci.yml`, and local release history:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p liora-docs --bin check_snippets
cargo run --release -p xtask -- package validate
cargo run --release -p xtask -- package release-readiness
git diff --check -- . ':(exclude).omx'
```

## Current release state

Latest local release commit/tag before CTX takeover: `v0.2.9` pushed from `main` on 2026-07-06 session history. Re-verify with `git tag --sort=-v:refname | head` before future releases.

""",
    )
    append_if_missing(
        root / ".context/system/conventions.md",
        """# Conventions

## Core engineering conventions

- Keep Liora pure Rust + native GPUI. Do not introduce Tauri, WebView, HTML/CSS/DOM runtime paths, or browser shells.
- Prefer official `zed-industries/zed` GPUI sources and the exact pinned revision in `Cargo.lock`/`Cargo.toml` for GPUI behavior.
- Keep SDK crates business-agnostic. Gallery/Docs examples and app-specific resources must not leak into reusable SDK modules.
- Components should follow builder-style public APIs and implement GPUI `RenderOnce`/`IntoElement` patterns used across `crates/liora-components`.
- Public types should not be prefixed with `Liora`; crate/module namespaces provide context.
- Prefer existing utilities and focused modules over dumping logic into public entry files.

## Documentation conventions

- README changes are required when public APIs, features, dependency/setup guidance, packaging, updater, locales, icons, CI, or release behavior changes.
- Docs snippets under `apps/liora-docs/content/snippets` should remain compile-checked by `cargo check -p liora-docs --bin check_snippets`.

""",
    )
    append_if_missing(
        root / ".context/system/risks.md",
        """# Risks

## R-001 — GPUI upstream drift

- Severity: High
- Evidence: `Cargo.toml` pins `gpui` and `gpui_platform` to official `zed-industries/zed` revisions.
- Trigger: GPUI/Zed API changes, platform behavior changes, or dependency upgrades.
- Failure mode: compile errors, runtime panics, packaging regressions, or platform-specific behavior differences.
- Mitigation: verify against official upstream sources and run focused compile/tests before claiming completion.
- Rollback: revert dependency/API changes and restore the previous lockfile revision.
- Status: Open.

## R-002 — SDK/application coupling

- Severity: High
- Evidence: prior tray/packager/updater/icon optimizer work separated SDK code from Gallery/Docs business behavior.
- Trigger: moving demo app assets, menus, updater selectors, or business flows into reusable crates.
- Failure mode: downstream developers inherit Liora demo behavior or resources unintentionally.
- Mitigation: keep Gallery/Docs code in `apps/`; SDK crates expose generic configuration and primitives only.
- Rollback: move business-specific code back to app layer and add tests/docs for generic boundaries.
- Status: Open.

## R-003 — Legacy context migration semantic loss

- Severity: Medium
- Evidence: CTX takeover archives `prompt.md`, `.memory/`, and `.prompt/` after content-unit migration.
- Trigger: future agents relying on old paths or failing to read `.context/` first.
- Failure mode: stale phase state, missed project red lines, or broken compile-time include paths.
- Mitigation: keep source markers, migration state, archive bytes, and tests updated to canonical `.context` paths.
- Rollback: restore archived legacy sources from `.context/archive/legacy-sources/` if needed.
- Status: Open.

""",
    )
    append_if_missing(
        root / ".context/plans/001_ctx_takeover.md",
        f"""# 001 CTX Takeover

- 状态: 进行中
- 负责人: Codex
- 当前任务: `{CURRENT_TASK}`

## 目标

建立 `AGENTS.md` + `.context/` 作为唯一规范上下文入口，迁移旧 `prompt.md`、`.memory/`、`.prompt/` 内容并归档旧入口。

## 非目标

- 不改变产品功能行为。
- 不发布新版本。
- 不重写历史文档内容，只迁移和标记来源。

## 约束和依赖

- 保留人工说明和源码事实。
- 不创建根级 `.agents/`、`.claude/`、`.cursor/`、`.gemini/` 技能目录。
- 迁移后必须运行 `scripts/context_bootstrap.py validate --root .`。

## 阶段

1. 预检旧上下文和仓库类型。
2. 创建 `.context` 脚手架。
3. 逐内容单元迁移旧上下文并写入来源标记。
4. 归档旧入口。
5. 修正旧路径引用并运行验证。

## 阶段退出标准

- `.context` 规范文件存在且当前计划/任务互相指向。
- 遗留入口不再位于根目录读取路径。
- 迁移审计和 validate 通过。

## 检查点和计划级风险

- 检查点: 归档前 `audit-migration` 必须通过。
- 风险: 测试中硬编码 `prompt.md` / `.memory` / `.prompt` 路径，需要同步更新。

## 完成标准

- `AGENTS.md` 指向当前 plan/task。
- `.context/migration-state.json` 覆盖所有旧内容单元。
- `prompt.md`、`.memory/`、`.prompt/` 移至 `.context/archive/legacy-sources/`。
- 验证命令完成并记录在任务完成记录中。
""",
    )
    append_if_missing(
        root / ".context/tasks/001_ctx_takeover.md",
        f"""# 001 CTX Takeover Execution

- 状态: 进行中
- 计划: `{CURRENT_PLAN}`
- 规模: 中
- 依赖: 无
- 是否改变生产行为: 否

## 1. 任务目标

完成当前仓库的 CTX 接管，使 `AGENTS.md` 和 `.context/` 成为唯一规范上下文入口。

## 2. 范围

- 新增/更新 `.context/`。
- 更新 `AGENTS.md`。
- 迁移并归档 `prompt.md`、`.memory/`、`.prompt/`。
- 修正代码/文档中对旧上下文入口的引用。

## 3. 非目标

- 不改组件功能。
- 不改发布 tag。
- 不删除源码、README、Docs 页面或设计文档。

## 4. 预期文件

- `AGENTS.md`
- `.context/**`
- `scripts/context_bootstrap.py`
- `apps/liora-docs/src/markdown.rs`
- `docs/release-candidate-checklist.md`

## 5. 验收标准

- `scripts/context_bootstrap.py validate --root .` 通过。
- `git diff --check -- . ':(exclude).omx'` 通过。
- Docs snippet check 至少能编译受影响 include/assert 区域。

## 6. 验证

```bash
python scripts/context_bootstrap.py audit-migration --root .
python scripts/context_bootstrap.py validate --root .
cargo check -p liora-docs --bin check_snippets
git diff --check -- . ':(exclude).omx'
```

## 7. 风险与回滚

- 风险: 历史上下文量大，迁移文件体积增加。
- 风险: 旧 include 路径失效导致编译失败。
- 回滚: `git restore .` 可恢复本次未提交变更；归档字节保留在 `.context/archive/legacy-sources/`。

## 8. 完成记录

待验证后填写。
""",
    )


def managed_block(units: Iterable[Unit], destination: str) -> str:
    out = [
        "\n<!-- ctx-managed-legacy-migration:start -->\n",
        "\n## Migrated legacy source units\n\n",
        "The following sections preserve legacy context content verbatim enough for auditability. Prefer the summarized CTX sections above for day-to-day work.\n\n",
    ]
    for u in units:
        out.append(f"### {u.unit}\n\n")
        out.append(f'<!-- ctx-migration source="{u.source_path}" unit="{u.unit}" sha256="{u.sha256}" -->\n\n')
        out.append(u.content)
        if not u.content.endswith("\n"):
            out.append("\n")
        out.append("\n")
    out.append("<!-- ctx-managed-legacy-migration:end -->\n")
    return "".join(out)


def replace_managed_block(path: Path, block: str) -> None:
    text = path.read_text(encoding="utf-8") if path.exists() else ""
    pattern = re.compile(r"\n?<!-- ctx-managed-legacy-migration:start -->.*?<!-- ctx-managed-legacy-migration:end -->\n?", re.S)
    text = pattern.sub("\n", text).rstrip() + "\n" + block
    safe_write(path, text)


def migrate_auto(root: Path) -> None:
    base_files(root)
    records, units = collect_units(root, include_archive=True)
    by_dest: dict[str, list[Unit]] = {}
    for u in units:
        by_dest.setdefault(u.destination, []).append(u)
    for dest, dest_units in by_dest.items():
        replace_managed_block(root / dest, managed_block(dest_units, dest))
    state = {
        "version": STATE_VERSION,
        "generated_at": _dt.datetime.now().isoformat(timespec="seconds"),
        "mode": repo_kind(root),
        "sources": [asdict(record) for record in records],
    }
    safe_write(root / ".context/migration-state.json", json.dumps(state, indent=2, ensure_ascii=False) + "\n")
    lines = ["# CTX migration map\n\n", f"Generated: {state['generated_at']}\n\n", "| Source | Units | Destination(s) |\n| --- | ---: | --- |\n"]
    for record in records:
        dests = sorted({d for unit in record.units for d in unit.get("destinations", [])})
        lines.append(f"| `{record.path}` | {len(record.units)} | {', '.join(f'`{d}`' for d in dests)} |\n")
    safe_write(root / ".context/migration-map.md", "".join(lines))


def audit(root: Path, allow_archive: bool = True) -> tuple[bool, list[str]]:
    errors: list[str] = []
    state_path = root / ".context/migration-state.json"
    if not state_path.exists():
        return False, ["missing .context/migration-state.json"]
    state = json.loads(state_path.read_text(encoding="utf-8"))
    if state.get("version") != STATE_VERSION:
        errors.append("migration state is not version 2")
    _, current_units = collect_units(root, include_archive=allow_archive)
    current_by_id = {u.unit: u for u in current_units}
    expected_units = []
    for source in state.get("sources", []):
        for unit in source.get("units", []):
            expected_units.append((source, unit))
    for source, unit in expected_units:
        unit_id_ = unit.get("unit")
        sha = unit.get("sha256")
        status = unit.get("status")
        category = unit.get("category")
        destinations = unit.get("destinations", [])
        if status != "migrated":
            errors.append(f"{unit_id_}: unsupported non-migrated status {status}")
            continue
        if not destinations:
            errors.append(f"{unit_id_}: missing destinations")
            continue
        for dest in destinations:
            dest_path = root / dest
            if not dest_path.exists():
                errors.append(f"{unit_id_}: destination missing {dest}")
                continue
            text = dest_path.read_text(encoding="utf-8")
            marker = f'<!-- ctx-migration source="{source.get("path")}" unit="{unit_id_}" sha256="{sha}" -->'
            count = text.count(marker)
            if count != 1:
                errors.append(f"{unit_id_}: marker count in {dest} is {count}")
            if unit_id_ in current_by_id and current_by_id[unit_id_].content not in text:
                errors.append(f"{unit_id_}: full source content missing from {dest}")
        if category == "agent_rule" and "AGENTS.md" not in destinations:
            errors.append(f"{unit_id_}: agent_rule must target AGENTS.md")
        if category == "convention" and not any(d.endswith("system/conventions.md") for d in destinations):
            errors.append(f"{unit_id_}: convention must target conventions")
        if category == "risk" and not any(d.endswith("system/risks.md") for d in destinations):
            errors.append(f"{unit_id_}: risk must target risks")
        if category == "system_fact" and not any(d.endswith("system/overview.md") for d in destinations):
            errors.append(f"{unit_id_}: system_fact must target overview")
        if category == "plan" and not any(d.startswith(".context/plans/") for d in destinations):
            errors.append(f"{unit_id_}: plan must target plans")
        if category == "task" and not any(d.startswith(".context/tasks/") for d in destinations):
            errors.append(f"{unit_id_}: task must target tasks")
    return not errors, errors


def finalize(root: Path, apply: bool) -> None:
    ok, errors = audit(root, allow_archive=False)
    if not ok:
        raise SystemExit("migration audit failed before finalize:\n" + "\n".join(errors[:50]))
    archive = root / ".context/archive/legacy-sources"
    moves = []
    for item in LEGACY_ROOTS:
        src = root / item
        if src.exists():
            dest = archive / item
            if dest.exists():
                raise SystemExit(f"archive target already exists: {dest}")
            moves.append((src, dest))
    if not apply:
        print("finalize dry-run")
        for src, dest in moves:
            print(f"move {src.relative_to(root)} -> {dest.relative_to(root)}")
        return
    manifest = {
        "created_at": _dt.datetime.now().isoformat(timespec="seconds"),
        "moves": [{"source": rel(s, root), "destination": rel(d, root)} for s, d in moves],
    }
    safe_write(root / ".context/archive/finalize-transaction.json", json.dumps(manifest, indent=2) + "\n")
    archive.mkdir(parents=True, exist_ok=True)
    for src, dest in moves:
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.move(str(src), str(dest))
    # Recompute archived paths in state.
    state_path = root / ".context/migration-state.json"
    state = json.loads(state_path.read_text(encoding="utf-8"))
    for source in state.get("sources", []):
        source["archived_path"] = f".context/archive/legacy-sources/{source['path']}"
    safe_write(state_path, json.dumps(state, indent=2, ensure_ascii=False) + "\n")


def validate(root: Path) -> tuple[bool, list[str]]:
    errors: list[str] = []
    required = [
        "AGENTS.md",
        ".context/README.md",
        ".context/system/overview.md",
        ".context/system/conventions.md",
        ".context/system/risks.md",
        CURRENT_PLAN,
        CURRENT_TASK,
    ]
    for item in required:
        if not (root / item).exists():
            errors.append(f"missing required context file: {item}")
    agents = (root / "AGENTS.md").read_text(encoding="utf-8") if (root / "AGENTS.md").exists() else ""
    managed_stripped = re.sub(
        r"\n?<!-- ctx-managed-legacy-migration:start -->.*?<!-- ctx-managed-legacy-migration:end -->\n?",
        "\n",
        agents,
        flags=re.S,
    )
    for heading in [
        "Reading order",
        "Project summary",
        "Business boundaries",
        "Invariants",
        "Technical red lines",
        "Lifecycle",
        "External infrastructure",
        "Safe validation commands",
        "Risk and rollback",
        "Review, commit, and push policy",
        "Legacy migration coverage",
        "Current context pointers",
    ]:
        if heading not in agents:
            errors.append(f"AGENTS.md missing heading: {heading}")
    if managed_stripped.count("Current plan:") != 1 or managed_stripped.count("Current task:") != 1:
        errors.append("AGENTS.md must contain exactly one Current plan and one Current task pointer outside the managed legacy block")
    if CURRENT_PLAN not in agents or CURRENT_TASK not in agents:
        errors.append("AGENTS.md current pointers do not match canonical files")
    if (root / CURRENT_PLAN).exists() and CURRENT_TASK not in (root / CURRENT_PLAN).read_text(encoding="utf-8"):
        errors.append("current plan does not point to current task")
    if (root / CURRENT_TASK).exists() and CURRENT_PLAN not in (root / CURRENT_TASK).read_text(encoding="utf-8"):
        errors.append("current task does not point to current plan")
    for item in LEGACY_ROOTS:
        if (root / item).exists():
            errors.append(f"legacy context root still present: {item}")
    ok, audit_errors = audit(root, allow_archive=True)
    if not ok:
        errors.extend(audit_errors)
    return not errors, errors


def cmd_init(args: argparse.Namespace) -> None:
    root = Path(args.root).resolve()
    mode = repo_kind(root) if args.mode == "auto" else args.mode
    planned = [
        ".context/README.md",
        ".context/system/overview.md",
        ".context/system/conventions.md",
        ".context/system/risks.md",
        CURRENT_PLAN,
        CURRENT_TASK,
    ]
    print(json.dumps({
        "root": str(root),
        "mode": mode,
        "apply": args.apply,
        "legacy_sources": [canonical_source_path(p, root) for p in legacy_paths(root)],
        "planned_files": planned,
        "preserve": [p for p in ["AGENTS.md"] if (root / p).exists()],
    }, indent=2, ensure_ascii=False))
    if args.apply:
        base_files(root)
        records, _ = collect_units(root, include_archive=False)
        if records and not (root / ".context/migration-state.json").exists():
            state = {
                "version": STATE_VERSION,
                "generated_at": _dt.datetime.now().isoformat(timespec="seconds"),
                "mode": mode,
                "sources": [asdict(record) for record in records],
            }
            safe_write(root / ".context/migration-state.json", json.dumps(state, indent=2, ensure_ascii=False) + "\n")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser()
    sub = parser.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("init")
    p.add_argument("--root", default=".")
    p.add_argument("--mode", choices=["auto", "new", "existing"], default="auto")
    p.add_argument("--apply", action="store_true")
    p = sub.add_parser("migrate-auto")
    p.add_argument("--root", default=".")
    p = sub.add_parser("audit-migration")
    p.add_argument("--root", default=".")
    p = sub.add_parser("finalize")
    p.add_argument("--root", default=".")
    p.add_argument("--apply", action="store_true")
    p = sub.add_parser("validate")
    p.add_argument("--root", default=".")
    args = parser.parse_args(argv)
    root = Path(getattr(args, "root", ".")).resolve()
    if args.cmd == "init":
        cmd_init(args)
    elif args.cmd == "migrate-auto":
        migrate_auto(root)
        print("migration auto-applied")
    elif args.cmd == "audit-migration":
        ok, errors = audit(root, allow_archive=True)
        if ok:
            print("migration audit OK")
        else:
            print("migration audit FAILED", file=sys.stderr)
            print("\n".join(errors[:200]), file=sys.stderr)
            return 1
    elif args.cmd == "finalize":
        finalize(root, args.apply)
        print("finalize OK" if args.apply else "finalize dry-run OK")
    elif args.cmd == "validate":
        ok, errors = validate(root)
        if ok:
            print("context validation OK")
        else:
            print("context validation FAILED", file=sys.stderr)
            print("\n".join(errors[:200]), file=sys.stderr)
            return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
