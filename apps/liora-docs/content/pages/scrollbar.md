# Scrollbar

`Scrollbar` 是 Liora 自举滚动容器，用于在原生 GPUI 视图中展示可滚动内容和可拖拽滚动条。

- 适合固定高度面板、弹窗内容、日志列表、设置列表和长文本阅读区。
- hover 到右侧滚动轨道时 thumb 会加宽，整条轨道都能命中，便于鼠标拖拽。
- 对虚拟列表/表格请优先使用对应组件内置的 `VirtualScrollbar`，它会直接读取 GPUI `ListState`。

## 基础长列表

短视口 + 多行文本，适合菜单、日志和设置项列表。

### 效果

::Demo{component="ScrollbarBasic"}::

### 代码

```rust src="scrollbar/basic.rs"
```

## 卡片流

滚动内容可以组合 `Card`、`Tag`、`Space` 等复杂组件，滚动条仍只负责视口和滚动位置。

### 效果

::Demo{component="ScrollbarCards"}::

### 代码

```rust src="scrollbar/cards.rs"
```

## 长段落阅读

正文内容可以自动换行、鼠标选择，滚动条高度和位置由真实内容高度计算。

### 效果

::Demo{component="ScrollbarArticle"}::

### 代码

```rust src="scrollbar/article.rs"
```

## 紧凑高度

小高度视口适合弹窗、下拉面板和工具窗口。hover 轨道后 thumb 会加宽，拖拽命中区比可见 thumb 更宽。

### 效果

::Demo{component="ScrollbarCompact"}::

### 代码

```rust src="scrollbar/compact.rs"
```
