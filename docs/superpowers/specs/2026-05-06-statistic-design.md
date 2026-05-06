# Design Spec - Statistic 统计数值

**日期**: 2026-05-06
**阶段**: P4 Data
**状态**: 待评审

## 1. 概述

Statistic 统计数值组件用于展示强调的数值数据。

## 2. 核心功能

- **数值展示**: 显示格式化后的数值。
- **前缀/后缀**: 支持添加前缀和后缀内容（文本或 Icon）。
- **标题**: 提供数据维度的说明。
- **自定义样式**: 支持自定义数值颜色。

## 3. 架构设计

### 3.1 组件结构

- `Statistic`: `RenderOnce` 组件。

### 3.2 渲染策略

- 使用 `flex-col` 布局：上方展示 `title`，下方 `flex-row` 展示 `prefix` + `value` + `suffix`。

### 3.3 关键状态

```rust
pub struct Statistic {
    title: SharedString,
    value: SharedString,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    value_color: Option<Hsla>,
}
```

## 4. API 设计

```rust
Statistic::new("活跃用户数", "1,234,567")
    .prefix(Icon::new(IconName::User).into_any_element())
    .suffix(div().child("人").into_any_element())
    .value_color(theme.success.base)
```

## 5. UI 规范 (Tokens)

- **字体**: 数值部分使用大号字体（如 `text_2xl`）并加粗，标题和前后缀使用常规大小和辅助色 (`theme.neutral.text_3`)。

## 6. 待办事项 (Todos)

- [ ] 实现 `Statistic` 基础结构。
- [ ] 实现渲染逻辑。
- [ ] 注册到 Gallery Demo。
