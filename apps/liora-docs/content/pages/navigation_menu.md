# NavigationMenu

导航菜单用于表达页面结构与当前业务位置。Liora NavigationMenu 支持水平、垂直和折叠模式，并可配置 ESC 关闭子菜单浮层。

垂直 `NavigationMenu` 默认不创建内部滚动区。放进 `Sidebar` 时由
`Sidebar::scrollable()` 统一管理滚动；如果菜单需要在独立的有界容器内滚动，
再显式调用 `NavigationMenu::scrollable()`。避免同时启用两层垂直滚动。

## 水平模式

适合窗口顶部导航。`on_select` 会返回当前激活的菜单项 id，业务可据此切换内容区域。

### 效果

::Demo{component="NavigationMenuHorizontal"}::

### 代码

```rust src="navigation_menu/horizontal.rs"
```

## 垂直模式

适合左侧导航。子菜单和分组可表达更深的业务层级。

### 效果

::Demo{component="NavigationMenuVertical"}::

### 代码

```rust src="navigation_menu/vertical.rs"
```

## 折叠模式

适合紧凑侧栏，只保留图标宽度，子菜单仍通过原生浮层展开。

### 效果

::Demo{component="NavigationMenuCollapsed"}::

### 代码

```rust src="navigation_menu/collapsed.rs"
```
