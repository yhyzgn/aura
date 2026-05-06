# Design Spec - Empty 空状态

**日期**: 2026-05-06
**阶段**: P4 Data
**状态**: 待评审

## 1. 概述

Empty 空状态组件用于展示页面无数据时的占位图及提示。支持自定义图片、描述和操作区。

## 2. 核心功能

- **缺省提示**: 展示默认的缺省图。
- **自定义描述**: 支持自定义文案提示。
- **操作按钮**: 提供底部插槽，通常用于“去添加”、“刷新”等操作。
- **图片选择**: 支持自定义图片 URL 或内置 SVG 图标。

## 3. 架构设计

### 3.1 组件结构

- `Empty`: `RenderOnce` 组件。
- 垂直布局 (`flex-col`)。

### 3.2 渲染策略

- **Image**: 顶部展示图片或内置 SVG。
- **Description**: 中间展示文案。
- **Extra**: 底部渲染自定义元素。

### 3.3 关键状态

```rust
pub struct Empty {
    image: Option<AnyElement>,
    description: SharedString,
    extra: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
}
```

## 4. API 设计

```rust
Empty::new()
    .description("暂无搜索结果")
    .extra(|_, _| Button::new("返回首页").into_any_element())
```

## 5. UI 规范 (Tokens)

- **图片尺寸**: 默认约 160x160px。
- **颜色**: 文案使用 `theme.neutral.text_3`。
- **内边距**: 整体 padding 约 40px。

## 6. 待办事项 (Todos)

- [ ] 实现 `Empty` 基础结构。
- [ ] 实现默认缺省图 (内置 SVG)。
- [ ] 实现描述与操作区渲染。
- [ ] 注册到 Gallery Demo。
