# 002 GPUI Latest Upgrade

- 状态: 进行中
- 负责人: Codex
- 当前任务: `.context/tasks/002_gpui_latest_upgrade.md`

## 目标

在独立分支升级 Liora 到官方 `zed-industries/zed` GPUI 最新 `main` revision，并完整评估、接入前置调研中列出的 GPUI 新能力。

## 非目标

- 不发布版本。
- 不把 Gallery/Docs 业务逻辑耦合进 SDK crate。
- 不引入 WebView/Tauri/HTML/CSS/DOM runtime。

## 约束和依赖

- 必须使用官方 `zed-industries/zed` GPUI 源。
- 先升级 pinned revision，再按编译结果修复破坏性 API 变化。
- 覆盖新能力清单：同步旋转动画、accessibility_id、WebGL/web 依赖变化、container_query、div 滚动改进、系统通知 API、Window 状态/attention/embedded、Wayland popup/input region/layer-shell/drag、Render/RenderOnce -> View 迁移风险。

## 阶段

1. 建立分支和升级基线。
2. 更新 GPUI/GPU 平台相关 git revision 与 lockfile。
3. 修复 workspace 编译失败。
4. 接入适合 Liora 的公共 API/示例/文档与测试，不耦合 demo 业务。
5. 运行验证并记录风险。

## 检查点和计划级风险

- 检查点: `cargo check --workspace --all-targets` 至少通过或明确记录上游阻塞。
- 风险: 上游 GPUI pre-1.0 破坏性变更可能影响大量组件。
- 风险: 部分平台特性只能跨平台编译验证，无法在当前 Linux 会话完整手工验证 macOS/Windows/Web runtime。

## 完成标准

- 分支包含 GPUI 最新 revision 升级。
- 新能力清单逐项有代码接入、文档说明、测试/验证或明确不可接入原因。
- 验证结果写入任务完成记录。
