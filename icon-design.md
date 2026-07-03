# Liora UI - 图标系统架构与代码生成规范 (Icon System Specification)

> Lucide 图标库官网： https://lucide.dev/icons/
> Lucide 图标库仓库：https://github.com/lucide-icons/lucide.git

## 1. 架构背景与目标
**Liora UI** 是一套基于 Rust 和 GPUI 开发的原生高性能企业级 UI 组件库。为了兼顾开箱即用的开发体验与企业级真实业务的定制化需求，Liora 的图标系统采用 **“无边界基础容器 + 独立代码生成扩展包”** 的混合架构。

当前目标：
1. 实现一个纯粹的 SVG Path 渲染容器 `Icon`。
2. 开发一个基于 `build.rs` 的代码生成工作流，将 **Lucide** 图标库批量编译为 Rust 枚举，并封装为独立的 Crate。

## 2. 核心设计一：无边界图标容器 (Icon)
`Icon` 不应该与任何特定的图标库强绑定。它本质上是一个高度封装的 GPUI 视图容器，只负责处理以下职责：
* **尺寸 (Size):** 统一处理宽度和高度。
* **色彩 (Color):** 响应 Liora 全局主题，支持自定义颜色。
* **交互 (Interaction):** 处理 Hover、Active 等状态。
* **路径渲染 (Path Rendering):** 接收并渲染合法的 SVG `<path>` 字符串。

### 期望的 API 调用范式：
```rust
// 范式 1：直接传入从 Figma 或 Iconfont 导出的原生 SVG Path 字符串（满足定制业务需求）
Icon::new()
    .size(px(24.0))
    .color(theme.primary)
    .path("M12 2L2 22h20L12 2zm0 3.5l7.5 14.5h-15L12 5.5z")
```

## 3. 核心设计二：代码生成模式 (Codegen for Lucide)
为了提供丝滑的内置图标开发体验，我们需要将 **Lucide** 图标库剥离为一个独立的 Crate（例如命名为 `liora-icons-lucide`）。

该 Crate 的核心是通过 `build.rs` 在编译期自动完成以下工作：
1. **读取：** 遍历本地或作为 submodule 引入的 Lucide `.svg` 文件目录。
2. **解析：** 提取 SVG 文件中 `<path>` 标签的 `d` 属性（即路径字符串）。对于包含多个 `<path>` 的复杂图标，需进行字符串合并处理。
3. **生成：** 自动生成一个包含所有图标名称的巨大 Rust `enum`，并为其实现获取对应路径字符串的方法。

### 期望的 API 调用范式（结合 Codegen）：
```rust
// 范式 2：使用独立扩展包提供的强类型 Enum
use liora_icons_lucide::IconName;

// 推荐的丝滑调用方式
Icon::new(IconName::Home)
    .size(px(20.0))
    .color(theme.text_main)

// 或者采用 Builder 模式的变体
Icon::new().icon(IconName::ShoppingCart)
```

---

## 4. 具体的开发任务（Prompt for AI）

请基于以上架构规范，使用 Rust 为我实现以下代码：

### 任务一：实现 `build.rs` 解析与生成逻辑
请在 `liora-icons-lucide` Crate 中编写一个 `build.rs` 脚本。
* 脚本需要能够读取指定的 `assets/svgs` 目录。
* 使用正则表达式或轻量级 XML 解析库（如 `quick-xml` 或 `roxmltree`），提取每个 SVG 文件的 `<path d="...">` 内容。
* 生成一个 `generated.rs` 文件，内容包含一个 `pub enum IconName`，以及一个返回静态字符串的 `impl IconName { pub fn path(&self) -> &'static str }`。
* 请注意处理 Rust 标识符的命名规范（如将 `shopping-cart.svg` 转换为 `IconName::ShoppingCart`）。

### 任务二：实现 `Icon` 核心容器组件
请使用 GPUI 的 API 实现 `Icon` 组件。
* 它需要实现 `gpui::IntoElement` 或 `gpui::RenderOnce`。
* 内部使用 GPUI 的 `svg()` 原生组件来承载最终的路径。
* 请实现 Builder 模式的方法，如 `.size()`, `.color()`, `.path()` 等。

