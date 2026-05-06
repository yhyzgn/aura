# Design Spec - Skeleton 骨架屏

**日期**: 2026-05-06
**阶段**: P4 Data
**状态**: 待评审

## 1. 概述

Skeleton 骨架屏组件在页面数据加载时展示占位内容，提升用户体验。

## 2. 核心功能

- **占位渲染**: 模拟真实内容的轮廓。
- **多种形状**:
    - `Circle`: 圆形（通常用于头像）。
    - `Square`: 方形（通常用于按钮、标题）。
    - `Paragraph`: 段落（多行文本模拟）。
- **动画效果**: 支持 `Pulse` (呼吸灯) 效果。
- **动态控制**: 根据 `loading` 状态自动切换骨架屏与真实内容。

## 3. 架构设计

### 3.1 组件结构

- `Skeleton`: 容器组件，管理 `loading` 状态。
- `SkeletonItem`: 基础原子项，负责具体的占位块渲染。

### 3.2 渲染策略

- **原子块**: 使用 `div` 并配合 `bg(theme.neutral.hover)` (或更浅的灰色) 渲染。
- **动画**: GPUI 的动画支持目前较弱，首期先实现静态占位，通过定时器驱动透明度变化模拟呼吸灯效果（可选）。
- **插槽**: 通过 `template` 闭包自定义骨架结构，`children` 传递真实内容。

### 3.3 关键状态

```rust
pub struct Skeleton {
    loading: bool,
    rows: u32,
    animated: bool,
    template: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
}

pub struct SkeletonItem {
    variant: SkeletonVariant,
}

pub enum SkeletonVariant {
    Circle,
    Square,
    Paragraph,
    Image,
}
```

## 4. API 设计

```rust
Skeleton::new()
    .loading(true)
    .rows(4)
    .template(|_, _| {
        div().flex().gap_4()
            .child(SkeletonItem::new(SkeletonVariant::Circle))
            .child(SkeletonItem::new(SkeletonVariant::Paragraph))
            .into_any_element()
    })
    .child(div().child("真实内容"))
```

## 5. UI 规范 (Tokens)

- **颜色**: `theme.neutral.hover` 或特定的 `skeleton.bg`。
- **圆角**:
    - `Circle`: 50%
    - `Paragraph/Square`: 4px
- **行高**: 段落行高通常为 16px，间距 12px。

## 6. 待办事项 (Todos)

- [ ] 实现 `SkeletonItem` 原子组件。
- [ ] 实现 `Skeleton` 容器与逻辑控制。
- [ ] 实现多行段落自动生成逻辑。
- [ ] 注册到 Gallery Demo。
