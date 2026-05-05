# Design Spec - Breadcrumb 面包屑

**日期**: 2026-05-06
**阶段**: P4 Navigation
**状态**: 待评审

## 1. 概述

Breadcrumb 组件显示当前页面的路径，快速返回之前的页面。支持自定义分隔符和图标。

## 2. 核心功能

- **层级展示**: 依次渲染路径节点。
- **自定义分隔符**: 支持字符串或 Icon 作为分隔符。
- **图标支持**: 节点可包含图标。
- **点击交互**: 支持点击回调，通常用于路由跳转。

## 3. 架构设计

### 3.1 组件结构

- `Breadcrumb`: 容器组件，管理全局配置 (separator)。
- `BreadcrumbItem`: 节点组件，包含内容和跳转逻辑。

### 3.2 渲染策略

- 使用 `flex-row` 布局。
- 自动在每两个 `BreadcrumbItem` 之间插入 `separator`。
- 最后一个节点默认不渲染分隔符，且样式通常为不可点击（或加粗）。

### 3.3 关键状态

```rust
pub struct Breadcrumb {
    separator: BreadcrumbSeparator,
    items: Vec<BreadcrumbItem>,
}

pub enum BreadcrumbSeparator {
    String(SharedString),
    Icon(IconName),
}

pub struct BreadcrumbItem {
    label: SharedString,
    icon: Option<IconName>,
    on_click: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}
```

## 4. API 设计

```rust
Breadcrumb::new()
    .separator("/")
    .item(BreadcrumbItem::new("首页").icon(IconName::House).on_click(|_, _| println!("Home")))
    .item(BreadcrumbItem::new("推广管理"))
    .item(BreadcrumbItem::new("推广列表"))
    .item(BreadcrumbItem::new("推广详情"))
```

## 5. UI 规范 (Tokens)

- **颜色**:
    - 普通文字: `theme.neutral.text_2`
    - 悬浮文字: `theme.primary.base`
    - 最后一项文字: `theme.neutral.text_1` (加粗)
    - 分隔符颜色: `theme.neutral.text_3`
- **间距**: 节点间距通常由分隔符两侧的 margin 控制，约 8-12px。

## 6. 待办事项 (Todos)

- [ ] 实现 `Breadcrumb` 基础结构。
- [ ] 实现 `BreadcrumbItem` 渲染。
- [ ] 实现分隔符逻辑。
- [ ] 注册到 Gallery Demo。