### 任务三：优化 API 的丝滑度（Trait 抽象）
为了让 `Icon::new()` 既能接受字符串也能接受 `IconName` 枚举，请巧妙利用 Rust 的 Traits（例如定义一个 `IntoIconPath` trait，让 `&str` 和 `IconName` 都实现它），从而达成最精简、最优雅的 API 调用形态。请给出完整的组件结构和 Trait 实现代码。
```
---

## 5. 图标资源打包自动优化（Icon Bundle Auto Optimization）

### 5.1 目标与边界

Liora 的内置图标库规模较大，如果每个 `liora-icons-*` crate 在 `IconName` 的
`svg_source()` 中对全集 SVG 使用 `include_str!`，最终应用二进制和安装包会被未使用
图标资源显著放大。图标资源打包自动优化的目标是：

1. **业务代码不变**：应用仍然使用现有写法，例如
   `Icon::new(liora_icons_lucide::IconName::Search)` 或
   `Icon::new(LucideIconName::Search)`。
2. **清单不是业务 API**：自动清单只作为构建优化产物，业务代码不从清单引用图标。
3. **应用显式接入构建脚本**：应用方在唯一的 Cargo build script 中调用 builder：
   `liora_icons_optimizer::Optimizer::new().bundle_auto().run();`。如果已有
   `build.rs`，在现有逻辑前后追加该调用即可；如果想使用非默认文件名，可通过
   `[package] build = "..."` 指定，但 Cargo 仍只有一个 build script 入口。
4. **SDK 不耦合业务实现**：`liora-icons-optimizer` 只扫描调用方 crate 及其 Cargo
   metadata 中的 Liora 依赖源码，不写死 Gallery/Docs 路径，不了解业务页面、菜单或
   demo 结构。
5. **默认可开发、发布可瘦身**：开发环境可从源码 SVG 目录回退加载；打包/发布时只将
   optimizer 发现并复制的图标资源放入应用资源目录。

### 5.2 Builder 接入

应用 `Cargo.toml` 只需要增加构建依赖（业务代码不变）：

```toml
[build-dependencies]
liora-icons-optimizer = "0.1"
```

应用 `build.rs`：

```rust
fn main() {
    liora_icons_optimizer::Optimizer::new()
        .bundle_auto()
        .run();

    // existing build script logic can stay here.
}
```

高级用法可覆盖扫描目录、输出目录或报告目录：

```rust
fn main() {
    liora_icons_optimizer::Optimizer::new()
        .scan_dir("src")
        .scan_dir("content/snippets")
        .asset_out_dir("target/liora/icons/assets/liora-icons")
        .report_file("target/liora/icons/liora_icon_bundle_report.md")
        .bundle_auto()
        .run();
}
```

### 5.3 运行时资源路径

图标库的 `IconName` 不再直接返回内联 SVG 文本，而是返回稳定虚拟路径：

```text
liora-icon://lucide/search.svg
```

虚拟路径也可携带开发期源码 SVG fallback：

```text
liora-icon://lucide/search.svg?dev=/path/to/liora-icons-lucide/assets/svgs/search.svg
```

`IconAssetSource` 解析顺序：

1. 应用可执行文件旁的 `assets/liora-icons/<set>/<file>.svg`。
2. 当前工作目录下的 `assets/liora-icons/<set>/<file>.svg`。
3. 虚拟路径中的 `dev=` 源码 fallback（仅开发/本机运行可靠）。
4. 普通显式文件路径（兼容自定义 SVG 路径）。

因此 optimizer 只需要在构建时把实际使用的 SVG 复制到标准资源目录；业务代码无需
改为引用清单。

### 5.4 自动登记与删除

`Optimizer::bundle_auto()` 每次执行都全量扫描并重建输出目录：

1. 扫描调用方源码目录（默认 `src/`）。
2. 通过 `cargo metadata` 发现当前 crate 的 Liora 依赖，扫描这些依赖的 `src/`，确保
   `liora-components` 内部使用的图标也进入 bundle。
3. 识别强类型图标使用点：
   - `liora_icons_lucide::IconName::Search`
   - `liora::icons_lucide::IconName::Search`
   - `use liora_icons_lucide::IconName; IconName::Search`
   - `use liora_icons_lucide::IconName as LucideIcon; LucideIcon::Search`
   - `use liora::icons_lucide::IconName as LucideIcon; LucideIcon::Search`
4. 将图标 variant 映射为对应 SVG 文件名并复制到
   `target/liora/icons/assets/liora-icons/<set>/<file>.svg`。
5. 生成报告 `target/liora/icons/liora_icon_bundle_report.md`。

因为输出目录每次全量重建，所以新增图标会自动登记，删除图标使用后也会自动从资源
bundle 中移除。

### 5.5 打包集成

`liora-packager` / `xtask package` 应将应用构建产生的
`target/liora/icons/assets/liora-icons` 放进最终资源目录：

```text
assets/liora-icons/<set>/<file>.svg
```

这样 release raw executable、portable archive 和 installer 均可在没有源码树的目标机器
上解析图标。Gallery/Docs 只是普通应用接入此能力；SDK 模块不能写死它们的业务路径。

### 5.6 失败策略

- build script 中 `.run()` 适合普通应用：失败时输出 `cargo:error` 并终止构建。
- `.try_run()` 适合高级用户：返回 `Result`，由应用 build script 自行处理。
- 如果运行时缺少已使用图标资源，`IconAssetSource` 返回 `None`，GPUI 渲染会按缺失
  asset 处理；报告中会提示可运行 optimizer 重新生成资源 bundle。
