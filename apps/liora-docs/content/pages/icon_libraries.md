# Icon Libraries

Liora ships typed SVG icon packs for Lucide, Ant Design, Ionicons, Tabler, Carbon, and Material Design. Every icon pack follows the same API shape as `liora-icons-lucide`: each crate exposes an `IconName` enum, `IconName::all()`, `IconName::file()`, `IconName::svg_path()`, and implements `liora_icons::IntoIconPath` plus `gpui::IntoElement`.

`IconName` resolves to a virtual `liora-icon://...` asset path. Application code can keep using strong Rust enum names while release builds ship only the SVG files staged by the separate **Icon Optimizer** page.

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

## 完整 IconName 清单

Docs 左侧 `图标库` 分组下按图标库拆分为 `Lucide Icons`、`Ant Design Icons`、`Ionicons`、`Tabler Icons`、`Carbon Icons`、`Material Icons` 六个页面。每个页面使用虚拟化 + 自适应 `Grid` 渲染该库的完整图标墙，点击任意正方形 item 即可复制完整 Rust 路径。

## 打包优化入口

图标打包瘦身不在本页展开。请阅读 `Icon Optimizer` 页面：它说明如何在 `build.rs` 中自动扫描 `IconName::...` 使用点、如何生成 `target/liora/icons/apps/<app>/assets/liora-icons`、运行时如何 fallback，以及为什么自定义业务 SVG 不会被 optimizer 改写。
