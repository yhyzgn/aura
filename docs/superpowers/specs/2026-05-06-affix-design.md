# Design Spec - Affix 固钉

**日期**: 2026-05-06
**阶段**: P4 Navigation
**状态**: 待评审

## 1. 概述

Affix 组件将内容固定在特定可视区域。常用于侧边菜单、页头等。

## 2. 核心功能

- **固定位置**: 支持固定在顶部 (`top`) 或底部 (`bottom`)。
- **偏移量**: 设置距离边缘的偏移像素。
- **状态回调**: 固定状态改变时触发回调。
- **目标容器**: 支持相对于窗口或指定容器固定（首期仅实现相对于窗口）。

## 3. 架构设计

### 3.1 组件结构

- `Affix`: 核心 View 组件。
- 内部维护 `is_fixed` 状态。
- 使用一个占位元素 (`placeholder`) 留在文档流中，保持布局不坍塌。

### 3.2 实现原理

- 在 `paint` 阶段，通过 `window.bounds()` 获取当前组件在窗口中的实际位置。
- 如果位置达到 `offset` 阈值，则 `is_fixed = true`。
- 固定时，通过 `Portal` 或绝对定位将其渲染在固定坐标，同时让 `placeholder` 占据原位。

### 3.3 关键状态

```rust
pub struct Affix {
    offset: Pixels,
    position: AffixPosition,
    is_fixed: bool,
    on_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
    content: Option<Box<dyn Fn(&mut Window, &mut Context<Affix>) -> AnyElement + 'static>>,
}

pub enum AffixPosition {
    Top,
    Bottom,
}
```

## 4. API 设计

```rust
cx.new(|_| {
    Affix::new()
        .offset(px(10.0))
        .content(|_, _| div().child("固钉内容").into_any_element())
})
```

## 5. UI 规范 (Tokens)

- **Z-Index**: 固定状态下通常需要较高的 `z_index` (如 1000)。
- **背景**: 固定状态下通常需要背景色，以防透视。

## 6. 待办事项 (Todos)

- [ ] 实现 `Affix` View 结构。
- [ ] 实现坐标检测逻辑。
- [ ] 实现固定状态切换与布局占位。
- [ ] 注册到 Gallery Demo。
