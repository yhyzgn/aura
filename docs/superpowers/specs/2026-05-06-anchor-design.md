# Design Spec - Anchor 锚点

**日期**: 2026-05-06
**阶段**: P4 Navigation
**状态**: 待评审

## 1. 概述

Anchor 锚点组件用于在长页面中提供快速跳转，并能根据当前滚动位置自动高亮对应的链接。

## 2. 核心功能

- **基础链接**: 支持多级嵌套的锚点链接。
- **自动高亮**: 随页面滚动自动激活当前可视区域内的锚点。
- **点击跳转**: 点击链接后平滑（或即时）滚动到目标位置。
- **固钉模式**: 默认支持固钉模式，固定在侧边。

## 3. 架构设计

### 3.1 组件结构

- `Anchor`: 主视图 (View)，管理 `active_link` 状态。
- `AnchorLink`: 链接数据模型，包含 `title`, `href` (目标 ID), `children`。
- `AnchorTarget`: 包装器组件，用于标记目标区域并上报其窗口坐标。

### 3.2 实现原理

- **位置监听**: `Anchor` 通过监听 `ScrollHandle` 或在 `paint` 阶段汇总所有 `AnchorTarget` 的 bounds。
- **激活检测**: 遍历所有 Target，找到 `bounds.top()` 最接近（且通常小于）某个阈值的项，设为 `active_link`。
- **跳转实现**: 通过 `ScrollHandle.scroll_to_item` 或手动计算坐标并调用 `set_offset`。

### 3.3 关键状态

```rust
pub struct Anchor {
    scroll_handle: ScrollHandle,
    active_link: Option<SharedString>,
    links: Vec<AnchorLink>,
    offset: Pixels,
}

pub struct AnchorLink {
    title: SharedString,
    href: SharedString,
    children: Vec<AnchorLink>,
}
```

## 4. API 设计

```rust
let scroll_handle = ScrollHandle::new();
cx.new(|_| {
    Anchor::new(scroll_handle)
        .link(AnchorLink::new("基本用法", "#basic"))
        .link(AnchorLink::new("API", "#api").child(AnchorLink::new("Attributes", "#attributes")))
})
```

## 5. UI 规范 (Tokens)

- **边框**: 左侧通常有一条纵向轴线。
- **高亮**: 激活项左侧有明显的指示条。
- **间距**: 节点垂直间距约 8-12px。

## 6. 待办事项 (Todos)

- [ ] 实现 `Anchor` View 结构。
- [ ] 实现 `AnchorTarget` 包装组件（基于 `BoundsTracker`）。
- [ ] 实现激活项检测算法。
- [ ] 实现点击平滑滚动（或即时）。
- [ ] 注册到 Gallery Demo。
