# Design Spec - Progress 进度条

**日期**: 2026-05-06
**阶段**: P4 Data
**状态**: 待评审

## 1. 概述

Progress 进度条组件用于展示操作进度，告知用户当前状态。支持线形、环形等多种展示方式。

## 2. 核心功能

- **百分比展示**: 接收 0-100 的百分比数值。
- **多种类型**:
    - `Line`: 线形进度条。
    - `Circle`: 环形进度条（首期线形为主，环形可选实现）。
- **状态反馈**: 支持 `Success`, `Warning`, `Exception` (Error) 状态，颜色自动切换。
- **自定义定制**: 支持自定义高度 (`stroke_width`)、颜色、以及文本显示控制。

## 3. 架构设计

### 3.1 组件结构

- `Progress`: `RenderOnce` 组件。
- 线形进度条：外层轨道 (`track`) + 内层激活条 (`bar`)。
- 文本显示：通常位于激活条右侧或内部。

### 3.2 渲染策略

- **Line**: 两个嵌套的 `div`。内层 `div` 的宽度通过 `percentage` 计算 (百分比宽度)。
- **Circle**: 使用 GPUI 的 `Path` 或 `SVG` 渲染（首期优先实现 Line 模式）。

### 3.3 关键状态

```rust
pub struct Progress {
    percentage: f32,
    type_: ProgressType,
    stroke_width: Pixels,
    status: Option<ProgressStatus>,
    color: Option<Hsla>,
    show_text: bool,
}

pub enum ProgressType { Line, Circle }
pub enum ProgressStatus { Success, Warning, Exception }
```

## 4. API 设计

```rust
Progress::new(50.0)
    .status(ProgressStatus::Success)
    .stroke_width(px(12.0))
    .show_text(true)
```

## 5. UI 规范 (Tokens)

- **高度**: 默认 6px。
- **颜色**:
    - 轨道背景: `theme.neutral.hover`
    - Success: `theme.success.base`
    - Exception: `theme.danger.base`
    - Warning: `theme.warning.base`
    - 文本: `theme.neutral.text_2`

## 6. 待办事项 (Todos)

- [ ] 实现 `Progress` 基础结构。
- [ ] 实现 Line 模式渲染逻辑。
- [ ] 实现状态颜色映射。
- [ ] 注册到 Gallery Demo。
