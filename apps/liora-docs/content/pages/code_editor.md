# CodeEditor

`CodeEditor` 是 Liora 的原生代码编辑控件 v2 基座，面向配置面板、低代码脚本、小型规则编辑器和企业应用中的内嵌代码编辑场景。它保持纯 Rust + GPUI native：编辑状态由 `CodeBuffer` / selection / viewport 分层管理，可见行通过 GPUI 官方 `list` 虚拟渲染，编辑区内实时高亮复用 `CodeBlock` 的高亮缓存，诊断信息通过数据结构或 provider 注入，不绑定 WebView、Monaco 或浏览器运行时。

## 基础编辑器

### 效果

::Demo{component="CodeEditorBasic"}::

### 代码

```rust src="code_editor/basic.rs"
```

## Diagnostics 扩展点

### 效果

::Demo{component="CodeEditorDiagnostics"}::

### 代码

```rust src="code_editor/diagnostics.rs"
```

## 高级扩展点：搜索、补全和 hover

### 效果

::Demo{component="CodeEditorAdvanced"}::

### 代码

```rust src="code_editor/advanced.rs"
```

## 架构说明

- v2 不再把 `Input` 当作编辑内核，而是由独立 `CodeBuffer` 管理文本、行索引、offset 与 point 映射。
- selection、viewport 与 `CodeDisplayMap` 独立于渲染层；`CodeDisplayMap` 统一维护行高、gutter、内容偏移和 pointer-to-offset 映射，后续可继续扩展软换行、折叠、inlay hints 与更精确 glyph hit-test。
- 可见行通过 GPUI 官方 `list` 渲染，配合 Liora `VirtualScrollbar`，避免大文件时把所有行一次性塞进普通元素树。每一行直接渲染 syntax-highlighted text，编辑区本身就是实时效果区，不再额外展示割裂的预览面板。
- 设计参考 Zed 官方源码的分层思想，但不会复制或依赖 Zed GPL editor/language/multi_buffer/text/rope crate 代码。

## 能力边界

- 支持行号、语言、主题、缩进配置、编辑回调，`Tab` / `Shift+Tab` 可按当前缩进配置缩进或反缩进。
- 支持基础导航、键盘选择、鼠标点击定位、列级光标、拖拽选择、复制、粘贴、剪切、回车换行、缩进/反缩进、`Ctrl/Cmd+Z` undo 与 `Ctrl/Cmd+Shift+Z` redo。
- 支持搜索命中统计、静态/动态 completion candidates、hover/help provider 扩展点；这些 API 是语言服务接入点，不会把 SDK 绑定到某个 LSP 进程。
- 支持 `CodeDiagnostic` 静态注入，也支持 `diagnostics_provider` 根据最新文本动态生成诊断结果，用于展示语法检查、lint、业务规则检查等结果。
- 语法高亮已经进入编辑行内部，输入变化会驱动同一区域重新渲染；光标闪烁使用 overlay 绘制，不参与文本 flex 布局，避免把后续文本挤开；点击定位会按当前行内容估算列位置并保持 UTF-8 边界安全。
- 编辑事务栈已经可恢复文本和选区快照，后续增强重点是事务分组合并、基于 GPUI shaped line 的精确 glyph hit-test、增量高亮和 LSP 语义 token。
- 后续企业级增强应优先补 piece table/rope 后端、事务分组合并、Tree-sitter 可选后端和 LSP bridge traits。
