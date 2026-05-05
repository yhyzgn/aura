# Design Spec - Menu 导航菜单

**日期**: 2026-05-06
**阶段**: P4 Navigation
**状态**: 待评审

## 1. 概述

Menu 组件为网站提供导航轮廓。支持水平/垂直模式、子菜单嵌套、折叠状态、以及自定义主题色。

## 2. 核心功能

- **模式 (Mode)**: `Horizontal` (水平) / `Vertical` (垂直)。
- **折叠 (Collapse)**: 仅垂直模式支持，折叠后子菜单通过 Popover 弹出。
- **子菜单 (Submenu)**: 支持无限嵌套（建议 2-3 层）。
- **分组 (Group)**: 菜单项分组展示。
- **状态管理**: `active_index` 追踪当前选中项。
- **交互**: Hover 状态、点击选中、子菜单展开/收起。

## 3. 架构设计

### 3.1 组件结构

- `Menu`: 主视图 (View)，管理全局状态 (mode, collapse, active_index, open_submenus)。
- `MenuItem`: 基础菜单项。
- `SubMenu`: 包含子节点的菜单项，处理展开逻辑。
- `MenuItemGroup`: 逻辑分组，不参与选中逻辑。

### 3.2 渲染策略

- **水平模式**: 使用 `flex-row`。SubMenu 使用 `Popper` (基于 P3 实现的定位引擎)。
- **垂直模式**: 使用 `flex-col`。
    - 展开时：SubMenu 下方插入子节点 (Accordion 风格)。
    - 折叠时：图标展示，SubMenu 通过 `Popper` 弹出。

### 3.3 关键状态

```rust
pub struct Menu {
    mode: MenuMode,
    is_collapsed: bool,
    active_index: Option<SharedString>,
    opened_submenus: HashSet<SharedString>,
    items: Vec<MenuNode>,
    on_select: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
}
```

## 4. API 设计

```rust
// 基础用法
Menu::new()
    .mode(MenuMode::Horizontal)
    .default_active("1")
    .on_select(|index, window, cx| { ... })
    .item("1", "处理中心", Some(IconName::List))
    .submenu("2", "我的工作台", Some(IconName::Briefcase), |s| s
        .item("2-1", "选项1", None)
        .item("2-2", "选项2", None)
        .group("分组一", |g| g
            .item("2-3", "选项3", None)
        )
    )
```

## 5. UI 规范 (Tokens)

- **高度**: 水平模式 60px，垂直模式菜单项 56px。
- **颜色**:
    - 文字: `theme.neutral.text`
    - 激活文字: `theme.primary.base`
    - 激活背景: `theme.primary.opacity(0.1)` (或下划线，取决于模式)
    - Hover 背景: `theme.neutral.hover`
- **动画**: 垂直展开时的收缩动画 (GPUI 动画支持有限，首期可先实现即时切换)。

## 6. 待办事项 (Todos)

- [ ] 实现 `Menu` View。
- [ ] 实现 `MenuItem` 渲染逻辑。
- [ ] 实现 `SubMenu` 展开逻辑 (Vertical)。
- [ ] 实现 `SubMenu` Popover 逻辑 (Horizontal / Collapsed Vertical)。
- [ ] 注册到 Gallery Demo。
