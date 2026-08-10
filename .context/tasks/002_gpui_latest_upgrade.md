# 002 GPUI Latest Upgrade Execution

- 状态: 进行中
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

- 不发布 release/tag。
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

待完成。
