# 002 GPUI Latest Upgrade Execution

- 状态: 已完成
- 计划: `.context/plans/002_gpui_latest_upgrade.md`
- 规模: 大
- 依赖: 官方 `zed-industries/zed` GPUI main
- 是否改变生产行为: 是

## 1. 任务目标

在独立分支完成 GPUI 最新 main 升级，并使 Liora 代码、示例、文档与测试适配新能力和破坏性变更。

## 2. 范围

- `Cargo.toml` / `Cargo.lock` GPUI 相关 revision。
- GPUI API 适配影响到的 `crates/**`、`apps/**`、`xtask/**`、文档和测试。
- 与新 GPUI 能力相关的 SDK 公共 API、Gallery/Docs 示例或开发指南。

## 3. 非目标

- 不在 GPUI 适配任务中实现额外的发布流水线功能；版本发布由独立 release 步骤执行。
- 不进行无关组件重构。
- 不引入非官方 GPUI fork。

## 4. 预期文件

- `Cargo.toml`
- `Cargo.lock`
- `crates/**`
- `apps/liora-gallery/**`
- `apps/liora-docs/**`
- `README*.md` / `docs/**`（如公共能力变化需要）
- `.context/**`

## 5. 验收标准

- GPUI revision 指向官方最新 main commit。
- 编译错误全部修复或明确上游阻塞。
- 新特性清单逐项处置，不遗漏。
- SDK crate 不耦合 Gallery/Docs 业务。

## 6. 验证

```bash
python scripts/context_bootstrap.py validate --root .
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p liora-docs --bin check_snippets
git diff --check -- . ':(exclude).omx'
```

## 7. 风险与回滚

- 风险: GPUI View 统一导致大量 Render/RenderOnce API 迁移。
- 风险: lockfile 依赖升级引入平台特定编译问题。
- 回滚: 删除本分支或回退升级提交。

## 8. 完成记录

- GPUI 和 `gpui_platform` 已统一指向官方 Zed revision `492acd6c815cbe8c7366d54e6092341340afa6c7`。
- 已通过 workspace 编译、workspace 测试、Docs snippet 编译、workspace 文档生成、CTX 校验和 release readiness 检查。
- SDK 仍保持 app-agnostic，Gallery/Docs 业务未进入 SDK crate。
