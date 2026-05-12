# Aura Docs

Aura Docs 是 Aura UI 的官方原生文档主程序。它不是网页文档站，也不是 WebView，而是一个运行在 GPUI 原生窗口里的 Rust 应用。

## 目标

- 在原生窗口内展示 Aura UI 的设计理念、组件 API 和使用示例。
- 使用 `pulldown-cmark` 只解析 Markdown AST/Event。
- 把所有内容渲染为 Aura/GPUI 原生元素树。
- 通过 Live Demo 把真实组件直接插入文档流。

> 绝对边界：不引入 HTML、CSS、DOM、WebAssembly、WebView 或跨端转译路径。

## 当前文档能力

- 标题、段落、列表、引用块、分割线。
- 粗体、斜体、删除线、行内代码。
- 代码块语言识别、语法高亮、主题切换和复制。
- Button / Message 等组件效果与对应代码。
- 全局提示 Message / toast 宏。
- `::AuraDemo{component="Button"}::` 活体组件注入。
