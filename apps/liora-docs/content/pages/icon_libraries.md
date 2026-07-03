# Icon Libraries

Liora ships typed SVG icon packs for Lucide, Ant Design, Ionicons, Tabler, Carbon, and Material Design. Every icon pack follows the same API shape as `liora-icons-lucide`: each crate exposes an `IconName` enum, `IconName::all()`, `IconName::file()`, `IconName::svg_path()`, and implements `liora_icons::IntoIconPath` plus `gpui::IntoElement`. `IconName` now resolves to a virtual `liora-icon://...` asset path so applications can keep existing icon usage while `liora-icons-optimizer` copies only the actually used SVG files into the final package resources.

## 快速使用

使用顶层 `liora` facade 时可以直接从对应模块导入图标名；使用拆分 crate 时也可以从 `liora-icons-*` 包导入。

```rust src="icon/libraries.rs"
```

## 命名规则

| 图标库 | crate | facade 模块 | IconName 规则 | 示例 |
| --- | --- | --- | --- | --- |
| Lucide | `liora-icons-lucide` | `liora::icons_lucide` | upstream kebab-case 转 PascalCase | `IconName::Settings` |
| Ant Design | `liora-icons-antd` | `liora::icons_antd` | 图标名 + `Filled` / `Outlined` / `Twotone` | `IconName::SaveOutlined` |
| Ionicons | `liora-icons-ionic` | `liora::icons_ionic` | 基础名，必要时追加 `Outline` / `Sharp` | `IconName::AddCircleOutline` |
| Tabler | `liora-icons-tabler` | `liora::icons_tabler` | outline 使用基础名，filled 追加 `Filled` | `IconName::HomeFilled` |
| Carbon | `liora-icons-carbon` | `liora::icons_carbon` | Carbon 名称扁平化为 PascalCase，每个图标保留一个优先尺寸 | `IconName::CheckmarkFilled` |
| Material | `liora-icons-material` | `liora::icons_material` | 默认名，或追加 `Outlined` / `Round` / `Sharp` / `Twotone` | `IconName::SearchOutlined` |


## 图标打包自动瘦身 (Icon Bundle Auto Optimization)

由于 Liora 提供了多达数万个图标，如果在打包发布时将全部 SVG 打入应用中，会导致可执行文件和安装包体积异常臃肿（多达数十MB）。
Liora 引入了 **Icon Bundle Auto Optimization** 打包优化机制：**业务代码完全不需要改动**，依旧使用现有的 `IconName` 强类型写法。在项目打包构建时，Liora 的 optimizer 会自动扫描应用和依赖源码中实际使用到的图标，仅将这部分 SVG 文件复制到打包资源目录，从而将资源大小压缩至几十 KB。

### 集成步骤

#### 1. 配置 `Cargo.toml`
根据项目类型在应用包的 `[build-dependencies]` 中加入构建期依赖：

- **若是 Liora 根 monorepo 仓库内的应用** (如 Gallery/Docs)：
  ```toml
  [build-dependencies]
  liora-icons-optimizer = { path = "../../crates/liora-icons-optimizer" }
  ```
- **若是依赖 Liora SDK 的外部独立应用**：
  ```toml
  [build-dependencies]
  liora-icons-optimizer = "0.2"
  ```

#### 2. 在应用的唯一的 `build.rs` 中引入 Builder
在你的 `build.rs` 中，用链式调用的 Builder 风格运行优化器：

```rust
fn main() {
    // 运行图标打包自动优化
    liora_icons_optimizer::Optimizer::new()
        .bundle_auto()
        .run();

    // 你原有的 build.rs 逻辑（如 windows 图标、清单编译等）可以安全地写在后面
}
```

你也可以自定义配置扫描的源码目录、输出的资源目录以及分析报告的路径：

```rust
fn main() {
    liora_icons_optimizer::Optimizer::new()
        .scan_dir("src")
        .scan_dir("content/pages") // 额外扫描 markdown 页面里的图标使用点
        .asset_out_dir("target/custom_icons") // 自定义图标资源输出目录
        .report_file("target/report.md") // 自定义分析报告路径
        .bundle_auto()
        .run();
}
```

### 它是如何工作的？
- **自动扫描与删除**：每次调用 `cargo check` / `cargo run` 或 `cargo build` 时，优化器都会扫描源码中形如 `IconName::Search`、`lucide::IconName::Check` 的强类型使用，重建 `target/liora/icons/apps/<app>/assets/liora-icons`。如果你在代码中删除了某图标，下一次构建时它就会被自动从 bundle 资源目录中剔除。
- **报告自动生成**：optimizer 会写入 `target/liora/icons/reports/<app>.md`，其中包含扫描根目录、实际复制图标、运行时搜索路径。这个报告仅用于诊断，不需要开发者手动参与打包。
- **开发与打包双通道**：
  - **开发环境**：在本地运行开发时，如果没有生成的打包资源，`IconAssetSource` 会通过 `dev=` 参数自动回退到本地的 Liora 图标源码目录加载 SVG，不影响日常开发调试的效率。
  - **打包环境**：通过 `cargo-packager` 或 `xtask` 打包发布时，打包工具会自动将生成的 `assets/liora-icons` 目录打包入最终安装包，在没有开发源码树的用户电脑上实现无缝加载。
- **可见 fallback**：如果虚拟图标资源最终仍然缺失，`IconAssetSource` 会返回一个可见占位 SVG，避免页面出现难以排查的空白。
- **按需诊断**：设置 `LIORA_ICON_DEBUG=1` 后，运行时会打印候选路径、命中路径和 fallback 决策。正常开发与打包不需要设置这个变量。
- **自定义 SVG 不受影响**：optimizer 只扫描 Liora 内置强类型图标库的 `IconName::...` 使用点。业务项目自己的 `Icon::new("assets/icons/foo.svg")`、`Icon::new("file:///...")` 和 `inline_svg_asset_path(...)` 仍然走普通应用 assets / 文件路径 / 内联 SVG 加载链路，不会被 optimizer 复制、改写或删除。
- **自定义图库的科学做法**：如果只是少量业务图标，把 SVG 放在应用自己的 `assets/` 目录并让打包器复制即可；如果要做可复用、可优化的自定义图库，应单独做一个类似 `liora-icons-yourpack` 的 crate，提供 `IconName` enum 和稳定 SVG 目录，而不是把业务资源混入自动生成的 `assets/liora-icons`。
- **避免误用全量图标**：`IconName::all()` 会让 optimizer 自动打包整个图标库。它适合 Docs 的图标清单页面；普通应用应使用具体枚举变体，让包体保持最小。

## 完整 IconName 清单在哪里？

Docs 左侧 `图标库` 分组下按图标库拆分为 `Lucide Icons`、`Ant Design Icons`、`Ionicons`、`Tabler Icons`、`Carbon Icons`、`Material Icons` 六个页面。每个页面使用虚拟化 + 自适应 `Grid` 渲染该库的完整图标墙，点击任意正方形 item 即可复制完整 Rust 路径。
