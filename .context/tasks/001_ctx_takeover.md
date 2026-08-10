# 001 CTX Takeover Execution

- 状态: 已完成
- 计划: `.context/plans/001_ctx_takeover.md`
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

完成于 2026-08-10。

实际结果：

- 已创建 `scripts/context_bootstrap.py`，支持 `init`、`migrate-auto`、`audit-migration`、`finalize`、`validate`。
- 已创建 `.context/README.md`、`.context/system/*`、`.context/plans/*`、`.context/tasks/*`。
- 已将 `prompt.md`、`.memory/`、`.prompt/` 内容级迁移到规范目标，并把原始来源归档到 `.context/archive/legacy-sources/`。
- 已更新 `AGENTS.md` 为 ctx 入口，并保留 legacy source markers。
- 已修正 Docs 测试和文档中对旧上下文入口的活跃引用。

验证证据：

```bash
python scripts/context_bootstrap.py audit-migration --root .
# migration audit OK

python scripts/context_bootstrap.py validate --root .
# context validation OK

cargo check -p liora-docs --bin check_snippets
# Finished dev profile successfully

cargo test -p liora-docs release_candidate_readiness_docs_cover_current_boundaries
# 1 passed

cargo test -p liora-docs public_readmes_do_not_expose_internal_draft_scaffolding
# 1 passed

git diff --check -- . ':(exclude).omx'
# passed
```
