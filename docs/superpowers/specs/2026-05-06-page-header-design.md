# Design Spec - PageHeader 页头

**日期**: 2026-05-06
**阶段**: P4 Navigation
**状态**: 待评审

## 1. 概述

PageHeader 页头组件位于页面顶部，用于标识页面内容并提供相关的操作（如返回、主操作等）。

## 2. 核心功能

- **基础展示**: 显示主标题、副标题。
- **返回功能**: 提供返回按钮及点击回调。
- **操作区 (Extra)**: 允许在右侧添加自定义操作按钮。
- **内容区 (Content)**: 支持在标题下方展示详细的页面信息。
- **页脚 (Footer)**: 底部扩展区。

## 3. 架构设计

### 3.1 组件结构

- `PageHeader`: `RenderOnce` 组件。
- 采用 `flex` 布局划分布局区域。

### 3.2 布局策略

- **Row 1**: [Back Icon] | [Title] | [Vertical Divider] | [Subtitle] | [Spacer] | [Extra]
- **Row 2**: [Content]
- **Row 3**: [Footer]

### 3.3 关键状态

```rust
pub struct PageHeader {
    title: SharedString,
    sub_title: Option<SharedString>,
    back_icon: Option<IconName>,
    on_back: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    extra: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    content: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    footer: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
}
```

## 4. API 设计

```rust
PageHeader::new("详情页面")
    .sub_title("子标题")
    .on_back(|_, _| println!("Back"))
    .extra(|_, _| Button::new("Edit").into_any_element())
    .content(|_, _| div().child("这是页面描述内容").into_any_element())
```

## 5. UI 规范 (Tokens)

- **字体**: 标题 18px (Bold), 副标题 14px (Secondary color)。
- **间距**: 整体 padding 约 16-24px。
- **分割线**: 标题与内容、页脚之间可能需要边框或边距。

## 6. 待办事项 (Todos)

- [ ] 实现 `PageHeader` 基础结构。
- [ ] 实现标题及返回按钮渲染。
- [ ] 实现 Extra, Content, Footer 插槽逻辑。
- [ ] 注册到 Gallery Demo。
