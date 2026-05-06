# Design Spec - Pagination 分页

**日期**: 2026-05-06
**阶段**: P4 Data
**状态**: 待评审

## 1. 概述

Pagination 分页组件用于在大量数据呈现时，进行分页控制。

## 2. 核心功能

- **基础分页**: 支持上一页、页码列表、下一页。
- **总数展示**: 显示数据总条数。
- **快速跳转**: 支持直接输入页码跳转。
- **自定义布局**: 允许按需组合 `total`, `prev`, `pager`, `next`, `jumper` 等元素。
- **样式模式**: 支持带背景色的按钮模式（默认或 `background` 属性）。
- **折叠省略**: 当页数较多时，自动省略中间页码。

## 3. 架构设计

### 3.1 组件结构

- `Pagination`: 核心组件，维护 `current_page`, `page_size`, `total` 状态。
- 内部拆分为几个子组件（函数或直接渲染）：`PrevButton`, `NextButton`, `Pager`, `Total`, `Jumper`。

### 3.2 渲染策略

- 根据 `layout` 字符串（如 `"total, prev, pager, next, jumper"`）动态决定渲染顺序。
- `Pager` 逻辑：当页数 > 7 时，计算显示的页码序列，包含前后省略号（用 `...` 或 Icon 表示）。

### 3.3 关键状态

```rust
pub struct Pagination {
    total: usize,
    page_size: usize,
    current_page: usize,
    background: bool,
    layout: SharedString,
    on_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}
```

## 4. API 设计

```rust
Pagination::new(100)
    .page_size(10)
    .current_page(1)
    .background(true)
    .layout("total, prev, pager, next, jumper")
    .on_change(|page, _, _| println!("Jump to page: {}", page))
```

## 5. UI 规范 (Tokens)

- **尺寸**: 按钮高度约 32px，宽度视内容而定，最小 32px。
- **圆角**: 默认 4px (`theme.radius.sm`)。
- **颜色**:
    - 激活页码: `theme.primary.base` (背景色/文字色取决于是否带背景)。
    - 普通页码 Hover: `theme.primary.base` (文字色)。
    - 禁用状态: `theme.neutral.text_4` 和背景色变浅。

## 6. 待办事项 (Todos)

- [ ] 实现 `Pagination` 基础数据模型与构建器。
- [ ] 实现 `Pager` 页码计算算法 (含省略号逻辑)。
- [ ] 实现各布局模块 (Total, Prev, Next, Jumper) 渲染。
- [ ] 实现背景模式 (`background`) 样式。
- [ ] 注册到 Gallery Demo。
