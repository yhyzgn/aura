# Design Spec - Result 结果页

**日期**: 2026-05-06
**阶段**: P4 Data
**状态**: 待评审

## 1. 概述

Result 结果页组件用于在任务完成后通知用户结果。支持多种内置状态（成功、失败、警告、信息）及对应的图标。

## 2. 核心功能

- **内置状态**:
    - `Success`: 操作成功。
    - `Warning`: 警告提示。
    - `Error`: 操作失败。
    - `Info`: 信息提示。
- **自定义内容**: 支持自定义主标题、副标题。
- **操作区 (Extra)**: 底部支持放置按钮等操作。
- **图标定制**: 允许覆盖默认状态图标。

## 3. 架构设计

### 3.1 组件结构

- `Result`: `RenderOnce` 组件。
- 垂直居中布局 (`flex-col`)。

### 3.2 渲染策略

- **Icon**: 顶部展示状态对应的大尺寸图标。
- **Title**: 加粗的主标题。
- **SubTitle**: 辅助描述。
- **Extra**: 操作区。

### 3.3 关键状态

```rust
pub struct Result {
    status: ResultStatus,
    title: SharedString,
    sub_title: Option<SharedString>,
    icon: Option<AnyElement>,
    extra: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
}

pub enum ResultStatus {
    Success,
    Warning,
    Error,
    Info,
}
```

## 4. API 设计

```rust
Result::new("成功购买云服务器")
    .status(ResultStatus::Success)
    .sub_title("订单编号：2017182818828182881")
    .extra(|_, _| Button::new("返回列表").into_any_element())
```

## 5. UI 规范 (Tokens)

- **图标尺寸**: 默认 72x72px。
- **颜色**:
    - `Success`: `theme.success.base`
    - `Warning`: `theme.warning.base`
    - `Error`: `theme.danger.base`
    - `Info`: `theme.primary.base`
- **间距**: 整体 padding 约 48px。

## 6. 待办事项 (Todos)

- [ ] 实现 `Result` 基础结构。
- [ ] 实现状态图标映射。
- [ ] 实现标题及操作区渲染。
- [ ] 注册到 Gallery Demo。
