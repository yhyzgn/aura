# Native Architecture

Aura Docs 的核心原则是“文档也是原生应用”。Markdown 只是输入格式，最终输出必须是 Aura/GPUI 节点。

## Workspace 边界

- `crates/aura-components`：所有可复用 UI 组件。
- `apps/aura-gallery`：组件看板，用于展示组件交互效果。
- `apps/aura-docs`：官方文档主程序，负责 Markdown 文档渲染、组件效果说明和活体组件注入。

## 文档渲染流水线

1. `pulldown-cmark` 读取 Markdown 文本并产生事件。
2. Renderer 使用 `Vec` 栈管理块级结构。
3. Inline 样式通过上下文状态记录。
4. 文本片段交给 `Paragraph` / `Text` 渲染为 `StyledText`。
5. 代码块交给 `CodeBlock` 组件。
6. Live Demo 标记转换为真实 Aura 组件。

```rust src="architecture/render_pipeline.rs"
```

## 为什么不使用 Web 文档站

- Aura 的目标运行时是 GPUI 原生窗口。
- 文档系统必须反向验证组件库自己的排版、滚动、文本和交互能力。
- Live Demo 必须是真实组件，而不是截图、iframe 或转译产物。
