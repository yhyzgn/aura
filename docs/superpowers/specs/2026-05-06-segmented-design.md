# Design Spec - Segmented 分段控制器

**日期**: 2026-05-06
**阶段**: P4 Data
**状态**: 待评审

## 1. 概述

Segmented 分段控制器用于展示多个选项并允许用户选择其中单个选项。通常用于切换不同的视图或数据范围。

## 2. 核心功能

- **选项切换**: 可以在一组选项中进行单选。
- **块状样式**: 支持 `block` 属性，使组件宽度撑满父容器。
- **数据源**: 接受一组 `SegmentedOption` 作为数据源。
- **自定义内容**: 支持自定义渲染选项内容。

## 3. 架构设计

### 3.1 组件结构

- `Segmented`: 主视图 (View)，维护当前选中的值 (`value`)。
- `SegmentedOption`: 数据模型，包含 `label`, `value`, `disabled` 等属性。

### 3.2 渲染策略

- 外层容器使用较深的背景色 (`theme.neutral.hover`) 和圆角。
- 内部选项平铺，使用相对定位或 flex 布局。
- 选中的选项有一个带阴影的白色（或卡片色）背景滑块，由于 GPUI 动画限制，首期可先做静态激活样式切换。

### 3.3 关键状态

```rust
pub struct Segmented {
    options: Vec<SegmentedOption>,
    value: SharedString,
    block: bool,
    on_change: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
}

pub struct SegmentedOption {
    pub label: SharedString,
    pub value: SharedString,
    pub disabled: bool,
}
```

## 4. API 设计

```rust
cx.new(|_| {
    Segmented::new(vec![
        SegmentedOption::new("Daily", "daily"),
        SegmentedOption::new("Weekly", "weekly"),
        SegmentedOption::new("Monthly", "monthly"),
    ])
    .value("daily")
    .on_change(|val, _, _| println!("Selected: {}", val))
})
```

## 5. UI 规范 (Tokens)

- **背景**: 容器 `theme.neutral.hover`，选中块 `theme.neutral.card` 带有小阴影。
- **高度**: 默认 32px。
- **字体**: 14px，选中加粗，未选中置灰。

## 6. 待办事项 (Todos)

- [ ] 实现 `Segmented` View 结构。
- [ ] 实现选项渲染逻辑。
- [ ] 注册到 Gallery Demo。
