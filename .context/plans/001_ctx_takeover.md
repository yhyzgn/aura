# 001 CTX Takeover

- 状态: 已完成
- 负责人: Codex
- 当前任务: `.context/tasks/001_ctx_takeover.md`

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
