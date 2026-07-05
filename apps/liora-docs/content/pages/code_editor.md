# CodeEditor

`CodeEditor` 是 Liora 的原生代码编辑控件 v2 基座，面向配置面板、低代码脚本、小型规则编辑器和企业应用中的内嵌代码编辑场景。它保持纯 Rust + GPUI native：编辑状态由 `CodeBuffer` / selection / viewport 分层管理，可见行通过 GPUI 官方 `list` 虚拟渲染，高亮预览复用 `CodeBlock`，诊断信息通过数据结构或 provider 注入，不绑定 WebView、Monaco 或浏览器运行时。

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
- selection 与 viewport 独立于渲染层，后续可继续扩展多光标、事务 undo/redo、软换行、折叠和 inlay hints。
- 可见行通过 GPUI 官方 `list` 渲染，配合 Liora `VirtualScrollbar`，避免大文件时把所有行一次性塞进普通元素树。
- 设计参考 Zed 官方源码的分层思想，但不会复制或依赖 Zed GPL editor/language/multi_buffer/text/rope crate 代码。

## 能力边界

- 支持行号、语言、主题、缩进配置、编辑回调，`Tab` / `Shift+Tab` 可按当前缩进配置缩进或反缩进。
- 支持基础导航、选择、复制、粘贴、剪切、回车换行和可见行虚拟渲染。
- 支持搜索命中统计、静态/动态 completion candidates、hover/help provider 扩展点；这些 API 是语言服务接入点，不会把 SDK 绑定到某个 LSP 进程。
- 支持 `CodeDiagnostic` 静态注入，也支持 `diagnostics_provider` 根据最新文本动态生成诊断结果，用于展示语法检查、lint、业务规则检查等结果。
- 当前阶段语法高亮以预览区呈现；后续可继续把高亮 run 与编辑布局合并，形成完整编辑态高亮。
- 后续企业级增强应优先补 transaction undo/redo、display map、Tree-sitter 可选后端和 LSP bridge traits。
