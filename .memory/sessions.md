# Session History

## Session 4 — 2026-05-04

### Actions
- 完成 P1 Basic Elements 所有 13 个组件的开发与验证
- **Button 增强**: 
  - 支持 `icon_start`/`icon_end` 接受任意元素 (`AnyElement`) 或直接接受 `IconName`
  - 为 `IconName` 实现 `IntoElement` trait
  - 添加 `Text` 变体 (Ghost 按钮)
  - 实现 `ButtonGroup` 容器
  - 更新 Button Demo 包含全部新特性，验证 `.icon_start(IconName::Check)` 语法
- **Typography**:
  - 创建 `typography_demo` 整合 Text, Title, Paragraph, Link
  - 修复 `Link` 对新 `ButtonVariant::Text` 的处理
- **Layout & Container**:
  - 重构 `Space` 为容器模式，支持 `gap` 和 `vertical`
  - 修正 `Col` 栅格宽度计算 (使用 `gpui::relative`)
  - 更新 `layout_demo` 展示 24 栅格系统
  - 更新 `container_demo` 包含 `Container` (Header/Aside/Main/Footer) 示例
- **验证**:
  - `cargo check` 0 errors (已清理大部分新增警告)
  - 13/13 组件在 Gallery 中均有 Demo 且符合 codex 范式
- **状态同步**:
  - 更新 `.memory/inventory.md` 和 `.memory/state.md`
  - 标记 P1 阶段完成，准备进入 P2

### Key Discoveries
- GPUI 0.2.2 `AnyElement` 不实现 `Clone`，在 `RenderOnce` 中需注意所有权转移
- `ButtonGroup` 这种复合组件需要子组件提供更细粒度的样式覆盖 (如 `rounded_none`) 才能实现完美连接，当前版本采用简化 Flex 布局实现
- 栅格系统百分比宽度在 GPUI 中通过 `relative(span/24.0)` 实现最为准确

### Decisions Made
- 统一使用 `AnyElement` 作为组件 Icon/Child 的通用类型
- P1 剩余组件 (Row, Col, Divider, Space) 均已按照 Element-Plus 规范进行功能补齐

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
