# Session History

## Session 5 — 2026-05-04 (Late Night)

### Actions
- **Input 组件深度增强**:
  - 实现光标闪烁逻辑 (500ms 周期，仅在 Focus 时激活)
  - 实现全选功能 (`SelectAll`, Cmd/Ctrl+A)
  - 实现剪贴板集成 (Copy/Paste/Cut, Cmd/Ctrl+C/V/X)
  - 实现鼠标选择逻辑 (双击选单词、三击全选、拖拽选择、Shift+点击选择)
  - 实现键盘选择逻辑 (Shift+方向键/Home/End)
  - 修正多行模式下的鼠标索引计算 (通过存储 `last_line_layouts`)
  - 修正多行模式下的选择区域渲染 (支持跨行选择背景)
- **表单组件通用增强**:
  - `Checkbox`, `Radio`, `Switch` 支持键盘操作 (Space/Enter 触发)
  - `Checkbox`, `Radio`, `Switch` 增加 Focus 视觉反馈 (Ring/Border 变色)
  - `RadioGroup` 支持方向键切换选项
  - 统一注册各组件键盘绑定到 Gallery
- **清理与优化**:
  - 修复 `checkbox` 与 `switch` 之间 `Toggle` 动作冲突 (重命名为专用名称)
  - 修复 `input` 与 `radio_group` 之间 `Up`/`Down` 动作冲突
  - 移除各文件中的冗余 import (`px`, `MouseUpEvent` 等)
  - 统一 `cx.spawn` 使用 `async move |this, mut cx|` 模式
- **状态同步**:
  - 更新 `.memory/state.md` 记录 P2 进度

### Key Discoveries
- GPUI 0.2.x 中 `cx.spawn` 在 `Context<T>` 上使用时，闭包签名需匹配 `async move |this, mut cx|` 以正确推导 `AsyncApp` 并处理生命周期
- 多行文本交互需要持久化所有行的 `ShapedLine` 布局信息，否则 `index_for_x` 无法跨行工作
- `this.update(cx, ...)` 在 `AsyncApp` 环境下直接传递 `cx` 即可 (无需 `&mut cx` 若 `cx` 已经是引用)

### Decisions Made
- 录入 P2 首个核心组件 `Input` 的完整交互逻辑，作为后续复杂组件的参考

## Session 2 — 2026-05-03

### Actions
- codex 重构 button 为 RenderOnce + IntoElement
- codex 消除 .build(theme) 传参模式
- codex 实现按钮内置唯一 ID

### Key Discoveries
- GPUI RenderOnce 适合无状态一次性组件
- Component::new() 包装后可直接用于 .child()

## Session 1 — 2026-05-03

### Actions
- 搭建 Cargo workspace 结构 (4 crate + 2 app per structure.txt)
- 实现 aura-theme: Theme, Design Tokens, light/dark 模式
- 实现 aura-core: Config (Global), init_aura(), ContextExt, Z-Index utils
- 实现 aura-icons: AuraIcon trait, IconSize, 10 个占位图标
- 实现 aura-components: AuraButton (6 variants × 3 sizes × disabled/loading)
- 适配 GPUI 0.2.2 API (Render trait, Context<'_, V>, InteractiveElement, AnyElement)
- 解决 GPUI feature 策略: 显式 features 替代 default-features=true
- 实现 aura-gallery: 分类卡片式组件看板
- 编写 architecture-design.md: 完整项目设计文档
- 搭建 .memory/ + .prompt/ + prompt.md 协作基础设施

### Key Discoveries
- GPUI 0.2.2 中 `StatefulInteractiveElement` 仅在 `.id()` 之后可用
- `.active()` 和 `.on_click()` 需要 `Stateful<Div>` 包裹
- `.when()` / `.when_some()` 在 0.2.2 中已移除
- `default-features = true` 覆盖 workspace 设置可能有 bug，改用显式 features
- `WindowContext` 类型在 0.2.2 中不存在，使用 `Window` + `Context<'_, V>` + `App`

### Decisions Made
- 组件与主题解耦: `.build(&theme)` 显式传入
- Demo 返回 `AnyElement` 用于注册表类型统一
- 库 crate 不启用 GPUI 平台 features

## Session 2 — 2026-05-03

### Actions
- 调查 AuraButton hover/pressed 交互不生效问题。
- 移除 `AuraButton` 的双路径实现中过度复杂的 `Render` + 手写 `is_pressed` 状态路径，保留 `.build(&theme)` 的 theme-explicit builder 模式。
- 为 Button 添加可选 `.id(...)` builder；默认使用调用位置 + label 生成稳定 GPUI element id，供 `.hover()` / `.active()` 状态追踪使用。
- 修正按钮主题色：filled variant 的 hover 比 base 稍暗，active 比 hover 更深；Default 透明按钮现在有可见 hover/active overlay。
- 添加 aura-theme 单元测试锁定 hover/active 背景层级。

### Key Discoveries
- Gallery Demo 实际调用 `AuraButton::build(theme)`，之前 `impl Render for AuraButton` 的手写 pressed 状态是死路径。
- GPUI 0.2.2 的 `.active()` 是 `StatefulInteractiveElement` 样式能力，需要 `.id(...)` 后使用；不需要组件自己维护 `is_pressed`。
- 之前每次 build/render 使用全局递增 id 会让交互状态难以稳定追踪；稳定 id 更符合 GPUI 的 element state 模型。
- Primary hover 原本使用 theme `family.hover`，在当前 NaiveUI token 下比 base 更亮，不满足“hover 暗一点”的需求；Default 原本 hover/active 背景全透明。

### Verification
- `cargo test -p aura-theme` passed: 2 tests.
- `cargo check` passed; existing aura-gallery dead_code warnings remain.
- `timeout 5s cargo run -p aura-gallery` compiled successfully, then failed to open a window in tmux due to GPUI Linux `NoCompositor` (environment/display issue, not compile issue).

### Follow-up — Button id policy
- 明确 Button 不能依赖业务开发者手写 `.id(...)` 才获得基础 hover/active 交互。
- `AuraButton::new(...)` 现在通过 `#[track_caller]` 捕获组件创建位置，默认 id 由创建位置 + label + variant/size/状态参数生成。
- `.id(...)` 保留为高级覆盖项，用于同一调用点批量渲染同 label/variant 按钮等潜在冲突场景。
- Added aura-components tests for automatic id generation and explicit id override.

### Follow-up — Global theme API
- Replaced public `AuraButton::build(&theme)` usage with GPUI `IntoElement + RenderOnce`; Button now reads `Config.theme` from `App` during render.
- Gallery demo registry no longer passes theme through function pointers; button demo wraps content in a `RenderOnce` demo component and reads global theme internally.
- Updated prompt.md, P1 prompt, and decisions to supersede explicit `.build(&theme)` policy.

### Follow-up — Button icon hover color
- Re-read the icon library after adding `aura-icons-lucide` and Button icon support.
- Root cause: `AuraIcon` intentionally inherits parent `text_color` when no explicit color is set, but `AuraButton` passed `.color(c.text)` to every internal icon, fixing the SVG text color at the normal state.
- Fix: Button-created icons no longer set explicit icon color; they inherit the button container text color, including hover `text_color(c.text_hover)` and disabled text color.
- Verified `cargo check`, `cargo test -p aura-theme`, and `cargo test -p aura-components`.

### Correction — SVG color inheritance in GPUI
- Previous assumption was wrong: GPUI `Svg::paint` requires the SVG element's own final `style.text.color`; it does not render from an ancestor div's text color automatically.
- Removing explicit icon color caused Button icons to disappear because `style.text.color` on `svg()` was `None`.
- Correct fix: keep normal icon `.color(c.text)` and add `AuraIcon::group_hover_color(group, c.text_hover)`; Button assigns a hover group to the button container so child icons switch color via GPUI `group_hover`.
- Verified `cargo check`, `cargo test -p aura-icons`, and `cargo test -p aura-components`.

## Session 6 — 2026-05-05

### Actions
- **初始化项目上下文**: 加载 `prompt.md`, `.memory/state.md`, `.prompt/P3-popper-feedback.md`。
- **验证编译基线**: 运行 `cargo check` 确认无 Error。
- **完成 Popper 基础架构 (Popper Foundation)**:
  - 扩展 `Placement` 枚举，支持全部 12 种方位（Top/Bottom/Left/Right × Start/End/Center）。
  - 实现 `Placement::flip()` 翻转逻辑。
  - 实现 `Popper::calculate_position_with_flip`，支持视口溢出检测与自动翻转/贴边 (Clamp)。
  - 实现 `ZIndexStack` 全局状态，定义 popup/modal/notification/tooltip 标准层级。
  - 在 `init_aura` 中完成 `ZIndexStack` 初始化。
- **完成 P3 全部组件开发 (13/13)**:
  - **反馈类**: Tooltip, Popover, Popconfirm, Dialog, Drawer, Message, Notification, Alert, Loading, MessageBox。
  - **容器/导航**: Card, Collapse, Dropdown。
  - **全局管理器**: 实现 `MessageManager` 和 `NotificationManager` 单例，支持异步自动销毁。
  - **Portal 深度集成**: 将所有弹出层组件对接至 `Popper` 引擎与 `Portal` 传送门系统。
- **Bug 修复与增强**:
  - 🐛 **Rate**: 修复鼠标移出后的 1s 延迟，改用 `on_hover` 实时监听状态。
  - 🐛 **Input**: 修复多行模式下的垂直居中偏离问题（改为 `items_start`）；新增 `.text_align(TextAlign)` API 支持文本对齐。
- **Gallery 完善**:
  - 为 P3 所有组件添加了对应的 `Demo` 页面并注册到全局路由。
  - 优化了 `main.rs` 的渲染循环，确保全局消息和通知层能在最顶层实时呈现。
- **状态同步**:
  - 更新 `.memory/state.md` 标记 P3 已 100% 完成，进入 P4 准备阶段。
  - 更新 `.memory/inventory.md` 标记所有 P3 组件为 ✅ Done。

### Key Discoveries
- GPUI 0.2.x 的 `push_portal` 必须支持 `FnOnce` 以便捕获非 Clone 的 `AnyElement`。
- 全局单例管理器（如 Message）通过 `Entity` 包装并存入 `Global` 是处理跨视图通信的最佳实践。

### Decisions Made
- 弹出层组件全部采用 `Arc<dyn Fn...>` 封装内容闭包，确保在 Portal 渲染循环中可安全多次调用（或在 `FnOnce` 环境下重建）。
- Dialog 和 Drawer 采用“消费点击”策略：点击内容区域不触发背景 Overlay 的关闭逻辑。
- **状态同步**:
  - 更新 `.memory/state.md` 标记 P2 已完成，P3 进行中。
  - 更新 `.memory/inventory.md` 标记 Popper 基建 ✅ Done。
  - 更新 `.memory/decisions.md` (ADR-010) 记录弹出层设计决策。

### Key Discoveries
- `PortalLayer` 已在 `aura-gallery` 的 `main.rs` 中实现，通过 `std::mem::take` 消费全局 `Portal` 栈，支持每帧重新推送实现即时渲染。
- GPUI 0.2.2 的 Z-Index 逻辑可以通过 `Config` 全局配置统一管理。

### Decisions Made
- 采用 "Flip then Clamp" 策略处理 Popper 视口溢出：优先尝试翻转（如 Top -> Bottom），若翻转后依然溢出，则强制贴边渲染。
- 沿用当前 Gallery 中的 `Portal` 全局单例模式，作为 P3 所有弹出组件的通信隧道。
