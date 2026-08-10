

<!-- ctx-managed-legacy-migration:start -->

## Migrated legacy source units

The following sections preserve legacy context content verbatim enough for auditability. Prefer the summarized CTX sections above for day-to-day work.

### prompt-p0-foundation-md-0001-9f6de76c2a8c

<!-- ctx-migration source=".prompt/P0-foundation.md" unit="prompt-p0-foundation-md-0001-9f6de76c2a8c" sha256="9f6de76c2a8c2e858e6c0e5f5f97e5e4434a2e8f078553b30d6215caa7b2b563" -->

# P0 Foundation — 完成总结

> 下一阶段: `.prompt/P1-basic-elements.md`


### prompt-p0-foundation-md-0002-8bf3ee4876d2

<!-- ctx-migration source=".prompt/P0-foundation.md" unit="prompt-p0-foundation-md-0002-8bf3ee4876d2" sha256="8bf3ee4876d23cf8a31d46587ecb8f4183723bd52c4fd01c5c762f9810b1e22d" -->

## 已完成工作


### prompt-p0-foundation-md-0003-7ce78fc8774a

<!-- ctx-migration source=".prompt/P0-foundation.md" unit="prompt-p0-foundation-md-0003-7ce78fc8774a" sha256="7ce78fc8774ad2267fb7013f121c5f3751c5934ff9bb93418ef12bd20e58d8cd" -->

### 工程骨架
- Cargo workspace: `crates/{liora-core, liora-theme, liora-components, liora-icons}` + `apps/{liora-gallery, docs}`
- GPUI git 依赖策略: workspace `default-features=false`, app 显式 `features=["wayland","x11","font-kit"]`


### prompt-p0-foundation-md-0004-898cea12111e

<!-- ctx-migration source=".prompt/P0-foundation.md" unit="prompt-p0-foundation-md-0004-898cea12111e" sha256="898cea12111e31d42760b503b6c6e60dda958f5b0f2f096ee913786757e5a9f5" -->

### liora-theme (设计系统)
- `Theme` struct: 亮/暗双主题，Design Tokens（色板/间距/圆角/字号）
- `ButtonVariant` (6 种), `ButtonSize` (3 种), `ButtonVariantColors`
- `color_by_variant()` 自动计算配色
- 完整 Element-Plus 色板: Primary/Success/Warning/Danger/Info + hover/active 态


### prompt-p0-foundation-md-0005-85a38ebf643b

<!-- ctx-migration source=".prompt/P0-foundation.md" unit="prompt-p0-foundation-md-0005-85a38ebf643b" sha256="85a38ebf643be9ab6cc00bbe152d33263243a91174b2aadb44b0536b7fa9748c" -->

### liora-core (核心层)
- `Config` 实现 `gpui::Global` trait
- `init_liora(cx: &mut App, theme)` 全局注入
- `ContextExt` trait: 为 `Context<'_, V>` 提供 `.liora() -> &Theme`
- `ElementExt` 通用 trait 骨架
- Z-Index 工具函数: `z_index_popup/modal/notification/tooltip`


### prompt-p0-foundation-md-0006-f98bb857bf43

<!-- ctx-migration source=".prompt/P0-foundation.md" unit="prompt-p0-foundation-md-0006-f98bb857bf43" sha256="f98bb857bf43ab479193742fc42dc82ccf2d30463cde23f8d5b02709393669b8" -->

### liora-components (组件)
- `Button` Builder 模式组件
  - 6 种变体: `.primary()` `.success()` `.warning()` `.danger()` `.info()`
  - 3 种尺寸: `.small()` `.large()` + 默认
  - 状态: `.disabled(bool)` `.loading(bool)`
  - 构建: `.build(&theme) -> impl IntoElement`


### prompt-p0-foundation-md-0007-e4cb9edc8a2d

<!-- ctx-migration source=".prompt/P0-foundation.md" unit="prompt-p0-foundation-md-0007-e4cb9edc8a2d" sha256="e4cb9edc8a2d0978882f32a8b395b2b2284fe2573b6f276bfb0411586d85321c" -->

### liora-icons (图标)
- `Icon` trait (需 `Styled` supertrait)
- `IconSize` 枚举 (Small/Default/Large)
- 10 个占位图标函数 (纯文本, 待替换为 SVG)


### prompt-p0-foundation-md-0008-b0c9fd40c279

<!-- ctx-migration source=".prompt/P0-foundation.md" unit="prompt-p0-foundation-md-0008-b0c9fd40c279" sha256="b0c9fd40c279fb10a13704cdc423cc4513fd3bbbd4c38fda1c250509825817bf" -->

### liora-gallery (看板)
- Gallery struct: 分类卡片式组件展示
- `category.rs`: 6 种 Category 枚举
- `demos/mod.rs`: DemoEntry 注册表, `registry()` 函数
- `demos/button_demo.rs`: Button 四小节 Demo
- **增量规约**: 新增 Demo = 1 demo 文件 + 1 registry 行


### prompt-p0-foundation-md-0009-4e90ffaa456c

<!-- ctx-migration source=".prompt/P0-foundation.md" unit="prompt-p0-foundation-md-0009-4e90ffaa456c" sha256="4e90ffaa456cb5be587edd6eb04ada6acadd0f5c7b196488386089bb399f42a3" -->

## 关键架构决策

1. **Builder Pattern**: 所有组件 Builder::new().method().build(&theme)
2. **Global Theme**: cx.set_global() → cx.global::<Config>()
3. **组件/主题解耦**: .build(&theme) 显式传参，不隐式读 Global
4. **Demo 注册表**: 函数指针 + AnyElement 返回，类型统一存储


### prompt-p0-foundation-md-0010-f60f92be7f8a

<!-- ctx-migration source=".prompt/P0-foundation.md" unit="prompt-p0-foundation-md-0010-f60f92be7f8a" sha256="f60f92be7f8a5b1cd409bd25f4b2add0bc6e6d8038e30fdb1f8dd2fe0d8e3e85" -->

## 编译状态

```
cargo check  → 0 errors, 0 warnings ✅
cargo run -p liora-gallery → 窗口正常打开 ✅
```

### prompt-p1-basic-elements-md-0001-79cb7c560842

<!-- ctx-migration source=".prompt/P1-basic-elements.md" unit="prompt-p1-basic-elements-md-0001-79cb7c560842" sha256="79cb7c56084247f3db5400c7b8e6f743ded9fd4b78fbe46c77639d6cdf515e54" -->

# P1 Basic Elements — 基础物料层

> 上游: `.prompt/P0-foundation.md` | 主文档: `architecture-design.md#四`


### prompt-p1-basic-elements-md-0002-6d20c901a631

<!-- ctx-migration source=".prompt/P1-basic-elements.md" unit="prompt-p1-basic-elements-md-0002-6d20c901a631" sha256="6d20c901a631cb2fbdeeb4dcb2cd9691e36ea9a1e6830efb9cbd95994c4c9834" -->

## 目标

完成 13 个基础组件的开发和 Gallery Demo 注册。


### prompt-p1-basic-elements-md-0003-d04ece69806b

<!-- ctx-migration source=".prompt/P1-basic-elements.md" unit="prompt-p1-basic-elements-md-0003-d04ece69806b" sha256="d04ece69806b2ff2758921dfdf5622ecc767048c33e5c014ab111284bae340b1" -->

## 组件清单 (共 13 个，按优先级排列)


### prompt-p1-basic-elements-md-0004-80e537e5418c

<!-- ctx-migration source=".prompt/P1-basic-elements.md" unit="prompt-p1-basic-elements-md-0004-80e537e5418c" sha256="80e537e5418c0f96146daea598947d4dc362fd5d001088d8f7d5be8292bec4cd" -->

### 第一优先级 (完善现有)
1. **Button 增强** — 添加 `.icon_start()` / `.icon_end()` 支持、`ButtonGroup`、幽灵按钮 (text variant)
   - 文件: `crates/liora-components/src/button.rs` (修改)
   - 文件: `crates/liora-components/src/button_group.rs` (新建)
   - Demo: 更新 `apps/liora-gallery/src/demos/button_demo.rs`


### prompt-p1-basic-elements-md-0005-b52a0969be82

<!-- ctx-migration source=".prompt/P1-basic-elements.md" unit="prompt-p1-basic-elements-md-0005-b52a0969be82" sha256="b52a0969be82609378936623054259dadc14193ab664809f8f6342ba97f5d485" -->

### 第二优先级 (核心布局与排版)
2. **Link** — 链接按钮 (underline, hover 变色)
3. **Text** — 单行文本组件 (截断 ellipsis, 行数限制)
4. **Title** — 标题组件 (h1-h6 级别, 字重)
5. **Paragraph** — 段落组件 (行高, 首行缩进)
6. **Space** — 间距包裹组件 (横向/纵向自动 gap)
7. **Divider** — 分割线 (横向/纵向, 带文字, 虚线样式)


### prompt-p1-basic-elements-md-0006-9df790d34e96

<!-- ctx-migration source=".prompt/P1-basic-elements.md" unit="prompt-p1-basic-elements-md-0006-9df790d34e96" sha256="9df790d34e9627068d433a2e3a31f32c31af153b2721bdb2d0d85dbbb0eb260a" -->

### 第三优先级 (布局系统)
8. **Row** — 栅格行 (gutter, justify, align)
9. **Col** — 栅格列 (span, offset, push, pull)
10. **Container** — 布局容器 (header/aside/main/footer)
11. **Scrollbar** — 自定义滚动条
12. **Splitter** — 分隔面板 (拖拽调整宽度)


### prompt-p1-basic-elements-md-0007-6d8a44923391

<!-- ctx-migration source=".prompt/P1-basic-elements.md" unit="prompt-p1-basic-elements-md-0007-6d8a44923391" sha256="6d8a44923391f813c989df713394cd704be98626940a67604d95accf748116b9" -->

### 第四优先级 (图标升级)
13. **Icon** — SVG 图标集成 (选择 Lucide 或 Element Icons 作为图标集)


### prompt-p1-basic-elements-md-0008-cd271de9c98c

<!-- ctx-migration source=".prompt/P1-basic-elements.md" unit="prompt-p1-basic-elements-md-0008-cd271de9c98c" sha256="cd271de9c98cdd3fd6679c240386cea512018e91e2865d30d3026695bb34738e" -->

## 开发流程

```
每个组件:
  1. 创建/修改 crates/liora-components/src/<name>.rs
  2. 在 crates/liora-components/src/lib.rs 中 pub mod + pub use
  3. 创建 apps/liora-gallery/src/demos/<name>_demo.rs (render() -> AnyElement)
  4. 在 apps/liora-gallery/src/demos/mod.rs 注册表添加 DemoEntry
  5. cargo check 通过
  6. cargo run -p liora-gallery 验证窗口效果
  7. git add + commit + push
  8. 更新 .memory/inventory.md
```


### prompt-p1-basic-elements-md-0009-fa0011da4d45

<!-- ctx-migration source=".prompt/P1-basic-elements.md" unit="prompt-p1-basic-elements-md-0009-fa0011da4d45" sha256="fa0011da4d459f95173c09d86e591ca697edd2857f24450cde1bb589b0241376" -->

## Demo 编写规范

```rust
use gpui::{div, prelude::*, px, AnyElement, App, Component, RenderOnce, Window};

pub fn render() -> AnyElement {
    Component::new(NameDemo).into_any_element()
}

struct NameDemo;

impl RenderOnce for NameDemo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<liora_core::Config>().theme;
        div().flex().flex_col().gap_4()
            .child(section(theme, "Variants 变体"))
            .child(demo_row(vec![...]))
            .child(section(theme, "Sizes 尺寸"))
            .child(demo_row(vec![...]))
            .child(section(theme, "States 状态"))
            .child(demo_row(vec![...]))
    }
}
```


### prompt-p1-basic-elements-md-0010-e5892bc8b5df

<!-- ctx-migration source=".prompt/P1-basic-elements.md" unit="prompt-p1-basic-elements-md-0010-e5892bc8b5df" sha256="e5892bc8b5dfcfb75a4f6f003f25a51bc673054a07ce3f46ab07e1cf454444b7" -->

## 布局组件特殊说明

Row/Col 栅格系统参照 Element-Plus 24 栅格:
```rust
LioraRow::new()
    .gutter(px(20.0))
    .child(LioraCol::new().span(12).child(...))
    .child(LioraCol::new().span(6).offset(6).child(...))
```


### prompt-p1-basic-elements-md-0011-a3aadaf02016

<!-- ctx-migration source=".prompt/P1-basic-elements.md" unit="prompt-p1-basic-elements-md-0011-a3aadaf02016" sha256="a3aadaf020166d8fb068fe85ace4d44ea0e8f6b2e432effbf2e0e5981da09177" -->

## 完成标准

- [ ] 全部 13 个组件编译通过 (cargo check 0 errors)
- [ ] 每个组件在 Gallery 中有 Demo 卡片
- [ ] `cargo run -p liora-gallery` 可滚动查看全部组件
- [ ] Git commit 已推送
- [ ] .memory/ 已更新 (state.md, inventory.md)
- [ ] .prompt/P2-form-controls.md 已就绪

### prompt-p10-charts-md-0001-b541acefa304

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0001-b541acefa304" sha256="b541acefa304dd3eb6e1b874860b29bc3bb8ef8843463d941c89c77e039cb033" -->

# P10 Charts — 原生统计图组件

> 上游: `.prompt/P8-engineering.md` / 当前阶段
> 参考: GPUI 官方源码优先；`https://github.com/vicanso/zedis` 仅作为 GPUI Metrics 图表案例参考


### prompt-p10-charts-md-0002-3dfbbd3345bb

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0002-3dfbbd3345bb" sha256="3dfbbd3345bb8ad859a89c1c2a18233646b3661e2e3369e0b07fcd75c38287e6" -->

## 目标

为 Liora UI 新增一组企业级统计图控件，用于 Dashboard、监控、报表和数据分析页面。所有图表必须是 **Rust + GPUI 原生绘制**，并作为 `liora-components` 的一等组件交付。


### prompt-p10-charts-md-0003-a07fd0dd82d4

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0003-a07fd0dd82d4" sha256="a07fd0dd82d40311a95bff4722704fcf6fa2c764ebcdbd2228d769d0f3a3990b" -->

## 绝对边界

- 禁止 WebView、HTML、CSS、DOM、SVG DOM、WASM、ECharts、Vega、Plotly 等 Web 图表运行时。
- 禁止把图表作为图片远程渲染后嵌入；图表必须可由 GPUI paint pipeline 直接绘制。
- 外部案例仓库只能启发结构和 GPUI 用法，不能照搬 API、命名或依赖。
- 如 GPUI API 不确定，优先查本地 `gpui` 源码和当前仓库既有用法。


### prompt-p10-charts-md-0004-aa2c856df918

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0004-aa2c856df918" sha256="aa2c856df91809d21cbd0191b592ecdc2c0b01d83d5dffc19326659128e86e4d" -->

## 技术路线


### prompt-p10-charts-md-0005-ad6b144cb654

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0005-ad6b144cb654" sha256="ad6b144cb654717ad737b04989c6831c4f7a87d919b39c945072f24764ebc6c7" -->

### GPUI 官方能力优先

当前本地 GPUI 源码提供的关键能力：

- `gpui::canvas(prepaint, paint)`：短期自定义绘制入口。
- `gpui::PathBuilder`：构建 stroke/fill path，支持 line、curve、arc、polygon、dash array。
- `Window::paint_path(path, color)`：绘制矢量 path。
- `Window::paint_quad(...)` / `fill(...)` / `quad(...)`：绘制柱体、背景、网格辅助块。
- GPUI TextSystem / Liora `Text`、`Paragraph`：绘制标题、坐标轴标签、legend、tooltip 文案。


### prompt-p10-charts-md-0006-16a3432a93c0

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0006-16a3432a93c0" sha256="16a3432a93c0b3a1def3a11ba1e1ccab1be511f7a74ce78f10a83688e847b793" -->

### zedis 案例参考结论

`vicanso/zedis` 的 Metrics 页面采用 GPUI 原生方案：

- 用 `canvas(...)` 建立图表绘制区域。
- 将图表拆为 `scale`、`axis/grid`、`shape` 层。
- Area/Line/Bar 都在 paint 回调中根据 bounds 计算 scale，再绘制 path/quad。

Liora P10 采用同类分层思想，但实现自己的组件 API、主题 token、测试、Gallery demo 和 Docs 文档。


### prompt-p10-charts-md-0007-4095ffdb381e

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0007-4095ffdb381e" sha256="4095ffdb381e8ac37d17caae49071f3ed7ed278b1890fddbff13c29da1ba0771" -->

## 架构分层

建议文件结构：

```text
crates/liora-components/src/
├── chart.rs            # 公共数据模型、ChartTheme、Legend、Tooltip、ChartFrame
├── chart_scale.rs      # ScaleLinear / ScaleBand / ScalePoint
├── chart_axis.rs       # Axis、Grid、Tick、Label 布局
├── chart_shape.rs      # Line/Area/Bar/Pie/Ring/Sparkline 绘制 primitive
├── line_chart.rs       # LineChart 组件
├── area_chart.rs       # AreaChart 组件
├── bar_chart.rs        # BarChart 组件
├── pie_chart.rs        # PieChart / RingChart 组件
└── sparkline.rs        # Sparkline 组件
```

如实现初期文件过多影响 review，可先在 `chart.rs` 内部模块化，稳定后再拆文件。


### prompt-p10-charts-md-0008-f1b39c44bcf0

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0008-f1b39c44bcf0" sha256="f1b39c44bcf0455149abac938b78ba1d62a60492824a874318d2cf1368504dc5" -->

## 数据模型建议

```rust
ChartSeries::new("CPU")
    .points(vec![ChartPoint::new("10:00", 12.0), ...])
    .color(theme.primary.base)

LineChart::new(series)
    .height(px(280.0))
    .show_grid(true)
    .show_legend(true)
    .y_format(|v| format!("{v:.0}%"))
```

原则：

- X 轴第一版支持分类值（`SharedString`）和后续可扩展数值/时间值。
- Y 轴第一版支持 `f64`，自动计算 min/max，可手动覆盖 domain。
- 空数据必须渲染 Liora `Empty` 或轻量占位，不 panic。
- 所有组件提供 `.id(...)` 覆盖，默认使用内置唯一 ID。


### prompt-p10-charts-md-0009-e1a39ee2fd5b

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0009-e1a39ee2fd5b" sha256="e1a39ee2fd5bc7255ccf3662e4d94a8cbd9f778c70a181a027919b0c8f768457" -->

## 首批组件范围

| 组件 | 必备能力 | 后续扩展 |
|------|----------|----------|
| `LineChart` | 单/多 series、直线、点标记、grid、axis、legend | smooth curve、step line、hover tooltip |
| `AreaChart` | 单/多 series、透明填充、axis/grid | stacked area、gradient fill |
| `BarChart` | 竖向柱、分类 x 轴、axis/grid、legend | grouped/stacked/horizontal bar |
| `PieChart` | 扇区绘制、百分比、legend | label line、selection offset |
| `RingChart` | donut inner radius、中心文本 | progress ring composition |
| `Sparkline` | tiny line/area，无 axis，适合卡片嵌入 | threshold color、last-point marker |


### prompt-p10-charts-md-0010-779b9cb21ecf

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0010-779b9cb21ecf" sha256="779b9cb21ecf3e54c0202a830fbf5b1755dbba0f2b8d5a91e93ece5ffb9a6326" -->

## Theme / Token

优先复用现有语义色：

1. primary
2. info
3. success
4. warning
5. danger
6. neutral.text_3 / neutral.border / neutral.divider 用于 axis/grid

若视觉不足，再在 `liora-theme` 中新增 `ChartPalette`，包含 `series_1..series_8`、`axis`、`grid`、`tooltip_bg` 等 token。


### prompt-p10-charts-md-0011-0c48695f61df

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0011-0c48695f61df" sha256="0c48695f61df06bdec6c00f8ad4e839f3a7ae5b55de7d4dd3b45440b274a449e" -->

## Gallery / Docs 要求

每个图表控件都必须同时完成：

- `apps/liora-gallery/src/demos/<chart>_demo.rs`
- Gallery registry 注册
- `apps/liora-docs/content/pages/<chart>.md`
- `apps/liora-docs/content/snippets/<chart>/*.rs`
- Docs 页面按“效果 → 对应代码”组织


### prompt-p10-charts-md-0012-a2350589ea52

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0012-a2350589ea52" sha256="a2350589ea5245a2308089fc37b91d3fa0d962556020aa2e516c9dbfcf0efb43" -->

## 测试要求

必须覆盖：

- scale 计算：linear/band/point domain-range 映射、空/单点/负值边界。
- path 数据生成：line/area/bar/pie 的核心点位或角度计算。
- API builder：show_grid/show_legend/height/y_format/id 等状态被正确记录。
- 空数据和异常值：NaN/Infinity 应过滤或降级，不得 panic。
- Gallery/Docs 注册完整性。


### prompt-p10-charts-md-0013-7c5e8623b2b7

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0013-7c5e8623b2b7" sha256="7c5e8623b2b7cca4954fea73110833a9ea6d44d801989e697df1c870dd4d4451" -->

## 推荐执行顺序

1. 研究本地 GPUI `canvas` / `PathBuilder` / `paint_path` 示例，写最小 chart primitive spike。
2. 实现 `chart_scale.rs` + tests。
3. 实现 `chart_axis.rs` + grid/label 布局数据计算 tests。
4. 实现 `LineChart` MVP + demo/docs。
5. 抽象共享 shape 后实现 `AreaChart`、`BarChart`。
6. 实现极坐标基础后实现 `PieChart`、`RingChart`。
7. 实现 `Sparkline` 并集成 Statistic/Card 示例。
8. 性能审查：大 series 采样/降采样策略、缓存、hover hit test 边界。


### prompt-p10-charts-md-0014-0428431980da

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0014-0428431980da" sha256="0428431980dac16a245adc29ed84ad712256c944dd6fa0c53df2b44cb538b13c" -->

## 完成标准

- [x] 首批 6 类图表组件完成并导出。
- [x] Gallery demos 完整且自举。
- [x] Docs 每个控件按“效果 → 代码”展示。
- [x] `cargo fmt`、`cargo check -p liora-components`、`cargo check -p liora-docs`、`cargo check -p liora-gallery`、相关 chart tests 通过。
- [x] `timeout 15s cargo run -p liora-docs`、`timeout 15s cargo run -p liora-gallery` 可启动无即时崩溃。
- [x] `.memory/*` 与 `architecture-design.md` 更新。



### prompt-p10-charts-md-0015-52f78367c40b

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0015-52f78367c40b" sha256="52f78367c40b0979e9b81aa97d85968ee62b01d9c527eeea281f4f884c812e81" -->

## 2026-06-16 Performance maintenance update

- 首批 6 类图表组件已实现：`LineChart`、`AreaChart`、`BarChart`、`PieChart`、`RingChart`、`Sparkline`。
- 已完成第一轮大数据性能增强：`LineChart`、`AreaChart`、`Sparkline` 支持共享 min/max bucket 降采样，通过 `max_render_points(...)` 限制绘制点数，并可用 `disable_downsampling()` 关闭。
- 已完成第二轮大数据性能增强：`LineChart`/`AreaChart` 的 x 轴改为 index-only scale，轴标签通过默认 `max_axis_labels` 稀疏绘制；value label 通过默认 `max_value_labels` 限流。
- 已完成第三轮大数据性能修正：核心采样新增 `downsample_index_range`/`downsample_indexed_values`，LineChart/AreaChart/Sparkline 不再先构建全量 `(index,value)`/GPUI Point 中间 Vec 再采样；demo/snippet 不再靠显式稀疏标签参数掩盖卡顿。
- 降采样策略保留首尾点和局部峰谷，避免长序列在 GPUI native path 中产生过量绘制，同时不隐藏监控尖峰。
- 已完成 Cartesian hover tooltip / hit testing：`LineChart` 与 `AreaChart` Overlay 模式支持原生最近点 tooltip，底层提供可测试的 `nearest_cartesian_hit_point`。
- 已完成 BarChart 矩形 hover tooltip / hit testing：Grouped 命中单根柱，Stacked 命中具体堆叠分段。
- 已完成 PieChart/RingChart 极坐标扇区 hover tooltip / hit testing：Pie 命中扇区，Ring 命中圆环分段并排除内圆空洞。
- 当前 P10 已完成。后续如有真实 profiling 证据，再在独立性能阶段继续缓存策略。



### prompt-p10-charts-md-0016-c7c75a95084d

<!-- ctx-migration source=".prompt/P10-charts.md" unit="prompt-p10-charts-md-0016-c7c75a95084d" sha256="c7c75a95084da782a3896ccb29cc08a2282f0a66752a7eaa373c621f8f6fd451" -->

## 2026-06-17 Cartesian tooltip maintenance

- Completed LineChart and AreaChart Overlay nearest-point hover tooltip support using shared pure hit-testing helpers.
- Completed BarChart rectangular hover hit testing for grouped bars and stacked bar segments.
- Completed PieChart/RingChart polar-sector hover hit testing. P10 is complete; future cache policy work should be driven by profiling evidence in a separate performance phase.

### prompt-p11-tray-md-0001-b9035d75debc

<!-- ctx-migration source=".prompt/P11-tray.md" unit="prompt-p11-tray-md-0001-b9035d75debc" sha256="b9035d75debce10edffce034467deb3ff65c18381776e17f6ca031b97b80ac5d" -->

# P11 — Native Tray / Process Resident Support


### prompt-p11-tray-md-0002-03e9625a8cc0

<!-- ctx-migration source=".prompt/P11-tray.md" unit="prompt-p11-tray-md-0002-03e9625a8cc0" sha256="03e9625a8cc08bda58b2349c67c9169c2e0ce2933f6e64cf041cd4ef86f4710b" -->

## Goal

Add a native Rust system-tray integration layer for Liora applications so GPUI apps can remain resident after their windows close, expose native tray menus, and update tray state dynamically.


### prompt-p11-tray-md-0003-d34e58af7ee0

<!-- ctx-migration source=".prompt/P11-tray.md" unit="prompt-p11-tray-md-0003-d34e58af7ee0" sha256="d34e58af7ee01b76e4fcca9d92961ed84016187dba03e2a67e6c89277924ba9f" -->

## Technical Direction

- Create and maintain `crates/liora-tray` as the public Liora facade.
- Use `tray-icon` for cross-platform tray icons and its `muda` menu re-export for native menu items.
- Do not vendor or fork `tauri-apps/tray-icon`/`muda` unless a future customization requirement cannot be met through public APIs.
- GPUI apps that enable tray residency must use `QuitMode::Explicit` and keep the `LioraTray` handle alive for the whole process lifetime.


### prompt-p11-tray-md-0004-594547f76588

<!-- ctx-migration source=".prompt/P11-tray.md" unit="prompt-p11-tray-md-0004-594547f76588" sha256="594547f76588a8dc1c7f53885de0a719a076b99803e224e0333cf68a99100285" -->

## Required Capabilities

- Basic tray install from `TrayConfig`.
- Dynamic icon updates:
  - `set_icon`
  - `clear_icon`
  - `set_icon_from_rgba`
  - `set_icon_from_path`
- Tooltip and visibility updates.
- Native menu DSL:
  - action item
  - checkbox item
  - separator
  - recursive submenu for 2nd/3rd/N-level menus
- Stable command mapping:
  - `Show`
  - `Hide`
  - `Toggle`
  - `Quit`
  - `SetIcon(name)`
  - `Custom(name)`
- Checkbox state sync via command id.


### prompt-p11-tray-md-0005-93d65ed4851d

<!-- ctx-migration source=".prompt/P11-tray.md" unit="prompt-p11-tray-md-0005-93d65ed4851d" sha256="93d65ed4851dd6a598636b23b333756c298fca3d9ff085c6c2f59fa49b8c14d7" -->

## Demo / Docs Contract

- `liora-gallery` must include a Tray demo entry with large, readable examples of dynamic icon state, CheckBox preferences, and deep submenus.
- `liora-docs` must include a Tray page with external compile-checked snippets for basic install, dynamic icon switching, checkbox menus, and nested menus.
- Normal docs/gallery previews should avoid creating a real OS tray icon unless a future explicit runtime integration demo is requested; use config previews to avoid intrusive tray side effects during component browsing.


### prompt-p11-tray-md-0006-07bda184b304

<!-- ctx-migration source=".prompt/P11-tray.md" unit="prompt-p11-tray-md-0006-07bda184b304" sha256="07bda184b304134a04bc1ac5fd2de1f0f30419078ff12f06cf09b02cdd208f6c" -->

## Platform Notes

- Linux requires GTK/AppIndicator dependencies and same-thread event-loop/tray creation.
- macOS tray creation must happen on the main thread; template icon mode is useful for menu-bar appearance.
- Windows/Linux/macOS all require the tray handle to be retained; dropping it removes the icon.

### prompt-p12-packaging-md-0001-480b1103b0cc

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0001-480b1103b0cc" sha256="480b1103b0cc5b28ed8b4b315a8b897487297002bdc907fdca2d1d2e7f90a6e6" -->

# P12 — Native Installer Packaging


### prompt-p12-packaging-md-0002-772e9be4570f

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0002-772e9be4570f" sha256="772e9be4570f19a9216907c56d494008a5cfcef0732eb0f1df61a56036767575" -->

## Goal

Build a cross-platform installer and package generation pipeline for Liora's pure Rust + GPUI native applications.


### prompt-p12-packaging-md-0003-1bd56fba0bec

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0003-1bd56fba0bec" sha256="1bd56fba0beca8413074f0c18c890dc2b2eab497fd404691963711bb9a743b05" -->

## Non-negotiable Constraint

Liora applications must remain pure Rust + GPUI native apps. Do not convert `liora-gallery`, `liora-docs`, or future Liora apps to Tauri. Do not introduce WebView, HTML/CSS/DOM, browser runtime, or frontend build systems as application runtime dependencies.


### prompt-p12-packaging-md-0004-87c084a13d72

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0004-87c084a13d72" sha256="87c084a13d72e00a3f8f9c92dbf5fd46d5e030dbee689c5dd0dc6d2012ba136c" -->

## Naming Decision

The internal packaging module is named `liora-packager`, not `liora-installer`.

- `liora-packager` is a Rust library for packaging domain logic.
- `xtask` is the command entrypoint: `cargo run -p xtask -- package ...`.
- `packaging/` stores static platform resources and packager configuration.


### prompt-p12-packaging-md-0005-42b3fb41be4b

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0005-42b3fb41be4b" sha256="42b3fb41be4ba36313420863e0cbc8e63ca02d597c6ba44489331e11519785b4" -->

## Technical Direction

- `crates/liora-packager`: app metadata, package formats, checksums, output manifests, validation helpers.
- `xtask`: build orchestration, app/format selection, future cargo-packager/RPM backend invocation.
- `packaging/`: Packager config, icons, Linux desktop/metainfo, macOS entitlements, Windows installer resources.
- Primary backend: `cargo-packager` for app/dmg/deb/AppImage/NSIS/MSI/Pacman where practical.
- RPM backend: `cargo-generate-rpm` or `nfpm` as a supplemental path.


### prompt-p12-packaging-md-0006-0f114541d113

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0006-0f114541d113" sha256="0f114541d113f48e7bddf31ca8e8a957b151d6afc08f56425b75e2a2a43785e2" -->

## Required Package Formats

- Linux: AppImage, deb, rpm, portable tar.gz.
- macOS: app, dmg.
- Windows: NSIS exe, MSI.


### prompt-p12-packaging-md-0007-bb6103347671

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0007-bb6103347671" sha256="bb6103347671a01c9cbb3b5508fb87cd42f1c30db7647d244266631828b7be2b" -->

## Current Readiness Baseline

- `docs/packaging-installer-technical-plan.md` is the source technical plan.
- `crates/liora-packager` contains the packaging domain model: known apps, platform formats, checksums, manifests, release notes, cargo-packager config generation, and RPM metadata generation.
- `xtask package` is the public entrypoint for validate/build/package/ci/smoke/install-smoke flows.
- `packaging/` contains real Liora/Gallery/Docs icons plus Linux desktop/metainfo, macOS entitlements, and Windows installer resource directories.
- `.github/workflows/package.yml` runs Linux/macOS/Windows package preview builds, uploads raw binaries and packages, performs artifact smoke, generates install-smoke plans, groups changelog entries by commit type, and validates `vX.Y.Z` release tags against `crates/liora-packager/Cargo.toml`.


### prompt-p12-packaging-md-0008-1b86bd1127ba

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0008-1b86bd1127ba" sha256="1b86bd1127ba3b776f9a8fd7ff6211cb96a9c3699841cc9a406f4d723a5d92a7" -->

## Completion Status

P12 repository-owned implementation is complete. The earlier external-policy TODOs have been converted into explicit release readiness gates instead of remaining as loose notes:

1. Signing / notarization: documented in `packaging/signing-policy.md`; `xtask package release-readiness` warns by default and fails strict release runs when `LIORA_REQUIRE_SIGNING=true` and required platform secrets are missing.
2. Real system-level install/uninstall: `xtask package install-smoke` writes auditable install/launch/uninstall plans for every artifact and executes only the safe portable tar path locally; destructive/system-level execution remains intentionally gated by approved runners.
3. License policy: `LICENSE.md` declares the current `LicenseRef-Liora` policy until the owner chooses OSS or commercial terms.
4. Real `v*` release path: `.github/workflows/package.yml` validates tag/version matching, runs release-readiness before packaging, downloads package/raw-binary artifacts, generates grouped release notes, and publishes GitHub Releases.
5. Artifact naming normalization: portable tar naming is deterministic; backend-produced installer names remain smoke-validated and should only be further normalized after real backend evidence requires it.

---


### prompt-p12-packaging-md-0009-14dd7bf03241

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0009-14dd7bf03241" sha256="14dd7bf032413180fb88b2cb48a11869b0908f9bc6052aa71229ab1075810535" -->

## Handoff Snapshot — 2026-05-15

> 接手入口：本节是打包器当前进度和剩余工作的最新交接信息。后续继续 P12 时优先阅读本节，再看 `docs/packaging-installer-technical-plan.md`。


### prompt-p12-packaging-md-0010-ee29e6ec4e32

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0010-ee29e6ec4e32" sha256="ee29e6ec4e32b4e76dc5d6e0c369060687a267ddb5ece0944c92bdcc11c8266f" -->

### 已完成并推送

- `crates/liora-packager`
  - app metadata / known app registry：Gallery、Docs。
  - package format model：Linux/macOS/Windows 默认格式。
  - checksum：SHA-256。
  - package manifest：JSON manifest、`checksums.txt`、`release-notes.md`。
  - `cargo-packager` 配置生成。
  - `cargo-generate-rpm` metadata overwrite 配置生成。
- `xtask`
  - `cargo run -p xtask -- package validate`
  - `cargo run -p xtask -- package build --app <gallery|docs>`
  - `cargo run -p xtask -- package --app <gallery|docs> --format <format>`
  - `cargo run -p xtask -- package ci --all-apps --format platform-defaults`
  - `--dry-run --skip-build` 可生成后端配置并打印真实后端命令。
- `packaging/`
  - Gallery / Docs packager metadata skeleton。
  - Linux `.desktop` / metainfo。
  - macOS entitlements placeholder。
  - Windows nsis/wix resource folders（用 `.gitkeep` 跟踪空目录，确保 GitHub runner validate 通过）。
  - app icon sets：`liora-gallery.*`、`liora-docs.*`。
  - main Liora brand icon 已选第 3 套 ribbon，落到 `packaging/icons/liora.*`。
- CI
  - `.github/workflows/package.yml` 已添加 Linux/macOS/Windows packaging matrix。
  - package artifact 上传前运行 `cargo run -p xtask -- package smoke ...`，对 portable `.tar.gz` 做结构校验，并对其他格式做 runner-safe 头部/非空检查。
  - CI 真实反馈修正：`cargo-generate-rpm --metadata-overwrite` 必须使用 `GenerateRpm.<app>.toml#package.metadata.generate-rpm` 分支加载，且生成 TOML 必须把 metadata 放在 `[package.metadata.generate-rpm]` 下、依赖放在 `[package.metadata.generate-rpm.requires]` 下。
  - CI 真实反馈修正：artifact collection/smoke 必须忽略 `.cargo-packager` 等隐藏后端工作目录，避免把 deb 内部 `control.tar.gz` 误判为 Liora portable tar.gz。
  - `workflow_dispatch` 默认 dry-run。
  - `main` push 触发 preview 打包，包版本使用 `LIORA_PACKAGE_VERSION=<base>-preview.<run_number>.<short_sha>`。
  - `v*` tag 触发 release 打包，包版本使用 tag 去掉 `v` 后的版本。
  - 上传 `target/packages/**` 和 `target/liora-packager/*.toml`，artifact 命名区分 `liora-preview-packages-*` / `liora-release-packages-*`。
  - release job 下载各平台 release artifacts，按 `feat` / `fix` / `docs` / `ci` / `build` / `refactor` / `perf` / `test` / `style` / `chore` / `revert` / `Other` 分组收集 git changelog，创建/更新 GitHub Release，并上传全部构建产物。


### prompt-p12-packaging-md-0011-e7a23ade6bdc

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0011-e7a23ade6bdc" sha256="e7a23ade6bdc81024953ca3b87fa91691ffbe185a6eb4164e8ec5bd0689c3788" -->

### 已验证命令

```bash
cargo check -p xtask -p liora-packager
cargo test -p liora-packager
cargo run -p xtask -- package validate
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run
```


### prompt-p12-packaging-md-0012-1d3279301441

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0012-1d3279301441" sha256="1d3279301441f26dec076b1d2808dee362ed7eee6f66e184276a04a68067e2e7" -->

### GitHub preview runner 验证

- `27613242837` / commit `5a3615d`：`Package native Liora apps` workflow 成功。
- Linux/macOS/Windows matrix 均完成：release binary build、package generation、artifact smoke、raw binary upload、package artifact upload。
- Linux 真实生成路径已覆盖 AppImage、deb、rpm、portable tar.gz；macOS 覆盖 app/dmg；Windows preview 覆盖 NSIS。
- 修复过的 CI 反馈：RPM metadata-overwrite 分支、RPM TOML 嵌套结构、artifact smoke 忽略 `.cargo-packager` 内部工作目录。

Dry-run 预期生成：

```text
target/liora-packager/Packager.gallery.toml
target/liora-packager/Packager.docs.toml
target/liora-packager/GenerateRpm.gallery.toml
target/liora-packager/GenerateRpm.docs.toml
```


### prompt-p12-packaging-md-0013-ed4ae773d0f6

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0013-ed4ae773d0f6" sha256="ed4ae773d0f6ca5dc5d0093beedfe59b5edf7506774fa13376c9f38fcdfa697c" -->

## Historical Remaining Work / Completed Gate Mapping


### prompt-p12-packaging-md-0014-e65be3d49291

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0014-e65be3d49291" sha256="e65be3d49291da90016f9d60e1336d7a2ac9587fe9b5a02b9e4bc1804e4f28ce" -->

### 1. 真实后端 smoke 验证（最高优先级）

本地或 CI 安装：

```bash
cargo install cargo-packager --locked
cargo install cargo-generate-rpm --locked
cargo run -p xtask -- package ci --all-apps --format platform-defaults
```

需要验证真实产物：

- Linux：AppImage、deb、rpm、pacman/tar-like package。
- macOS：`.app`、`.dmg`。
- Windows：NSIS `.exe`、WiX MSI `.msi`。


### prompt-p12-packaging-md-0015-80dbbb76e580

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0015-80dbbb76e580" sha256="80dbbb76e580f4c12eeb5f5e89b5677bee46b8c5ce1bc7d94d975d83bd49d93a" -->

### 2. Linux runtime dependency metadata（已补）

已在生成配置中补齐 Linux 运行依赖：

- `.deb`：`[deb].depends` 包含 GTK3、Ayatana AppIndicator、X11、Wayland、xkbcommon、fontconfig/freetype、Vulkan、ALSA、xdg-utils。
- `.rpm`：`GenerateRpm.<app>.toml` 的 `[requires]` 包含对应 RPM 依赖，并继续保留 `auto-req = "builtin"` 和 `require-sh = false`。

后续只需在真实 Linux 发行版安装 smoke 后微调包名兼容性。


### prompt-p12-packaging-md-0016-6873346850d6

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0016-6873346850d6" sha256="6873346850d6a44e139b211e960dc88d83daf4651ab7364571e69f4f5c13d207" -->

### 3. 真正的 portable `.tar.gz` backend（已补）

`tar.gz` 不再映射为 cargo-packager `pacman`，而是 Liora supplemental backend：

- 收集 `target/release/<binary>`；
- 收集 PNG/SVG app icons；
- Linux 下收集 `.desktop` 与 metainfo；
- 生成顶层启动脚本 `./<binary>`；
- 生成 portable `README.md`；
- 使用系统 `tar -czf` 输出 `<package>-<version>-<platform>-<target-triple>.tar.gz`；
- 非 dry-run 会被 manifest/checksum/release-notes 扫描记录。


### prompt-p12-packaging-md-0017-95e9415f2228

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0017-95e9415f2228" sha256="95e9415f222802d6d61d00eb6ef9aed203303ebf6b960d399bb35b8a043f149f" -->

### 4. Signing / notarization

接入但不要硬编码 secrets：

- macOS：`codesign`、`notarytool`、`stapler`。
- Windows：`signtool`、timestamp server。
- CI secrets + unsigned fallback policy。


### prompt-p12-packaging-md-0018-96535359760e

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0018-96535359760e" sha256="96535359760ea65852cf45c9995a10a499c82c5c344c0e831c4403aba0926cd4" -->

### 5. GitHub Release automation（已接入 readiness gate）

基础能力已接入，并在打包前运行 release-readiness：`main` push 会生成 preview 包；`v*` tag package matrix 完成后，release job 会下载 `liora-release-packages-*` artifacts，自动收集上一个 tag 以来的 git commit changelog，并按 Conventional Commit 类型分组生成 release notes，随后创建/更新 GitHub Release 并上传全部构建产物。

后续增强：

- 增加 draft / prerelease 策略；
- 对上传产物做最终命名清洗；
- 汇总多平台 manifest/checksum 为顶层 release manifest；
- 在真实 CI 包产物验证后补 release smoke gate。


### prompt-p12-packaging-md-0019-24c09585983b

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0019-24c09585983b" sha256="24c09585983bd7ce1f23dbf1110a4a22b285a341a7e6d5b05d1e0967b3fd5839" -->

### 6. Install / uninstall smoke scripts

建议新增平台 smoke：

- deb：`dpkg -i`、启动 smoke、卸载。
- rpm：`rpm -i`、启动 smoke、卸载。
- AppImage：可执行 smoke。
- macOS / Windows：runner-safe 的有限 install/open checks。


### prompt-p12-packaging-md-0020-c5c6f1c892fc

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0020-c5c6f1c892fc" sha256="c5c6f1c892fcdefa5c1defffe0b338da599ee10878825ccdb8e49df1d1b6e3fe" -->

### 7. Artifact naming and metadata normalization（部分已补）

已完成：

- portable `.tar.gz` 命名为 `<package>-<version>-<platform>-<target-triple>.tar.gz`；
- `package-manifest.json` 增加 `targetTriple` 与 `gitSha`；
- `release-notes.md` 展示 version、target triple、git sha；
- checksum 继续覆盖全部已发现产物。

待真实 cargo-packager 后端 smoke 后再对 `.deb` / `.rpm` / `.dmg` / `.exe` / `.msi` 做最终重命名清洗。


### prompt-p12-packaging-md-0021-6b45629c03f2

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0021-6b45629c03f2" sha256="6b45629c03f21e885bbbc267ac9293d832282d151394ce4870b6c387f446d9f0" -->

### 8. License / metadata cleanup（已明确当前策略）

已新增 `LICENSE.md`，明确当前仓库和 package metadata 使用 `LicenseRef-Liora`，直到 owner 决定正式 OSS 或商业 license。RPM config 继续使用 `LicenseRef-Liora`，这是显式策略而非遗漏。


### prompt-p12-packaging-md-0022-d2793a696ebd

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0022-d2793a696ebd" sha256="d2793a696ebd1efd02c7bffa11a361bc4a6c9c9db2d8c989fb14df5135957e20" -->

### 9. CI real-run iteration

`.github/workflows/package.yml` 的 preview runner 已通过；release tag path 已补 `vX.Y.Z` + Cargo.toml version match validation。后续还需要真实 `v*` tag runner 验证 GitHub Release asset 上传、Windows MSI、签名/公证相关行为。预期会需要修：

- Linux AppImage dependencies/tools；
- Windows WiX/NSIS availability；
- macOS dmg/codesign behavior。


### prompt-p12-packaging-md-0023-6e444be63873

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0023-6e444be63873" sha256="6e444be638730cb1088ac8f6790e8c1bc25bbbe0e8fb643f02c1dc99a3e64f47" -->

## Guardrails for P12 Continuation

- 绝对不要把 Liora app 改成 Tauri。
- 保持应用为纯 Rust + GPUI native。
- packaging tools 可以使用，但 WebView / HTML / CSS / browser runtime 不能进入 app architecture。
- `xtask` 继续作为唯一公开打包入口。



### prompt-p12-packaging-md-0024-9959d11599e8

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0024-9959d11599e8" sha256="9959d11599e83d6786864ea43aa288d00a3ea70807f58843ab9c8cb2aae195d4" -->

## Install/uninstall smoke plan — 2026-06-17

已新增并修正 `cargo run -p xtask -- package install-smoke ...`：

- 默认 plan-only，复用已存在 artifact discovery + `package smoke`，输出每个产物的 install / launch-smoke / uninstall 命令。
- `--dry-run` 是真正的 runner-safe 计划模式：根据 app / platform / format 生成预期产物路径和 install/uninstall plan，不依赖真实后端产物存在，也不会误扫陈旧 `target/packages` artifact。
- 写入 `target/packages/install-smoke-plan.md`，便于 CI artifact 审计。
- `--execute-install` 仅允许 portable `.tar.gz` 做安全解压/验证/删除；系统级 deb/rpm/AppImage/macOS/Windows 安装仍保持计划输出，等待真实 runner policy、签名和人工 QA 后再放开。
- GitHub Actions `package.yml` 已在 artifact smoke 后加入 plan-only install/uninstall smoke gate。


### prompt-p12-packaging-md-0025-5dca4eb24979

<!-- ctx-migration source=".prompt/P12-packaging.md" unit="prompt-p12-packaging-md-0025-5dca4eb24979" sha256="5dca4eb24979e568ba1c979e32a3b6a5f3a3bd4b3357c84f6514dc72e9eb36d3" -->

## Final Closure — 2026-06-18

P12 is complete for this repository's controllable scope. Final closure added:

- `cargo run -p xtask -- package release-readiness`;
- `target/packages/release-readiness.md` report generation;
- `LICENSE.md` with explicit `LicenseRef-Liora` policy;
- `packaging/signing-policy.md` for macOS/Windows signing and notarization inputs;
- CI dry-run readiness check in `.github/workflows/ci.yml`;
- strict release readiness gate in `.github/workflows/package.yml` for `v*` tag releases;
- Docs updates in `apps/liora-docs/content/pages/packaging_workflow.md` and `docs/packaging-installer-technical-plan.md`.

The only actions not executed by the agent are intentionally external and credential-gated: provisioning signing credentials, approving protected release environments, running destructive system-level installers on dedicated machines, and creating a real public `vX.Y.Z` GitHub Release. The repository now blocks or documents those gates rather than silently treating them as unfinished implementation work.

### prompt-p13-component-expansion-md-0001-9c4dd89a8797

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0001-9c4dd89a8797" sha256="9c4dd89a87979a0705762733b5c9ee5db8255dc4d46d5f508525718720a1a8d5" -->

# P13 Component Expansion — Advanced Widgets & Customization

> 上游: `.prompt/P10-charts.md` / `.prompt/P12-packaging.md`
> 状态: Implemented / 收尾维护
> 目标: 补齐用户提出的新一批业务控件与既有控件增强，形成 Dashboard、低代码配置面板、数据监控、操作面板、内容编辑等场景的完整组件能力。


### prompt-p13-component-expansion-md-0002-ff9470ee5fac

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0002-ff9470ee5fac" sha256="ff9470ee5fac983c6afaf9652c17b91facfe01823efc91e86c14ebfbf2629a4a" -->

## 目标

P13 已完成主体实现，聚焦两类工作：

1. **新增控件**：只为当前组件库中不存在的能力新增文件/API，例如二维码、代码编辑器、信号图、热力图、分段比例条、水平可拖动列表、计时器、Label、Operation 等；“独立柱状图”按用户截图理解为 BarChart 的无坐标迷你柱样式，不新增平行控件。
2. **既有控件增强**：凡是已经存在的控件，必须直接在原组件、原 Demo、原 Docs 页面上增强；例如 RingChart、LineChart、BarChart、Progress/RingProgress、Button、Tag、Radio、Checkbox、Vertical/List 类控件，禁止为了单个增强点另建平行新控件。

所有组件必须保持 **Rust + GPUI 原生渲染**，遵守 Liora 组件 API 范式，不能引入 WebView/HTML/CSS/DOM/browser runtime。


### prompt-p13-component-expansion-md-0003-38583f0c6a25

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0003-38583f0c6a25" sha256="38583f0c6a2502373152dabc7bdde2c6023561e70714e77f5fc7f090427a51e7" -->

## 绝对边界

- 禁止引入 Web 编辑器运行时（Monaco/CodeMirror WebView 等）。
- 禁止用网页二维码、SVG DOM、Canvas DOM 方案替代 GPUI 原生节点。
- QR 生成/识别可使用纯 Rust 算法库；识别需要作为可选能力或清晰隔离依赖，避免把图像解码链污染到基础控件路径。
- 代码编辑器第一阶段允许采用 `syntect` 高亮；语法检查必须设计成可插拔 diagnostics provider，不在 P13 MVP 中硬绑定 LSP。
- 图表/进度增强必须复用 P10 chart primitive，避免每个控件重复绘制基础设施。
- 交互组件拖动必须明确数据模型、拖动状态、drop 回调、无障碍 fallback，不能只做视觉移动。
- **已有控件增强优先原则**：如果能力属于已有控件（如 `Tag` flow、`Progress` 环形渐变、`LineChart` 线型、`BarChart` 区间色/独立迷你柱样式、`RingChart` 外置文本、`Button` 自定义颜色、`Radio`/`Checkbox` option 自定义），必须修改已有组件文件、已有 demo 和已有 docs/snippets；不得新增 `TagFlow`、`RingProgress2`、`AdvancedButton`、`FlatBarMeter` 等替代控件。


### prompt-p13-component-expansion-md-0004-41004d705a0d

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0004-41004d705a0d" sha256="41004d705a0d9e6277d8184255725235a833f5e0540bf9a07c8d18aacf36e141" -->

## 组件清单与需求拆解

| # | Component / Enhancement | 类型 | 核心需求 | 优先级 |
|---|---|---|---|---|
| 1 | `QrCode` | 新增 | 二维码生成、识别、尺寸/纠错等级/颜色/Logo/复制/导出 | P0 |
| 2 | `CodeEditor` | 新增 | 行号、缩进、高亮、选择/复制、编辑、diagnostics 扩展点 | P0 |
| 3 | `SignalMeter` | 新增 | 手机信号/WiFi 风格、等级、每级颜色、禁用/空状态 | P0 |
| 4 | `HeatBar` / `HeatmapBar` | 新增 | 按截图实现时间轴柱状热力图：细圆角竖柱、按 severity/category 或 value range 渐变映射颜色、顶部 legend 汇总、可选 y 轴刻度/时间 x 轴 label、tooltip | P0 |
| 5 | `BarChart` standalone mini mode | 增强 | 按截图实现无坐标/无网格/无 legend 的独立迷你柱状样式：窄圆角竖柱、淡入/渐变配色、紧凑高度、可嵌入卡片；直接扩展现有 `BarChart`，不新增 `FlatBarMeter` | P0 |
| 6 | `SegmentRatioBar` | 新增 | 按截图实现一条横向分段比例条 + 可配置位置的 legend/value 文本行：文本可在上方、下方、上下同时或隐藏；每段颜色、圆点、label、比例值 pattern 可自定义，支持 label 与比例值在每个 legend item 两端对齐 | P0 |
| 7 | `HorizontalList` | 新增 | 横向滚动、item 完全自定义、divider 自定义、item 拖动 | P1 |
| 8 | Vertical list drag | 增强 | 既有列表/VirtualizedList 增加垂直 item 拖动 | P1 |
| 9 | `RingChart` external labels | 增强 | 图例 + 比例值完全外置，垂直/水平排列，不需要折线引导 | P1 |
| 10 | `LineChart` line style | 增强 | 每条线自定义虚线/实线/点线、颜色、粗细、点样式 | P1 |
| 11 | `BarChart` range colors | 增强 | 按指标值区间映射颜色，支持默认和 per-series override | P1 |
| 12 | `RingProgress` gradient | 增强 | 进度色渐变、完成色自定义、默认取渐变末色 | P1 |
| 13 | `Timer` | 新增 | 时间单位、正计时/倒计时、开始/暂停/重置、按单位获取结果 | P1 |
| 14 | `Button` gradient/custom color | 增强 | 渐变色、完全自定义颜色、自动推导 hover/pressed/disabled | P1 |
| 15 | `Tag` flow layout | 增强 | 标签流式布局、自动换行、gap、max rows/collapse | P2 |
| 16 | `Label` | 新增 | Icon + Text，间距、位置、尺寸、颜色可配 | P2 |
| 17 | `Operation` | 新增 | 左侧 Label + 右侧操作区域，两端对齐，操作区自定义 | P2 |
| 18 | `Radio` / `Checkbox` option customization | 增强 | option 布局/样式完全自定义，选中态布局/样式可自定义 | P2 |


### prompt-p13-component-expansion-md-0005-6f59c1df98a6

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0005-6f59c1df98a6" sha256="6f59c1df98a65d57e212eee821b6bce148fb1af9096b0c509571c6e642ab0217" -->

## 推荐实施分批


### prompt-p13-component-expansion-md-0006-f536f9e0ac5a

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0006-f536f9e0ac5a" sha256="f536f9e0ac5a6e9ca7becf80f22d2ac56f7309131f02359875d8bbf5293eff8b" -->

### Wave 1 — 基础视觉与轻交互控件

目标：先交付可快速复用、风险低的新增控件，并把已有 `Tag` 的 flow 能力合并进原 Tag 组件文档/Demo。

- `SignalMeter`
- `BarChart` standalone mini mode（按截图：无坐标紧凑圆角竖柱，直接增强现有 BarChart）
- `HeatBar`
- `SegmentRatioBar`
- `Label`
- `Operation`
- `Tag` flow layout

验收：

- 新增控件导出 API、Gallery demo、Docs 页面、snippet；已有控件增强直接补充原 demo/docs/snippet。
- 样式参数覆盖颜色、尺寸、间距、label pattern。
- `cargo check -p liora-components -p liora-gallery -p liora-docs` 通过。


### prompt-p13-component-expansion-md-0007-f53942eb79ed

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0007-f53942eb79ed" sha256="f53942eb79ed5a64f0fb908234cea71549ca1d0c92d15fecfbbfeba8302c023f" -->

### Wave 2 — 图表与进度增强

目标：复用 P10/P4 图表进度能力，补齐用户明确指出的高度自定义。

- `RingChart` 外置 legend/value pattern
- `LineChart` per-series stroke style
- `BarChart` standalone mini mode + value range color rules
- `RingProgress` gradient + completion color

验收：

- 共享 chart style model，不在各图表中重复定义不兼容配置。
- Docs/Demo 在原控件页面展示每个新增配置项，不另起平行控件页。
- 单元测试覆盖 style resolution、range color matching、legend layout 数据结构。


### prompt-p13-component-expansion-md-0008-e46899134d57

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0008-e46899134d57" sha256="e46899134d57620b81bdfe0613ce2199bf71315ff54ab8bd7beadee2e8efbf2c" -->

### Wave 3 — 拖动列表与布局容器

目标：提供横向/纵向可拖动列表能力，服务配置项、步骤流、标签流、看板排序等场景。

- `HorizontalList`
- 既有垂直列表 / `VirtualizedList` item drag enhancement
- divider 自定义：默认垂直线段，也支持 icon/arrow/自定义 element

验收：

- 拖动排序回调提供 `from_index` / `to_index` / item key。
- 支持禁用拖动、固定项、拖动占位样式。
- 对虚拟列表避免保存跨 frame 的 `AnyElement` / `ArenaRef`，item 必须由闭包重新渲染。


### prompt-p13-component-expansion-md-0009-1a04084da820

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0009-1a04084da820" sha256="1a04084da820971f82855f25d2504bf8239fae15ac7f44ac5ba72106dd500496" -->

### Wave 4 — QR 与 CodeEditor

目标：处理依赖和复杂交互风险最高的两个控件。

- `QrCode`：生成与识别。
- `CodeEditor`：编辑器基础能力。

验收：

- QR 生成支持纯数据结构测试；识别能力先以静态图片/bytes API 测试，不要求摄像头。
- CodeEditor 支持：行号、缩进、Tab/Shift+Tab、语法高亮、选择复制、基础 diagnostics 渲染。
- 语法检查只定义 provider trait 与 diagnostics 数据模型，不硬编码 Rust analyzer/LSP。


### prompt-p13-component-expansion-md-0010-abe123377896

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0010-abe123377896" sha256="abe1233778960fd08239e8ce68ab68448b60fdf5650b17e66046b4e2d014d169" -->

### Wave 5 — 表单控件深度自定义

目标：增强 Button/Radio/Checkbox 的企业级主题与低代码可配置能力。

- `Button` gradient/custom palette + derived states。
- `Radio` / `Checkbox` option render customization。
- 与 `Label` / `Operation` 组合展示复杂 option 内容。

验收：

- 自定义颜色模式下 hover/pressed/disabled 由统一 color resolver 推导。
- Radio/Checkbox 可通过 builder 设置 option render/style hooks，同时保留普通用法。
- Demo 展示默认、卡片式、图标式、左右结构、选中态替换布局。


### prompt-p13-component-expansion-md-0011-b9acba5a96de

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0011-b9acba5a96de" sha256="b9acba5a96de2bf31d378976c045058af33a6e68c7ce5740e7553a4bb60987b7" -->

## API 设计草案


### prompt-p13-component-expansion-md-0012-f77784c9357c

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0012-f77784c9357c" sha256="f77784c9357c79eb7fb3754f103548e29b28faeb71431eb264aa014cc71d5ae5" -->

### QrCode

```rust
QrCode::new("https://github.com/yhyzgn/liora")
    .size(px(180.0))
    .error_correction(QrErrorCorrection::Medium)
    .foreground(theme.primary.base)
    .background(theme.background)
    .logo(Icon::new(IconName::Sparkles))
    .copyable(true)
```

识别 API 建议拆为非 UI helper：

```rust
let result = QrDecoder::decode_image_bytes(bytes)?;
```


### prompt-p13-component-expansion-md-0013-b03c56d70215

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0013-b03c56d70215" sha256="b03c56d702155a9313d0b8e8c1f654e9d18addd81464b241d1e44861da040e67" -->

### CodeEditor

```rust
CodeEditor::new(source)
    .language(CodeLanguage::Rust)
    .line_numbers(true)
    .tab_size(4)
    .soft_tabs(true)
    .diagnostics(diagnostics)
    .on_change(|text, window, cx| { /* ... */ })
```


### prompt-p13-component-expansion-md-0014-b26148fbf3b1

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0014-b26148fbf3b1" sha256="b26148fbf3b1d08f2b4062a371cab68ed205794a2be0f25a66e2d816bcbed00c" -->

### SignalMeter

```rust
SignalMeter::new(3, 5)
    .kind(SignalKind::Wifi)
    .level_colors(vec![danger, warning, success])
    .bar_gap(px(3.0))
```


### prompt-p13-component-expansion-md-0015-5f91bb52d33c

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0015-5f91bb52d33c" sha256="5f91bb52d33c209ceb01ede69f52b9605ea4a7b60b1cbcbaea6785e186c1d86b" -->

### BarChart standalone mini mode

```rust
BarChart::new(series)
    .standalone()
    .show_axis(false)
    .show_grid(false)
    .show_legend(false)
    .bar_radius(px(4.0))
    .bar_width(px(5.0))
    .bar_gap(px(8.0))
    .value_color_ranges(vec![
        BarValueColorRange::up_to(20.0, theme.success.soft),
        BarValueColorRange::above(20.0, theme.success.base),
    ])
```

用户截图语义：一组轻量迷你竖向圆角柱，没有横竖坐标、没有边框、没有 legend，视觉上可像信号/频谱，但本质仍是 BarChart 的一个展示模式。


### prompt-p13-component-expansion-md-0016-428559d76f0d

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0016-428559d76f0d" sha256="428559d76f0dc25fde6f98856ef1da66c44c51d8281e1b68adefd8e147ef8f9c" -->

### HeatBar / HeatmapBar

```rust
HeatBar::new(points)
    .x_labels(HeatAxisLabels::time())
    .y_ticks(vec![0.0, 5.0, 10.0])
    .legend(vec![
        HeatLegendItem::new("错误", 3).color(theme.danger.base),
        HeatLegendItem::new("警告", 24).color(theme.warning.base),
    ])
    .color_ranges(vec![
        HeatColorRange::new(0.0..=3.0, theme.warning.soft),
        HeatColorRange::new(3.0..=7.0, theme.warning.base),
        HeatColorRange::new(7.0..=10.0, theme.danger.base),
    ])
    .bar_width(px(4.0))
    .bar_radius(px(2.0))
```

用户截图语义：不是日历网格热力图，而是按时间分布的柱状热力图。顶部显示分类 legend 与数量汇总；主体是密集细柱，柱色按类别或数值区间从浅色到高亮色映射；可带轻量 y 轴刻度与 x 轴时间标签。


### prompt-p13-component-expansion-md-0017-fa6ab6cc3496

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0017-fa6ab6cc3496" sha256="fa6ab6cc3496614935d14b3e9ffd496ad3d6faff48808467686bf8c8f4526b1e" -->

### SegmentRatioBar

```rust
SegmentRatioBar::new(vec![
    Segment::new("Direct", 42.0).color(blue).value_pattern("{percent:.0}%"),
    Segment::new("Proxy", 51.0).color(green).value_pattern("{percent:.0}%"),
    Segment::new("Reject", 7.0).color(red).value_pattern("{percent:.0}%"),
])
.bar_height(px(7.0))
.bar_radius(px(4.0))
.legend_layout(SegmentLegendLayout::Inline)
.legend_position(SegmentLegendPosition::Bottom)
.label_align(SegmentLabelAlign::SplitEnds)
```

用户截图语义：默认可表现为上方一条横向分段比例条、下方 legend/value 信息行；但文本位置必须可配置，支持 `Top`、`Bottom`、`Both`、`Hidden`。每段宽度按占比计算、颜色独立；legend/value 通常为彩色圆点 + label + 百分比。每个 legend item 内 label 与比例值需要可分开两端对齐，也要支持自定义 pattern，如 `{label}`、`{percent:.1}%`、`{value}/{total}`。



### prompt-p13-component-expansion-md-0018-b003f0725a22

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0018-b003f0725a22" sha256="b003f0725a22e2a44cd4699734136d5e58bc38cd2649ba0b12063ff0eedcf054" -->

### HorizontalList

```rust
HorizontalList::new(items)
    .item(|item| item.render())
    .divider(|| Icon::new(IconName::ChevronRight))
    .draggable(true)
    .on_reorder(|from, to, cx| { /* update model */ })
```


### prompt-p13-component-expansion-md-0019-742e92ad3fce

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0019-742e92ad3fce" sha256="742e92ad3fcea94b834fd1b2cd532723145a2c5f59d2aa8e823a34f5747fb020" -->

### Timer

```rust
Timer::count_up()
    .unit(TimeUnit::Second)
    .precision(2)
    .on_tick(|elapsed, cx| {})

Timer::count_down(Duration::from_secs(300))
    .on_finish(|cx| {})
```


### prompt-p13-component-expansion-md-0020-5470cea81f60

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0020-5470cea81f60" sha256="5470cea81f60b0aefe10f02f5fa0504e4fd774ac3e4ae6d0597f8d57b5777020" -->

## 依赖调研清单

实现过程中按需完成轻量 dependency review，后续新增依赖仍需遵守：

| 能力 | 候选 | 关注点 |
|---|---|---|
| QR 生成 | `qrcode`, `fast_qr` | 纯 Rust、许可、SVG 依赖可关闭、image 输出能力 |
| QR 识别 | `rqrr`, `quircs` | 纯 Rust/FFI、image crate 兼容、识别率、维护状态 |
| 高亮 | 已有 `syntect` / `two-face` | 复用 CodeBlock 高亮资源，避免重复依赖 |
| 语法检查 | provider trait / 后续 LSP | P13 只做扩展点，不强绑定外部进程 |
| 拖拽 | GPUI mouse event + state | 不引入平台 DnD 作为 item reorder 前提 |

如需引入新依赖，必须先在 phase 执行记录中说明理由、替代方案和许可证风险。


### prompt-p13-component-expansion-md-0021-8d9434728f3f

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0021-8d9434728f3f" sha256="8d9434728f3fc48ce279c60e8b5c7ab052b8083c3c18584961741b49fdb99c49" -->

## Demo / Docs 要求

每个新增或增强项都必须同步；其中已有控件增强必须改原文件/原页面：

- 新增控件：`crates/liora-components/src/<component>.rs`，并在 `lib.rs` 导出。
- 已有控件增强：直接修改现有文件，例如 `tag.rs`、`progress.rs`、`line_chart.rs`、`bar_chart.rs`、`button.rs`、`radio*.rs`、`checkbox*.rs`。
- 新增控件：创建 `apps/liora-gallery/src/demos/<component>_demo.rs` 并注册。
- 已有控件增强：补充现有 `<component>_demo.rs`，不新增平行 demo。
- 新增控件：创建 `apps/liora-docs/content/pages/<component>.md` 与 snippets。
- 已有控件增强：补充现有 `content/pages/<component>.md` 与原 snippets 目录。

Docs 页面继续保持：**一种效果 → 对应代码 → 下一种效果 → 对应代码**。已有控件增强必须追加到原控件页面中的对应效果段落。


### prompt-p13-component-expansion-md-0022-adcef0ad8e4a

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0022-adcef0ad8e4a" sha256="adcef0ad8e4a2286b43e569ac5c74e2b80b8716a90c46accb4eb51dbcf6e1537" -->

## 测试要求

- Builder 状态测试：所有新增配置项都可被断言。
- 纯计算逻辑测试：比例、颜色区间、渐变 stop、timer 单位换算、drag reorder 结果。
- 边界测试：空数据、单 item、NaN/Infinity、负值、总数为 0、超出等级范围。
- 文档 snippet 必须能被 `cargo check`/现有 snippet checker 覆盖。
- 复杂交互至少用可测试的 reducer/helper 把状态转换与 GPUI event 分离。


### prompt-p13-component-expansion-md-0023-db09257a74e3

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0023-db09257a74e3" sha256="db09257a74e3f88813463e6c6bd97a5dfb6642f76186392836a02e6aa7be1c61" -->

## 完成标准

- [x] Wave 1 完成并通过 check/test。
- [x] Wave 2 完成并通过 check/test。
- [x] Wave 3 完成并通过 check/test。
- [x] Wave 4 完成并通过 check/test。
- [x] Wave 5 完成并通过 check/test。
- [x] Gallery demos 完整、布局不拥挤。
- [x] 新增控件有独立页面；已有控件增强只补充原页面章节。
- [x] `.memory/inventory.md`、`.memory/state.md`、`prompt.md` 更新。
- [x] 如新增依赖，`Cargo.lock`、许可证说明、依赖取舍记录完成。



### prompt-p13-component-expansion-md-0024-b679d0c99a76

<!-- ctx-migration source=".prompt/P13-component-expansion.md" unit="prompt-p13-component-expansion-md-0024-b679d0c99a76" sha256="b679d0c99a760a841ceaed5cfdabcb4ff79859bc974bb04021e3d1541caf7e1a" -->

## 2026-06-16 实现状态快照

P13 主体功能已按当前计划落地：新增控件和既有控件增强均已接入 `liora-components`、Gallery、Docs/snippets，并同步 `.memory/inventory.md`。后续如继续扩展，应按本文件的边界和测试要求作为维护规约执行，而不是重新创建平行控件。

已验证的收尾命令包括：

```bash
cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets
```

### prompt-p14-deferred-advanced-md-0001-3c1d5a125cd1

<!-- ctx-migration source=".prompt/P14-deferred-advanced.md" unit="prompt-p14-deferred-advanced-md-0001-3c1d5a125cd1" sha256="3c1d5a125cd166b02a0f593d03271f4590caef39957a9edeb97a3f216474fa6b" -->

# P14 Deferred Advanced — Advanced Component Backlog Completion

> 上游: `.prompt/P9-deferred-advanced.md` / `.prompt/P13-component-expansion.md`
> 状态: Complete
> 目标: 将 P9 deferred backlog 正式转为可交付阶段，逐步补齐 Carousel、Calendar、TreeSelect、InputTag、Mention、Watermark、Tour、VirtualizedTable、VirtualizedTree。


### prompt-p14-deferred-advanced-md-0002-3e3284c42191

<!-- ctx-migration source=".prompt/P14-deferred-advanced.md" unit="prompt-p14-deferred-advanced-md-0002-3e3284c42191" sha256="3e3284c42191996d4f113a63893d79d42bf053e16934e336ce9e44f60322bc57" -->

## 背景

P9 原本是 P5 跳过的高级组件暂存 backlog。用户要求在 P13/P12 完成后自动进入下一个 P，因此本阶段将 P9 的 deferred 条目迁移为 P14 执行阶段，避免继续停留在“等待明确要求”的状态。


### prompt-p14-deferred-advanced-md-0003-7964d43cf870

<!-- ctx-migration source=".prompt/P14-deferred-advanced.md" unit="prompt-p14-deferred-advanced-md-0003-7964d43cf870" sha256="7964d43cf870fde6ec76797afeb1b0b963c166b2df802e69226f2a035a9ec5f4" -->

## 执行原则

- 继续保持 **纯 Rust + GPUI native**；禁止 WebView/HTML/CSS/DOM/browser runtime。
- 每个组件都必须包含：`liora-components` API、Gallery demo、Docs 页面、compile-checked snippets、关键测试。
- 优先复用现有组件：Input、TagFlow、Tree、Table、VirtualizedList、Popover/Portal、Button、Icon。
- 复杂交互先交付真实可用 MVP，再扩展高级配置；不能提交空壳/占位组件。


### prompt-p14-deferred-advanced-md-0004-7ded7b7e02de

<!-- ctx-migration source=".prompt/P14-deferred-advanced.md" unit="prompt-p14-deferred-advanced-md-0004-7ded7b7e02de" sha256="7ded7b7e02de2724b9262bc3ac9ed87b1748a5ef7e8e4d1bac65d6b4f6e6cd13" -->

## 组件清单

| # | Component | 状态 | 交付要求 |
|---|-----------|------|----------|
| 1 | Carousel | ✅ Wave 1 | 轮播项、方向、指示器位置、箭头、自动播放配置、自定义内容 slot |
| 2 | Calendar | ✅ Wave 1 | 月视图、选中日期、范围、禁用日期、事件标记、选择回调 |
| 3 | InputTag | ✅ Wave 1 | 输入生成标签、删除、限制数量、重复策略、TagFlow 展示 |
| 4 | TreeSelect | ✅ Wave 3 | 树形选择、单选/多选、过滤、禁用节点、默认值 |
| 5 | Mention | ✅ Wave 2 | 触发符、建议列表、过滤、选择回调、禁用状态 |
| 6 | Watermark | ✅ Wave 2 | 文字水印、密度、间距、透明度、颜色、页眉/页脚局部位置 |
| 7 | Tour | ✅ Wave 4 | 步骤、target 描述、placement、进度、下一步/上一步、关闭/完成回调 |
| 8 | VirtualizedTable | ✅ Wave 5 | 大数据行虚拟化、固定表头、TableColumn API 对齐、排序状态回调 |
| 9 | VirtualizedTree | ✅ Wave 6 | 大树虚拟化、展开/折叠、单选/多选、checkbox、滚动性能 |


### prompt-p14-deferred-advanced-md-0005-76a6c42391ac

<!-- ctx-migration source=".prompt/P14-deferred-advanced.md" unit="prompt-p14-deferred-advanced-md-0005-76a6c42391ac" sha256="76a6c42391ac1575ad85ac62934d0ae41c55d447a45dc83159076fb8decb0933" -->

## Wave 1 — 2026-06-16

已实现：

- `Carousel`: `CarouselItem` + `Carousel` builder API，支持方向、指示器位置、箭头开关、autoplay/interval/pause 配置、自定义内容 slot。
- `Calendar`: `CalendarDate`、`CalendarEvent`、月视图 42 单元格、选中/范围/禁用/事件标记、选择回调。
- `InputTag`: `Input` + `TagFlow` 组合控件，支持回车添加、删除、最大数量、重复项策略、on_change 回调。
- Gallery: 新增 `carousel_demo.rs`、`calendar_demo.rs`、`input_tag_demo.rs` 并注册。
- Docs: 新增 `carousel.md`、`calendar.md`、`input_tag.md` 与 snippets，并接入 `check_snippets`。


### prompt-p14-deferred-advanced-md-0006-d5800439b44d

<!-- ctx-migration source=".prompt/P14-deferred-advanced.md" unit="prompt-p14-deferred-advanced-md-0006-d5800439b44d" sha256="d5800439b44d096dcd962bdc78074669631cfe2dff47602aaa33beee8e17b1a5" -->

## 后续维护

P14 已完成。后续只在出现新的用户需求或缺陷报告时维护这些控件；不要把 P9 backlog 重新视为未完成。


### prompt-p14-deferred-advanced-md-0007-eb00a1ab6aa2

<!-- ctx-migration source=".prompt/P14-deferred-advanced.md" unit="prompt-p14-deferred-advanced-md-0007-eb00a1ab6aa2" sha256="eb00a1ab6aa26bbe723cb8bbeb532dde8a777dab7d9e66399b5c2e6e3667ae88" -->

## 验收命令

```bash
cargo check -p liora-components -p liora-gallery -p liora-docs --bin check_snippets
cargo test -p liora-components carousel && cargo test -p liora-components calendar && cargo test -p liora-components input_tag && cargo test -p liora-components mention && cargo test -p liora-components watermark && cargo test -p liora-components tree_select && cargo test -p liora-components tour
cargo test -p liora-gallery registry_entries_are_sorted_with_charts_grouped_last
cargo run -p liora-gallery
cargo run -p liora-docs
```


### prompt-p14-deferred-advanced-md-0008-80fc9e9a952a

<!-- ctx-migration source=".prompt/P14-deferred-advanced.md" unit="prompt-p14-deferred-advanced-md-0008-80fc9e9a952a" sha256="80fc9e9a952a1ea676cb6b36b685c4dc037f5e26281d92666d527f684de3c088" -->

## Wave 2 — 2026-06-16

已实现：

- `Mention`: 复用 `Input` 作为输入内核，支持触发符、候选过滤、最大候选数量、禁用状态和选择回调。
- `Watermark`: 支持覆盖/页眉/页脚位置、文字内容、密度、间距、透明度、颜色和旋转配置记录。
- Gallery: 新增 `mention_demo.rs`、`watermark_demo.rs` 并注册。
- Docs: 新增 `mention.md`、`watermark.md` 与 snippets，并接入 `check_snippets`。

剩余 P14：TreeSelect、Tour、VirtualizedTable、VirtualizedTree。


### prompt-p14-deferred-advanced-md-0009-5c20ea0cd328

<!-- ctx-migration source=".prompt/P14-deferred-advanced.md" unit="prompt-p14-deferred-advanced-md-0009-5c20ea0cd328" sha256="5c20ea0cd328c368e311c0d3c8373483be89c391189468d5bf1ddc4d0f0ae516" -->

## Wave 3 — 2026-06-16

已实现：

- `TreeSelect`: 新增树形选择控件，支持单选、多选、默认选中、禁用节点、filterable 搜索过滤、选择回调。
- Gallery: 新增 `tree_select_demo.rs` 并注册。
- Docs: 新增 `tree_select.md` 与 single/multiple/filterable snippets，并接入 `check_snippets`。

剩余 P14：Tour、VirtualizedTable、VirtualizedTree。


### prompt-p14-deferred-advanced-md-0010-4814c490b868

<!-- ctx-migration source=".prompt/P14-deferred-advanced.md" unit="prompt-p14-deferred-advanced-md-0010-4814c490b868" sha256="4814c490b868c11bda38fe34c9efa9ffaac31f03593f01918ef19046018ccc37" -->

## Wave 4 — 2026-06-17

已实现：

- `Tour`: 新增受控步骤引导组件，支持 step list、active_index、target 描述、placement、mask/progress 开关、上一页/下一页/完成/关闭回调。
- Gallery: 新增 `tour_demo.rs` 并注册。
- Docs: 新增 `tour.md` 与 basic/middle/no_mask snippets，并接入 `check_snippets`。

剩余 P14：无。



### prompt-p14-deferred-advanced-md-0011-c2102c96dc15

<!-- ctx-migration source=".prompt/P14-deferred-advanced.md" unit="prompt-p14-deferred-advanced-md-0011-c2102c96dc15" sha256="c2102c96dc15a6276364c67a98f2c1b2585366dfcfcdcaefbca9533becf47c1c" -->

## Wave 5 — 2026-06-17

已实现：

- `VirtualizedTable`: 新增大数据虚拟表格，支持 `TableColumn` 列定义、固定表头、`ListState` 可见区行渲染、Liora `VirtualScrollbar`、高度/行高/overdraw、斑马纹/边框/加载/空状态、三态排序回调。
- Gallery: 新增 `virtualized_table_demo.rs` 并注册基础万行表格与排序状态用例。
- Docs: 新增 `virtualized_table.md` 与 basic/sortable snippets，并接入 `check_snippets`。

剩余 P14：无。



### prompt-p14-deferred-advanced-md-0012-51b7916b75a5

<!-- ctx-migration source=".prompt/P14-deferred-advanced.md" unit="prompt-p14-deferred-advanced-md-0012-51b7916b75a5" sha256="51b7916b75a5b7a345a8bf69c2991bd2b766bf48d6d5bcfcef505c83c160cc1a" -->

## Wave 6 — 2026-06-17

已实现：

- `VirtualizedTree`: 新增大型树虚拟化控件，维护原始 `TreeNode`、展开 key、选择 key 和轻量可见节点元数据，使用 `ListState` 只渲染可见行，支持默认展开/选中、展开/折叠、单选/多选、checkbox 风格、on_node_click 回调、高度/行高/缩进/overdraw 配置和 Liora 滚动条。
- Gallery: 新增 `virtualized_tree_demo.rs` 并注册大型组织树与多选回调用例。
- Docs: 新增 `virtualized_tree.md` 与 basic/checkable snippets，并接入 `check_snippets`。

P14 deferred advanced backlog 已全部完成：Carousel、Calendar、InputTag、Mention、Watermark、TreeSelect、Tour、VirtualizedTable、VirtualizedTree。

### prompt-p15-quality-hardening-md-0001-cdc8b968b068

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0001-cdc8b968b068" sha256="cdc8b968b0682ae48e01e8d10f2b06d79a02b93df8bc2ca54e4e17086232531b" -->

# P15 — Quality Hardening

> 上游: `.prompt/P13-component-expansion.md` / `.prompt/P14-deferred-advanced.md` / `.prompt/P12-packaging.md`
> 状态: Complete
> 目标: 在组件功能补齐后，进入发布前质量收口阶段，系统性提升 Liora 的可维护性、一致性、性能、文档完整性和 CI 防回归能力。


### prompt-p15-quality-hardening-md-0002-82fc5e45a89a

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0002-82fc5e45a89a" sha256="82fc5e45a89a7601d69bad32c1ff895537c4bc2fe2d4782553b2826333511e9b" -->

## 背景

P13 高级控件扩展和 P14 deferred backlog 已完成，P12 本地 runner-safe packaging readiness 已完成但仍受签名、公证、真实系统安装、license policy、真实 release tag 等外部策略约束。P15 不继续盲目堆新控件，而是把现有组件库从“功能可用”推进到“长期可维护、可发布、可被外部项目放心依赖”。


### prompt-p15-quality-hardening-md-0003-cbdc4fd0a10e

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0003-cbdc4fd0a10e" sha256="cbdc4fd0a10e5fd0f8840348e6fbc001b13171c3f8ab62d8d14260c2fdcbcf59" -->

## 非目标 / 边界

- 不引入 WebView、HTML/CSS/DOM/browser runtime；Liora 继续保持纯 Rust + GPUI native。
- 不把 P12 外部策略项伪装成本地完成项；签名、公证、真实系统安装卸载和 license 仍需 owner policy。
- 不新增平行替代控件来掩盖已有控件问题；优先修原组件、原 demo、原 docs。
- 不用减少 demo 数据量替代真实性能优化；性能项必须基于 profiling 或可复现的基准/测试证据。


### prompt-p15-quality-hardening-md-0004-590fafa8089f

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0004-590fafa8089f" sha256="590fafa8089f00b929dc5c86beb0106963d59fba0e04a1668e647e384e21f7b3" -->

## 工作流

每个 hardening slice 都必须包含：

1. 明确审计目标和验收证据。
2. 小范围、可回滚的代码/文档/CI 改动。
3. 运行对应验证命令。
4. 更新 `.memory/state.md` / `.memory/sessions.md` / 本文件进度。
5. commit + push。


### prompt-p15-quality-hardening-md-0005-8d0d637dc18d

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0005-8d0d637dc18d" sha256="8d0d637dc18d92a3f6fab63bb3557c5691b288161b0743dbc368ec1d581ad994" -->

## Track A — CI / Verification Gates

目标：让每次普通提交都能自动验证核心质量，而不是只依赖 packaging workflow。

- [x] 新增通用 CI workflow：fmt、workspace check/test、docs snippet check、packaging validate、packaging dry-run、install-smoke dry-run。
- [x] 评估并拆分 Linux GUI workspace 质量 job 与 lightweight packaging dry-run job。
- [x] 将 release/package workflow 与 CI workflow 的职责边界写入 docs。


### prompt-p15-quality-hardening-md-0006-0a662731746a

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0006-0a662731746a" sha256="0a662731746ac728d357624d3f4e56672f51463d500012739fa421b7e63e6a1b" -->

## Track B — API Consistency Audit

目标：统一组件 builder、事件回调和状态命名。

审计重点：

- `on_change` / `on_select` / `on_click` / `on_close` / `on_finish` 签名一致性。
- `disabled(...)`、`size(...)`、`variant(...)`、`open(...)`、`default_*` builder 命名一致性。
- P13/P14 新增控件是否遵守已有 Liora 组件 API 范式。
- 避免生产路径中不必要的 `unwrap()` / `expect()` / `panic!()`。


### prompt-p15-quality-hardening-md-0007-f23a3ce3b531

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0007-f23a3ce3b531" sha256="f23a3ce3b531b30fb90ee7fb1d5217307f8cfac251cbc127145fc6b1733f6a84" -->

## Track C — Visual / Theme Consistency

目标：新增控件和历史控件在 light/dark/theme token 下保持一致。

审计重点：

- spacing / radius / border / shadow token 使用。
- disabled / hover / active / selected 状态。
- Button、Tag、Radio、Checkbox、Chart、Progress、Tour、Virtualized* 等 P13/P14 控件视觉一致性。


### prompt-p15-quality-hardening-md-0008-9ee4c43dcbf3

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0008-9ee4c43dcbf3" sha256="9ee4c43dcbf32b2c178fcbcb417f66f949b5216eb76a0a2f81d4d349d71fe5c0" -->

## Track D — Interaction / Keyboard / Overlay Behavior

目标：减少交互类控件的边界问题。

审计重点：

- ESC 关闭能力：Modal、Drawer、Popover、Dropdown、Select、Tour 等。
- Tab / Enter / Space 基础键盘操作。
- 点击外部关闭、焦点释放、选区取消、拖拽释放等状态清理。
- overlay 层级与 Portal/Modal/Drawer/Tooltip 的 z-index 关系。


### prompt-p15-quality-hardening-md-0009-96aa9c46f017

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0009-96aa9c46f017" sha256="96aa9c46f0177336e7eaef18335c414716cc96e68d14c5a56aacb675c083e923" -->

## Track E — Performance Hardening

目标：用证据驱动优化，而不是降低示例规模。

审计重点：

- CodeBlock / CodeEditor 高亮与选区性能。
- Line/Area/Sparkline 大数据降采样后的剩余热点。
- VirtualizedList / VirtualizedTable / VirtualizedTree 滚动和拖拽。
- Docs QuickStart 等长页面渲染/滚动性能。


### prompt-p15-quality-hardening-md-0010-1601fd847ea9

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0010-1601fd847ea9" sha256="1601fd847ea94ab8234db5d38534302e08d917d625235c994b66330c74ff41e6" -->

## Track F — Docs Completeness

目标：每个可公开组件都具备足够文档和可编译代码片段。

审计重点：

- 每个组件是否有页面、效果、对应 snippet。
- snippets 是否是完整 Rust 文件并由 `check_snippets` 覆盖。
- Gallery 与 Docs 示例是否同步。
- P12 打包流程、P15 质量门禁是否在 docs 中有清晰入口。


### prompt-p15-quality-hardening-md-0011-ed293e41f887

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0011-ed293e41f887" sha256="ed293e41f887ed09eb3a5da0033f5452a96a0b0a130db7b645baffdf25c5f190" -->

## P15 Progress


### prompt-p15-quality-hardening-md-0012-674417bf6fdb

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0012-674417bf6fdb" sha256="674417bf6fdb6798f8dc6967dba6b4631097472ad25762427bb6f7edadfbd5ae" -->

### 2026-06-17 — Track A initial quality gates

- Added `.github/workflows/ci.yml` for general quality gates independent from packaging release workflow.
- Gates cover Linux dependency install, `cargo fmt --all --check`, workspace check/test, docs snippet check, `xtask package validate`, packaging dry-run, and install-smoke dry-run.

### prompt-p15-quality-hardening-md-0013-2cea5382bd7e

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0013-2cea5382bd7e" sha256="2cea5382bd7ecab87fdee663b4dcac5729561b64298357d4b325ec25a8211ef5" -->

### 2026-06-17 — Track B API consistency slice

- Broadened remaining exact-`Pixels` public builder parameters to `impl Into<Pixels>` for chart dimensions/strokes, P13 visual components, `TagFlow`, `Operation`, and `HorizontalList` height. This is source-compatible for existing `px(...)` calls and aligns these APIs with newer controls such as Input, Select, Progress, QR Code, virtualized controls, and form controls.
- Kept call sites/tests using explicit `px(...)` where they document visual dimensions so unit intent remains clear.
- Added/extended builder-state assertions for SignalMeter, HeatBar, SegmentRatioBar, Label, Operation, and TagFlow dimension/gap options.

Validation evidence for this slice:
- `cargo test -p liora-components -- --nocapture` passed: 192 unit tests plus integration tests.
- Full P15 gate suite passed: fmt, workspace check/test, docs snippet check, package validate, packaging dry-run, install-smoke dry-run, and `git diff --check`.
- GUI smoke passed: `timeout 10s cargo run -p liora-gallery` and `timeout 10s cargo run -p liora-docs` both started successfully and exited via expected timeout.


### prompt-p15-quality-hardening-md-0014-bf060227fc00

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0014-bf060227fc00" sha256="bf060227fc004b84a550ca64c94cc1837b5689ef05ccd51e9459709db5285426" -->

### 2026-06-17 — Track B callback/state and panic audit

- Added API consistency audit tests that lock the public callback convention: value callbacks use `(value, &mut Window, &mut App)`, while entity-local controls such as `Input`, `CodeEditor`, and `HorizontalList` explicitly use `Context<...>` callbacks.
- Added state-builder audit coverage for `disabled(...)` and `close_on_escape(...)` naming across representative controls.
- Removed avoidable production-path `unwrap()` / `expect()` / paint-result panics from hardened paths: Button icon-only rendering, DateTimePicker defaults, Input masked/word selection and paint, InputNumber filtering, Chart downsampling, Sparkline empty-data handling, and CodeBlock paint paths.

Validation evidence for this slice:
- `cargo test -p liora-components api_consistency_audit_tests -- --nocapture` passed.
- `cargo test -p liora-components -- --nocapture` passed: 195 unit tests plus package integration tests.
- Full P15 gate suite passed after whitespace cleanup: fmt, workspace check/test, docs snippet check, package validate, packaging dry-run, install-smoke dry-run, and `git diff --check`.
- GUI smoke passed: `timeout 10s cargo run -p liora-gallery` and `timeout 10s cargo run -p liora-docs` both started successfully and exited via expected timeout.


### prompt-p15-quality-hardening-md-0015-089d433669c4

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0015-089d433669c4" sha256="089d433669c47f127be17d603bf24f44fca8b409aab84d0a7c306d5a8fed7c96" -->

### 2026-06-17 — Track C visual/theme consistency slice

- Replaced hard-coded production `gpui::white()` text on colored/dark Tag and line Progress surfaces with `theme.neutral.inverted`, preserving contrast intent while honoring light/dark theme tokens.
- Added visual/theme audit tests for hardened colored surfaces and representative Virtualized* surface/border/radius token usage.

Validation evidence for this slice:
- `cargo test -p liora-components -- --nocapture` passed: 197 unit tests plus package integration tests.
- Full P15 gate suite passed: fmt, workspace check/test, docs snippet check, package validate, packaging dry-run, install-smoke dry-run, and `git diff --check`.
- GUI smoke passed: `timeout 10s cargo run -p liora-gallery` and `timeout 10s cargo run -p liora-docs` both started successfully and exited via expected timeout.


### prompt-p15-quality-hardening-md-0016-d8b404468b5a

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0016-d8b404468b5a" sha256="d8b404468b5a28b9918d3bc9dcc0ad270a351e0452dd4b63001490acd58348c7" -->

### 2026-06-17 — Track C chart label theme-token slice

- Replaced hard-coded production `gpui::white()` value labels inside stacked BarChart fills and Pie/Ring slices with `theme.neutral.inverted` passed through render helpers.
- Extended visual/theme audit coverage so chart value labels stay covered alongside Tag and Progress colored surfaces.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-components -- --nocapture` passed: 197 unit tests plus package integration tests.
- `cargo check --workspace --all-targets` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0017-e69ed20a749b

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0017-e69ed20a749b" sha256="e69ed20a749b7e00cadf5f7e204ff2bd302d3180a63476de7cc79e54009e1c26" -->

### 2026-06-17 — Track C themed control text slice

- Replaced hard-coded production white text for Button gradient text, Badge text, and Pagination active-background text with `theme.neutral.inverted`.
- Kept remaining `gpui::white()` occurrences where they are non-text color math, marker/border overlays, tests, or caller-provided example values.
- Extended visual/theme audit tests for Badge/Pagination colored surfaces and Button gradient text.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-components -- --nocapture` passed: 198 unit tests plus package integration tests.
- `cargo check --workspace --all-targets` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0018-62cb3d27deac

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0018-62cb3d27deac" sha256="62cb3d27deac91c8bbf154f64293c9126d528da7809c73b1e8284738ba78806c" -->

### 2026-06-18 — Track D popover wrapper outside-close policy

- Added `close_on_click_outside(...)` to Dropdown and Popconfirm so Popover wrappers expose the same outside-click close policy as their underlying overlay shell.
- Forwarded the policy to `Popover::close_on_click_outside(...)` while preserving default close-on-outside behavior.
- Added source-level coverage for wrapper defaults, public builders, and forwarding.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-components -- --nocapture` passed: 199 unit tests plus package integration tests.
- `cargo check --workspace --all-targets` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0019-ea2bf0615347

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0019-ea2bf0615347" sha256="ea2bf0615347e2b285b5efb3bc848bdbabc08c3cae9007db845e9a0efc8f1126" -->

### 2026-06-18 — Track D outside-close docs/examples slice

- Added Dropdown close-strategy docs, live demo, compile-checked snippet, and Gallery example for `close_on_click_outside(false)` / `close_on_escape(false)`.
- Updated Popconfirm custom close-policy examples in Docs, snippets, and Gallery to show both ESC and outside-click close configuration.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0020-7f3f2bdd39bb

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0020-7f3f2bdd39bb" sha256="7f3f2bdd39bb9b96672e61b6bd8373b3aaf3952f199948b53d7ec4b0f2936686" -->

### 2026-06-18 — Track D input popup outside-close policy

- Added `close_on_click_outside(...)` to Select and Autocomplete so common input popups can opt out of automatic outside-click close without losing ESC policy control.
- Kept defaults unchanged (`true`) and bound outside-click handlers conditionally.
- Added source-level regression coverage for input popup outside-close defaults, builders, and conditional bindings.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-components overlay_escape_coverage_tests -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test -p liora-components -- --nocapture` passed: 200 unit tests plus package integration tests.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0021-d8ef10aa42c0

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0021-d8ef10aa42c0" sha256="d8ef10aa42c0a798dabd0fd26f3fa6788c3c0a08c5ca1007b78b44d312be5f5e" -->

### 2026-06-18 — Track D picker outside-close policy

- Added `close_on_click_outside(...)` to Cascader, DatePicker, DateTimePicker, TimePicker, and ColorPicker.
- Preserved default outside-click close behavior while making portal backdrop close handlers conditional on the new policy flag.
- Extended source-level popup policy coverage across input popups and picker popups.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-components overlay_escape_coverage_tests -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test -p liora-components -- --nocapture` passed: 200 unit tests plus package integration tests.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0022-5261736a817f

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0022-5261736a817f" sha256="5261736a817f1080fb45444a067daec971f233403ea2fe7599d80c6a7150b299" -->

### 2026-06-18 — Track D popup close-policy docs/examples slice

- Added close-policy examples to Select, Autocomplete, DatePicker, TimePicker, and ColorPicker Docs/Gallery coverage by applying `close_on_click_outside(false)` and `close_on_escape(false)` in representative scenarios.
- Updated compile-checked snippets and live docs renderers so the new public popup policy builders remain exercised.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0023-81cf77c530d0

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0023-81cf77c530d0" sha256="81cf77c530d0951623b7d84d71a37b0cc2a5068771602310d682cf2f109543e6" -->

### 2026-06-18 — Track D Preview outside-close policy

- Added `close_on_click_outside(...)` to Preview and ActiveImagePreview state so image preview overlays can opt out of backdrop click dismissal independently from ESC handling.
- Preserved default outside-click close behavior and made the overlay click handler conditional on the policy flag.
- Added source-level regression coverage for Preview outside-click policy.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-components overlay_escape_coverage_tests -- --nocapture` passed.
- `cargo test -p liora-components preview::tests::preview_overlay_has_escape_close_action_and_image_sized_hitbox -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test -p liora-components -- --nocapture` passed: 201 unit tests plus package integration tests.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0024-2339bb9edfac

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0024-2339bb9edfac" sha256="2339bb9edfacee803d6c47f75ead49816c3e84fb71decdbf52870bffbe8a6ffc" -->

### 2026-06-18 — Track D Preview close-policy docs/examples slice

- Documented Preview close policy controls in Docs by expanding ESC-only guidance into combined ESC and outside-click close strategy guidance.
- Updated the compile-checked Preview snippet and live docs renderer to exercise both `close_on_escape(false)` and `close_on_click_outside(false)`.
- Added a Gallery Preview close-policy example so the API-only overlay behavior is discoverable in the native demo app.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0025-46ac4c03d6d9

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0025-46ac4c03d6d9" sha256="46ac4c03d6d965451bb93bb3371d5d34aba9513d5b1bb5c66d5d9b991aa7316f" -->

### 2026-06-18 — Track D Tour close-policy docs/examples slice

- Hardened Tour overlay close-policy coverage by locking its ESC and outside-click conditional handlers in source-level tests.
- Added a controlled-close Tour Gallery example that disables both ESC and outside-click dismissal for critical guided flows.
- Added Tour close-policy docs and a compile-checked snippet, and fixed the docs snippet loader to display authored Tour snippets instead of falling back to missing-source text.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-components tour::tests -- --nocapture` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0026-1b6f66a8ab13

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0026-1b6f66a8ab13" sha256="1b6f66a8ab133d8078bb6c1e9eaa09549a1ef30eea0132d04b4a747c4f339bc3" -->

### 2026-06-18 — Track A CI/package workflow boundary docs

- Documented the responsibility split between `.github/workflows/ci.yml` and `.github/workflows/package.yml` in the Packaging Workflow docs page.
- Clarified that ordinary CI stops at validation/dry-run gates, while package workflow owns platform-specific packaging, raw binary staging, artifact upload, grouped changelog generation, and `v*` tag GitHub Release publishing.
- Added a docs regression test so the workflow boundary and release-asset rule stay visible in the native docs app.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-docs markdown::tests::packaging_docs_explain_ci_and_release_workflow_boundaries -- --nocapture` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0027-8dfcbab670d1

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0027-8dfcbab670d1" sha256="8dfcbab670d14066a46b5f4dd34770f44b04def6ad4a622625628aeddac4402f" -->

### 2026-06-18 — Track F docs snippet loader completeness

- Audited all authored docs `src="..."` code blocks against the Docs UI snippet loader and compile-check harness.
- Fixed 22 Docs UI loader gaps for Calendar, Carousel, InputTag, Mention, Progress gradient completion, TreeSelect, VirtualizedTable, VirtualizedTree, and Watermark snippets; all files were already present and compile-checked, but the UI could not display them.
- Added a regression test that parses every docs page and asserts every referenced snippet can be loaded by the native Docs renderer.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-docs markdown::tests::authored_page_snippets_are_available_to_docs_loader -- --nocapture` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0028-4fa678755aed

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0028-4fa678755aed" sha256="4fa678755aedf6e1b5b90bf84473d08854ddb05734d6a6aab0af092863f21689" -->

### 2026-06-18 — Track A split CI quality and packaging dry-run jobs

- Split `.github/workflows/ci.yml` into `rust-quality` and `packaging-dry-run` jobs.
- Kept full GPUI/Linux native dependencies only on the workspace fmt/check/test/docs-snippet job, while the packaging dry-run job now installs only lightweight tooling needed by `xtask` dry-run gates.
- Updated Packaging Workflow docs and regression tests to lock the CI job split and prevent packaging dry-run from silently inheriting unused rpm/zsync/native GUI dependency setup.

Validation evidence for this slice:
- Workflow YAML parsed successfully with PyYAML.
- `cargo fmt --all --check` passed.
- `cargo test -p liora-docs packaging -- --nocapture` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo run -p xtask -- package validate` passed.
- `cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build` passed.
- `cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0029-5441658e3d71

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0029-5441658e3d71" sha256="5441658e3d71b36cf8d8f496faee9234b82b2ac085c80929e2a371790b6802c0" -->

### 2026-06-18 — Track F QuickStart key binding completeness

- Updated the QuickStart minimal window snippet to register `CodeEditor` and `Tour` key bindings alongside the rest of the core app-level bindings.
- Added a docs regression test that compares the QuickStart example against Gallery and Docs for key bindings that affect text selection, code editing, Preview, and Tour overlay behavior.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-docs markdown::tests::quick_start_registers_core_app_key_bindings -- --nocapture` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0030-6ab8e8216e9c

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0030-6ab8e8216e9c" sha256="6ab8e8216e9c23b01aa233806476f42eaa52beb209c09348b0359c769648b383" -->

### 2026-06-18 — Track E CodeBlock highlight cache eviction

- Replaced CodeBlock's all-or-nothing highlight cache clear with a bounded FIFO eviction policy so one cache overflow no longer invalidates every highlighted snippet at once.
- Kept HashMap lookup behavior for render speed while adding insertion-order tracking for incremental eviction.
- Added a regression test proving the oldest entry is evicted while newer cached runs survive at capacity.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-components code_block::tests -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `git diff --check` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0031-18a7ba39e49b

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0031-18a7ba39e49b" sha256="18a7ba39e49b44b3b2b198dd98c021b34d6bc1ca95c5f768428d1c57610f9ac5" -->

### 2026-06-18 — Track E CodeBlock shared highlight runs

- Changed the CodeBlock highlight cache value from owned `Vec<TextRun>` to shared `Arc<[TextRun]>` storage so repeated block renders reuse the cached highlight run allocation instead of cloning the full run vector for every visible CodeBlock/CodeEditor preview.
- Added a cached helper that returns the highlight key together with shared runs, letting selectable/read-only code layouts invalidate from the cache key while preserving existing public `cached_highlight_runs` behavior for inline styled text.
- Added a regression test proving repeated block highlight lookups share the same Arc-backed run storage.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-components code_block::tests -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `git diff --check -- . ':(exclude).omx'` passed after removing markdown EOF whitespace.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0032-e7f31b6f66b5

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0032-e7f31b6f66b5" sha256="e7f31b6f66b559b1f332b97082ba5e3f83b3a81c75616343ad88a3a9008a10b8" -->

### 2026-06-18 — Track B synchronized state panic hardening

- Replaced production `expect("... lock poisoned")` paths in CodeBlock highlight/selection state, SelectableText selection state, and Timer runtime registries with poisoned-lock recovery via `into_inner()`.
- Added small lock helper functions so cache/selection/timer runtime state can continue operating after an unrelated panic poisons a mutex instead of crashing the GPUI UI loop.
- Extended the API consistency panic audit to lock this behavior for CodeBlock, SelectableText, and Timer synchronized runtime state.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-components code_block::tests -- --nocapture` passed.
- `cargo test -p liora-components selectable_text::tests -- --nocapture` passed.
- `cargo test -p liora-components timer::tests -- --nocapture` passed.
- `cargo test -p liora-components api_consistency_audit_tests::avoidable_runtime_panics_stay_out_of_hardened_paths -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `git diff --check -- . ':(exclude).omx'` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0033-d61ddffa7ae2

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0033-d61ddffa7ae2" sha256="d61ddffa7ae296e69ab3bd31a16932765c72f61185e4b30e6cfa91ad5f59c025" -->

### 2026-06-18 — Track B tray icon fallback hardening

- Replaced Gallery and Docs startup tray icon `expect(...)` calls with recoverable bundled-icon loading helpers.
- Added app-specific solid-color fallback icons for Gallery and Docs, and allowed tray installation to proceed without an icon only if both bundled and fallback icon creation fail.
- Updated dynamic tray icon switching to skip invalid icon updates instead of panicking during command handling.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo check -p liora-gallery --all-targets` passed.
- `cargo check -p liora-docs --all-targets` passed.
- `cargo test -p liora-gallery shell_tests::gallery_shell_uses_container_and_menu -- --nocapture` passed.
- `cargo test -p liora-docs markdown::tests::docs_shell_registers_core_documentation_pages -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `git diff --check -- . ':(exclude).omx'` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0034-92a060069d0e

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0034-92a060069d0e" sha256="92a060069d0ea4234a6b01427fab4062b2412d4a5bd5938949bc9ffbd45eea37" -->

### 2026-06-18 — Track B packager string rendering panic cleanup

- Removed avoidable `expect("write to string")` calls from `liora-packager` checksum and manifest rendering paths.
- Switched checksum hex and manifest text/JSON assembly to infallible `push_str(format!(...))` style output while preserving existing generated formats.
- Verified `liora-packager` unit tests plus full workspace gates so packaging metadata generation remains stable.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo test -p liora-packager -- --nocapture` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `git diff --check -- . ':(exclude).omx'` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0035-7e7b498f214b

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0035-7e7b498f214b" sha256="7e7b498f214b2daa320ab15aa65a272812933fbf95d16270a87b30302512670f" -->

### 2026-06-18 — Track B lucide build script error handling

- Replaced `liora-icons-lucide` build script `unwrap()` calls with a `try_main() -> io::Result<()>` flow.
- Build failures now emit a clear `cargo:error=...` message and exit non-zero instead of panicking with an unwrap stack.
- Added explicit UTF-8 validation errors for generated icon file names and propagated directory/file/write errors with context from the build script path being processed.

Validation evidence for this slice:
- `cargo fmt --all --check` passed.
- `cargo check -p liora-icons-lucide --all-targets` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `git diff --check -- . ':(exclude).omx'` passed.
- Gallery/Docs GUI smoke passed via expected `timeout 10s` startup runs.


### prompt-p15-quality-hardening-md-0036-cccd10fe796e

<!-- ctx-migration source=".prompt/P15-quality-hardening.md" unit="prompt-p15-quality-hardening-md-0036-cccd10fe796e" sha256="cccd10fe796ed0d068719c61dd92a7f7d2ea176c063203c8b7628ba5842385bc" -->

### 2026-06-18 — P15 final completion audit

P15 is complete. The final audit confirms all local quality hardening tracks are covered:

- Track A: CI/verification gates and CI/package workflow boundaries are in place and documented.
- Track B: API consistency and avoidable runtime panic cleanup passed the hardened-path audit, including synchronized UI state recovery, tray icon fallback handling, packager string rendering, and lucide build-script error handling.
- Track C: visual/theme consistency hardening uses theme tokens for representative colored text surfaces and chart labels.
- Track D: interaction/overlay behavior is covered for ESC and outside-click close policy across common overlays, popups, Preview, and Tour.
- Track E: CodeBlock performance hardening uses incremental highlight-cache eviction and shared `Arc<[TextRun]>` highlight-run storage; chart downsampling remains covered from prior phases.
- Track F: docs/snippet completeness is covered by the native Docs loader audit and compile-checked snippets; QuickStart key bindings are aligned with Gallery/Docs startup registration.

Final gate evidence:

- `cargo fmt --all --check` passed.
- `cargo check --workspace --all-targets` passed.
- `cargo test --workspace` passed.
- `cargo check -p liora-docs --bin check_snippets` passed.
- `cargo run -p xtask -- package validate` passed.
- `cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build` passed.
- `cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run` passed.
- `git diff --check -- . ':(exclude).omx'` passed.
- `timeout 10s cargo run -p liora-gallery` started successfully and exited via expected timeout status `124`.
- `timeout 10s cargo run -p liora-docs` started successfully and exited via expected timeout status `124`.

Residuals intentionally outside P15 local completion:

- `MessageManager::init` panic remains intentional usage-contract enforcement.
- Gallery fixed date/time demo `expect(...)` calls are compile-time/demo constant assumptions, not production component paths.
- P12 external-policy items remain outside P15: signing, notarization, real system installs/uninstalls, formal license policy, and a real `v*` release validation run.

### prompt-p16-adoption-readiness-md-0001-15bfacc4fa57

<!-- ctx-migration source=".prompt/P16-adoption-readiness.md" unit="prompt-p16-adoption-readiness-md-0001-15bfacc4fa57" sha256="15bfacc4fa576e4101b453227b3e69687ecee782c78cfb4e8f6358efedbb2c80" -->

# P16 — Public API & Adoption Readiness

> 上游: `.prompt/P12-packaging.md` / `.prompt/P15-quality-hardening.md`
> 状态: Complete
> 目标: 让外部 Rust/GPUI 项目能在 10 分钟内理解 Liora、跑起最小示例、找到 API/Docs/发布流程，并知道如何贡献与发布。


### prompt-p16-adoption-readiness-md-0002-003319413160

<!-- ctx-migration source=".prompt/P16-adoption-readiness.md" unit="prompt-p16-adoption-readiness-md-0002-003319413160" sha256="00331941316025ae8148f658adb03adfb82972688f243b428d27da5a859b375a" -->

## Scope

P16 不继续新增大组件，而是完成对外采用闭环：

- 根 README：项目定位、快速启动、示例、文档、验证命令、发布边界。
- 采用示例已回流到 Gallery/Docs；不再维护独立 `examples/minimal-app` workspace package。
- Public API 文档入口：核心 crate-level Rustdoc 说明组件导出、初始化、主题、托盘、图标和打包边界。
- Docs Adoption 页面：在 native Docs app 中说明从最小示例到真实项目采用的步骤。
- 贡献与发布说明：`CONTRIBUTING.md` / `CHANGELOG.md` 与 P12 release-readiness 流程对齐。
- 回归测试：锁住 README、Docs adoption 页面、minimal app、workflow/readiness 入口。


### prompt-p16-adoption-readiness-md-0003-e975d85cfa71

<!-- ctx-migration source=".prompt/P16-adoption-readiness.md" unit="prompt-p16-adoption-readiness-md-0003-e975d85cfa71" sha256="e975d85cfa718adc57f5285a23ade8cc58dd4759fd6be8a8488cb8f888d80479" -->

## Non-goals

- 不引入 Tauri/WebView/HTML/CSS/DOM/browser runtime。
- 不把当前 `LicenseRef-Liora` 伪装成正式 OSS license。
- 不执行真实 public release 或签名/公证；这些仍由 owner/protected environment 控制。
- 不为了文档重构大量组件 API。


### prompt-p16-adoption-readiness-md-0004-907422330f64

<!-- ctx-migration source=".prompt/P16-adoption-readiness.md" unit="prompt-p16-adoption-readiness-md-0004-907422330f64" sha256="907422330f642e1b7a2c6c4ee324f7f3ce69c527917e2931faecaa13e3327f36" -->

## Completion evidence

- Gallery/Docs 是 adoption 的 compile-checked surfaces；不再要求 `liora-minimal-app`。
- `cargo doc --workspace --no-deps` 通过。
- Docs app 包含 Adoption Guide，且测试覆盖 README/Docs/workflow 入口。
- Full local gates pass and changes are committed/pushed.

### prompt-p17-dogfood-dashboard-md-0001-b5798f79ff54

<!-- ctx-migration source=".prompt/P17-dogfood-dashboard.md" unit="prompt-p17-dogfood-dashboard-md-0001-b5798f79ff54" sha256="b5798f79ff540aa41331795cafc9c17eb7162e6d8e1e7be49e2cc5a40e3491c3" -->

# P17 — Dogfooding Dashboard App

> 上游: `.prompt/P16-adoption-readiness.md`
> 状态: Complete
> 目标: 用 Liora 自己构建一个真实 dashboard 示例，验证组件组合、布局、图表、表格、toast、CodeBlock 和启动流程是否适合外部项目采用。


### prompt-p17-dogfood-dashboard-md-0002-2d4c5e469621

<!-- ctx-migration source=".prompt/P17-dogfood-dashboard.md" unit="prompt-p17-dogfood-dashboard-md-0002-2d4c5e469621" sha256="2d4c5e469621ae190a088d49309214d1ce46fe5a46ad6ad51f5e244c84798b22" -->

## Scope

- Dashboard dogfooding 已回流到 Gallery/Docs；不再维护独立 `examples/dashboard-app` workspace package。
- Dashboard 覆盖 header、filters、metric cards、LineChart、BarChart、Progress、Table、CodeBlock、toast 和 key binding setup。
- Native Docs 增加 `Dashboard App` 页面。
- README / adoption docs 补 dashboard app 入口。
- 添加回归测试，锁住 dashboard app workspace、docs 页面和 README 入口。


### prompt-p17-dogfood-dashboard-md-0003-61f3e694ac7e

<!-- ctx-migration source=".prompt/P17-dogfood-dashboard.md" unit="prompt-p17-dogfood-dashboard-md-0003-61f3e694ac7e" sha256="61f3e694ac7e5c2290ed7fa2d7faa604c3b2f5fc429b3997bfc9d2a7d42f9d49" -->

## Completion evidence

- `cargo check -p liora-gallery` and `cargo check -p liora-docs` pass.
- `timeout 10s cargo run -p liora-gallery` and `timeout 10s cargo run -p liora-docs` start and exit with expected timeout status 124.
- Workspace checks/tests/docs/package dry-run gates pass.
- Commit pushed.

### prompt-p18-dashboard-polish-and-api-ergonomics-md-0001-bb52d70987a9

<!-- ctx-migration source=".prompt/P18-dashboard-polish-and-api-ergonomics.md" unit="prompt-p18-dashboard-polish-and-api-ergonomics-md-0001-bb52d70987a9" sha256="bb52d70987a9cff5f081b501d43edd3874d86b3dc95986383fb562487d59ec08" -->

# P18 — Dashboard Polish and API Ergonomics

> 上游: `.prompt/P17-dogfood-dashboard.md`
> 状态: Complete
> 目标: 用 P17 Dashboard Dogfooding App 反向优化 Liora 的真实应用组合体验。


### prompt-p18-dashboard-polish-and-api-ergonomics-md-0002-772d2c27ac12

<!-- ctx-migration source=".prompt/P18-dashboard-polish-and-api-ergonomics.md" unit="prompt-p18-dashboard-polish-and-api-ergonomics-md-0002-772d2c27ac12" sha256="772d2c27ac12cc182f09f26af7f18ea0f3eab2e1a347b96e9297a045ebe63058" -->

## Scope

- Polish Gallery/Docs so dashboard-style shell behaviors live in maintained surfaces, not standalone sample apps.
- Add light/dark theme switching in the dashboard dogfood app.
- Keep dashboard-specific composition helpers app-local unless they become neutral reusable component APIs across maintained surfaces.
- Add native Docs `Dashboard Patterns` guidance.
- Update README / prompt / memory and regression tests.


### prompt-p18-dashboard-polish-and-api-ergonomics-md-0003-f7cb4e6bb959

<!-- ctx-migration source=".prompt/P18-dashboard-polish-and-api-ergonomics.md" unit="prompt-p18-dashboard-polish-and-api-ergonomics-md-0003-f7cb4e6bb959" sha256="f7cb4e6bb959541e97294df2fb68d6ac1f61fa39b82a0a62094be2fe4100a99d" -->

## Completion evidence

- `cargo check -p liora-gallery` and `cargo check -p liora-docs` pass.
- `liora-components` has no `dashboard` module or dashboard sample/model exports.
- `cargo test -p liora-docs markdown::tests::dashboard_patterns_keep_sample_code_out_of_components -- --nocapture` passes.
- Workspace checks/tests/docs/package dry-run gates pass.
- GUI smoke for Gallery and Docs starts and exits by expected timeout.
- Commit pushed.

### prompt-p19-dashboard-state-and-data-flow-md-0001-c132359caae7

<!-- ctx-migration source=".prompt/P19-dashboard-state-and-data-flow.md" unit="prompt-p19-dashboard-state-and-data-flow-md-0001-c132359caae7" sha256="c132359caae728f391534de3f98f65fcfa740b4e10d4a5c4dd84dd918287bd54" -->

# P19 — Dashboard State and Data Flow

> 上游: `.prompt/P18-dashboard-polish-and-api-ergonomics.md`
> 状态: Complete
> 目标: 将 Dashboard Dogfooding App 从静态展示推进到可测试的数据模型、过滤、刷新和状态分支样板。


### prompt-p19-dashboard-state-and-data-flow-md-0002-2790a2bad188

<!-- ctx-migration source=".prompt/P19-dashboard-state-and-data-flow.md" unit="prompt-p19-dashboard-state-and-data-flow-md-0002-2790a2bad188" sha256="2790a2bad1884bfc26b8ae6ccff3488e96a929c2ec0592c14013b6254ba5be2b" -->

## Scope

- Add explicit dashboard model structs for metrics, services, filters, status, and generated data.
- Make search, region, and alerts-only filters affect the service table and generated mock data.
- Make refresh regenerate revisioned dashboard data across metrics, charts, table rows, and progress panels.
- Cover loading/ready/empty/degraded state branches with ordinary Liora components.
- Add native Docs `Dashboard State` page and regression coverage.


### prompt-p19-dashboard-state-and-data-flow-md-0003-79446134c4a4

<!-- ctx-migration source=".prompt/P19-dashboard-state-and-data-flow.md" unit="prompt-p19-dashboard-state-and-data-flow-md-0003-79446134c4a4" sha256="79446134c4a4d28da083671b99b9d391cc25db3d2116ef9331b67ef51d2eae50" -->

## Completion evidence

- `cargo check -p liora-gallery` and `cargo check -p liora-docs` pass.
- Gallery/Docs state/filtering guidance remains covered by docs and shell tests.
- Business sample models must not be moved into `liora-components`; keep them app-layer when needed.
- `cargo test -p liora-docs markdown::tests::dashboard_state_docs_cover_data_flow_model -- --nocapture` passes.
- Workspace checks/tests/docs/package dry-run gates pass.
- GUI smoke for Gallery, Docs, Minimal App, and Dashboard App starts and exits by expected timeout.
- Commit pushed.

### prompt-p2-form-controls-md-0001-5c1a61094958

<!-- ctx-migration source=".prompt/P2-form-controls.md" unit="prompt-p2-form-controls-md-0001-5c1a61094958" sha256="5c1a610949585cc8551f720315e6d8fb748f555be6928c1384ac6609ac0cd762" -->

# P2 Form Controls — 表单数据录入

> 上游: `.prompt/P1-basic-elements.md`


### prompt-p2-form-controls-md-0002-d394fb1a7344

<!-- ctx-migration source=".prompt/P2-form-controls.md" unit="prompt-p2-form-controls-md-0002-d394fb1a7344" sha256="d394fb1a73440e17d7b6b128a4237e67af54ef6e997e3cce8576255735546fd3" -->

## 目标

完成 10 个表单核心组件的开发。


### prompt-p2-form-controls-md-0003-f5da5ea97da5

<!-- ctx-migration source=".prompt/P2-form-controls.md" unit="prompt-p2-form-controls-md-0003-f5da5ea97da5" sha256="f5da5ea97da5bfd7bff5bbe1b6db31e99eb06b7eeb17262c5f9ffea156c6a3f5" -->

## 组件清单

1. **Input** — 文本输入框 (prefix/suffix icon, clearable, password toggle, maxlength)
2. **InputNumber** — 数字输入 (步进按钮 ±, min/max/precision)
3. **Textarea** — 多行文本 (auto-resize, maxlength 计数)
4. **Checkbox** / **CheckboxGroup** — 多选 (indeterminate 半选, min/max 限制)
5. **Radio** / **RadioGroup** — 单选 (button 样式, border)
6. **Switch** — 开关 (active/inactive 文字, loading)
7. **Select** — 下拉选择 ⚠️ (需要 Popper 定位基础)
8. **Slider** — 滑块 (范围选择, 刻度, input 联动)
9. **Form** / **FormItem** — 表单容器 (label-width, required, 校验, error message)
10. **Rate** — 评分 (半星, 文字辅助, 只读)


### prompt-p2-form-controls-md-0004-692994ee76fc

<!-- ctx-migration source=".prompt/P2-form-controls.md" unit="prompt-p2-form-controls-md-0004-692994ee76fc" sha256="692994ee76fcc43715dae435fab73fb3af19118fd8210af0cf89987f31d69697" -->

## 关键挑战

**Select 组件**: 这是第一个需要使用 Popper/Portal 弹出定位的组件。如果 Popper 基建尚未完成，Select 降级为使用 `div().absolute()` 的相对定位方案。


### prompt-p2-form-controls-md-0005-44b01da8149d

<!-- ctx-migration source=".prompt/P2-form-controls.md" unit="prompt-p2-form-controls-md-0005-44b01da8149d" sha256="44b01da8149d8dc25d06782f2ffc8997803d31329add0d2c46da40abc0139db2" -->

## 依赖关系

```
Input → (无依赖, 优先开发)
Textarea → (无依赖)
InputNumber → Input (共享基础样式)
Switch → (无依赖)
Checkbox/Radio → (无依赖)
Slider → (无依赖)
Rate → (无依赖)
Form/FormItem → (无依赖, 状态管理 Model)
Select → Popper基建 or 简化方案
```


### prompt-p2-form-controls-md-0006-b4f74dbd2153

<!-- ctx-migration source=".prompt/P2-form-controls.md" unit="prompt-p2-form-controls-md-0006-b4f74dbd2153" sha256="b4f74dbd21538eefdc9b5aa8fdeb38b1721b783cf42ba537d47b2dedc777674e" -->

## 推荐开发顺序

Input → Textarea → Switch → Checkbox → Radio → InputNumber → Slider → Rate → Form → Select

### prompt-p20-theme-and-interaction-polish-md-0001-9072f7013955

<!-- ctx-migration source=".prompt/P20-theme-and-interaction-polish.md" unit="prompt-p20-theme-and-interaction-polish-md-0001-9072f7013955" sha256="9072f7013955abc39848f2ae3bfa8c5dc67c41c726b7ab3315df18913896feb5" -->

# P20 — Theme and Interaction Polish

> 上游: `.prompt/P15-quality-hardening.md` / `.prompt/P19-dashboard-state-and-data-flow.md`
> 状态: Complete
> 目标: 收口 Liora 的 System/Light/Dark 主题模式、语义 token、浮层遮罩和关键交互状态一致性。


### prompt-p20-theme-and-interaction-polish-md-0002-529a34d8cdc8

<!-- ctx-migration source=".prompt/P20-theme-and-interaction-polish.md" unit="prompt-p20-theme-and-interaction-polish-md-0002-529a34d8cdc8" sha256="529a34d8cdc856bdbb9cd3766e49e3618bf39449c8153d585c22ae2c422b794e" -->

## Scope

P20 不新增大型业务组件，聚焦主题与交互一致性：

- System / Light / Dark 三模式作为正式主题入口。
- 深色模式下 subtle semantic token 不再用白色混合，而是低透明度语义色 overlay。
- 常见浮层遮罩改用 `theme.neutral.overlay`，全屏 Loading mask 改用 `theme.neutral.mask`。
- 关键控件避免写死浅色边框 / 浅色背景 / 红白关闭按钮。
- Gallery 增加 Theme dogfooding 页面；Docs 增加 Theme System 页面和 compile-checked snippet。
- 增加源码级回归测试防止重新引入硬编码 light/dark 颜色。


### prompt-p20-theme-and-interaction-polish-md-0003-c16a3ea8a34f

<!-- ctx-migration source=".prompt/P20-theme-and-interaction-polish.md" unit="prompt-p20-theme-and-interaction-polish-md-0003-c16a3ea8a34f" sha256="c16a3ea8a34fac7357d7772956a1c3a7f90af4fe950fbbf4a6d58249233c4da9" -->

## Completed changes

- `liora-theme`
  - 新增 dark semantic subtle token 生成路径：`ColorFamily::new_dark(...)`。
  - `primary/info/success/warning/danger.light_9/light_8/light_7` 在 dark 模式下为透明 overlay，避免 Table hover、Picker chip、Upload hover 等区域过亮。
  - 增加 light/dark subtle token 回归测试。

- `liora-components`
  - `Dialog` / `Drawer` / `Tour` 遮罩改用 `theme.neutral.overlay`。
  - `Loading::full_screen()` 背景改用 `theme.neutral.mask`。
  - `CodeEditor` 行号 gutter 边框改用 `theme.neutral.border`。
  - `AppWindowFrame` 自定义关闭按钮 hover 改用 `theme.danger.base` + `theme.neutral.inverted`。
  - 增加 `visual_theme_consistency_tests` 覆盖遮罩、CodeEditor、WindowFrame token 使用。

- `liora-gallery`
  - 新增 `Theme 主题系统` demo，展示当前 `ThemeMode`、语义色 token 和按钮交互状态。

- `liora-docs`
  - 新增 `Theme` 页面，说明 System/Light/Dark、`observe_window_appearance`、token 使用原则和 dark subtle token 策略。
  - 新增 `theme/system_mode.rs` compile-checked snippet。
  - 增加 Docs 页面注册和 snippet loader 回归测试。


### prompt-p20-theme-and-interaction-polish-md-0004-256bffe408b7

<!-- ctx-migration source=".prompt/P20-theme-and-interaction-polish.md" unit="prompt-p20-theme-and-interaction-polish-md-0004-256bffe408b7" sha256="256bffe408b7630ccf45ff993565aa72761ade1eb9fde996fcc94c7794cdecf6" -->

## Non-goals

- 不替换 Liora 的主题品牌色。
- 不把所有用户自定义颜色 demo 强制改为语义色；自定义颜色 API 的存在是刻意能力。
- 不执行截图级视觉自动化；当前验证以源码回归、编译、测试、Docs/Gallery smoke 为主。
- 不引入 WebView/HTML/CSS/DOM/browser runtime。


### prompt-p20-theme-and-interaction-polish-md-0005-6fe9ee6c8c24

<!-- ctx-migration source=".prompt/P20-theme-and-interaction-polish.md" unit="prompt-p20-theme-and-interaction-polish-md-0005-6fe9ee6c8c24" sha256="6fe9ee6c8c2432a7f9621944bb54593b1650dc33c345e861f7ec7847bd614fbd" -->

## Completion evidence

- `cargo fmt --all --check`
- `cargo check --workspace --all-targets`
- `cargo test --workspace`
- `cargo check -p liora-docs --bin check_snippets`
- `cargo doc --workspace --no-deps`
- `cargo run -p xtask -- package validate`
- `cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build`
- `cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run`
- `timeout 10s cargo run -p liora-gallery`
- `timeout 10s cargo run -p liora-docs`
- `git diff --check -- . ':(exclude).omx'`

### prompt-p21-release-candidate-readiness-md-0001-5ef5994f7df4

<!-- ctx-migration source=".prompt/P21-release-candidate-readiness.md" unit="prompt-p21-release-candidate-readiness-md-0001-5ef5994f7df4" sha256="5ef5994f7df4a7d8858472a095ac7612834287a59367b998f84703d543442b2f" -->

# P21 — Release Candidate Readiness


### prompt-p21-release-candidate-readiness-md-0002-60833c0467c5

<!-- ctx-migration source=".prompt/P21-release-candidate-readiness.md" unit="prompt-p21-release-candidate-readiness-md-0002-60833c0467c5" sha256="60833c0467c51bc15c74a871d99efdf7873985042062c9e95c06ec8d6cf05949" -->

## Status

✅ Complete — 2026-06-18


### prompt-p21-release-candidate-readiness-md-0003-703b2c2eb585

<!-- ctx-migration source=".prompt/P21-release-candidate-readiness.md" unit="prompt-p21-release-candidate-readiness-md-0003-703b2c2eb585" sha256="703b2c2eb585358f2caece328a30274e31b162b52123f1e285c0bb17bf20b76e" -->

## Goal

Close the repository-owned release-candidate gap for Liora `0.1.0` without changing Liora's runtime architecture. This phase does not ship a public release; it makes the repository ready for an owner-controlled protected release path.


### prompt-p21-release-candidate-readiness-md-0004-7303c6631a67

<!-- ctx-migration source=".prompt/P21-release-candidate-readiness.md" unit="prompt-p21-release-candidate-readiness-md-0004-7303c6631a67" sha256="7303c6631a67621c2a98deeffcc71493dedce730c16174d8949fcdcd569ae42c" -->

## Non-negotiable boundaries

- Liora remains pure Rust + GPUI native.
- Do not introduce Tauri, WebView, HTML/CSS/DOM, browser runtime, or web chart/runtime shells.
- Do not re-add standalone `examples/minimal-app` or `examples/dashboard-app`; Gallery and Docs remain the canonical adoption/dogfooding apps.
- Keep sample/business screens and mock dashboard models out of `liora-components`.
- Keep `.omx/**` out of commits.


### prompt-p21-release-candidate-readiness-md-0005-7e5a82cab254

<!-- ctx-migration source=".prompt/P21-release-candidate-readiness.md" unit="prompt-p21-release-candidate-readiness-md-0005-7e5a82cab254" sha256="7e5a82cab254a42b9dc0057d7945b50b8d4425379cd6ae7f2520e4dd0a91019d" -->

## Delivered scope

- Added `docs/release-candidate-checklist.md` as the RC source of truth.
- Refreshed README/CHANGELOG/prompt/memory so P12–P21 describe current reality instead of old sample-app or license TODOs.
- Added explicit package metadata to workspace package manifests: SDK crates are crates.io publishable with `license-file = "../../LICENSE.md"`; app/automation crates remain `publish = false` with `LicenseRef-Liora` metadata.
- Locked the RC boundary with docs regression tests covering commands, metadata, app boundaries, package workflow roles, SDK publishing, and absence of removed sample apps.


### prompt-p21-release-candidate-readiness-md-0006-19d21d2acf97

<!-- ctx-migration source=".prompt/P21-release-candidate-readiness.md" unit="prompt-p21-release-candidate-readiness-md-0006-19d21d2acf97" sha256="19d21d2acf97c537ab8d6b8cf45edd51e89f58c2bbcb526d154838097e7a6d2c" -->

## Required local verification

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p liora-docs --bin check_snippets
cargo doc --workspace --no-deps
cargo run -p xtask -- package validate
cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package ci --app gallery --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --app gallery --format platform-defaults --dry-run
git diff --check -- . ':(exclude).omx'
timeout 10s cargo run -p liora-gallery
timeout 10s cargo run -p liora-docs
```

`timeout 10s` GUI smoke exits with status `124` are expected after the native window starts.


### prompt-p21-release-candidate-readiness-md-0007-5b34f1ffdc6b

<!-- ctx-migration source=".prompt/P21-release-candidate-readiness.md" unit="prompt-p21-release-candidate-readiness-md-0007-5b34f1ffdc6b" sha256="5b34f1ffdc6ba282919175aa12af5dcf12f5007494eefa53b8adc21d03c9de36" -->

## Protected follow-up

Only the owner/protected release environment should run the real `v0.1.0` tag release, crates.io SDK publication through `release-sdk.yml`, macOS notarization, Windows signing, destructive system install/uninstall smoke, and GitHub Release publication.

### prompt-p22-gpui-component-harvest-md-0001-3bcc4a5a23ce

<!-- ctx-migration source=".prompt/P22-gpui-component-harvest.md" unit="prompt-p22-gpui-component-harvest-md-0001-3bcc4a5a23ce" sha256="3bcc4a5a23ced9a2338d811ebcacbf71a92231ca65e1c5e68417a51c86e58781" -->

# P22 — gpui-component Harvest


### prompt-p22-gpui-component-harvest-md-0002-7b8486c77de9

<!-- ctx-migration source=".prompt/P22-gpui-component-harvest.md" unit="prompt-p22-gpui-component-harvest-md-0002-7b8486c77de9" sha256="7b8486c77de950f6b1b92f8c527ca44f0508862190d80fa3d19bd0f8facc585f" -->

## Status

✅ Complete — started 2026-06-25; closed 2026-06-29


### prompt-p22-gpui-component-harvest-md-0003-6cbb6248c806

<!-- ctx-migration source=".prompt/P22-gpui-component-harvest.md" unit="prompt-p22-gpui-component-harvest-md-0003-6cbb6248c806" sha256="6cbb6248c8064af1279de4cece1d0111b1f2dc8fdea3980719ced1d37cc566d6" -->

## Goal

Turn the `design/gpui-component-collection-list.md` research into Liora-native components and enhancements while preserving Liora's Element-Plus-style builder APIs, theme tokens, and pure Rust + GPUI native architecture.


### prompt-p22-gpui-component-harvest-md-0004-9fa12f6fdc1c

<!-- ctx-migration source=".prompt/P22-gpui-component-harvest.md" unit="prompt-p22-gpui-component-harvest-md-0004-9fa12f6fdc1c" sha256="9fa12f6fdc1cdc23da6659d1a3919efad62de434bf309a85174ca7fd2226f85c" -->

## Non-negotiable boundaries

- Liora remains pure Rust + GPUI native.
- Do not introduce WebView, HTML/CSS/DOM, browser runtime, WASM gallery, or Tauri.
- Do not copy `longbridge/gpui-component` APIs directly; adapt only the product capability into Liora style.
- Existing components must be enhanced in place rather than replaced by parallel controls.
- Keep Gallery and Docs as the canonical adoption/dogfooding apps; do not re-add standalone sample apps.


### prompt-p22-gpui-component-harvest-md-0005-66d075efa171

<!-- ctx-migration source=".prompt/P22-gpui-component-harvest.md" unit="prompt-p22-gpui-component-harvest-md-0005-66d075efa171" sha256="66d075efa171558e73c8db81c535605b38f059f4761add0eaa065d1e371a9e23" -->

## Harvest closure

The full `design/gpui-component-collection-list.md` backlog is closed. Outcomes are one of:

- ✅ Standalone Liora components added where the capability needed a distinct public surface: `Spinner`, `Kbd`, `OtpInput`, `DropdownButton`, `Accordion`, `Sidebar`, `StatusBar`, `DockLayout`, `Settings`, `Sheet`, `HoverCard`, `GroupBox`, `ScrollableMask`, `CandlestickChart`, and `SearchableList`.
- ✅ Existing controls enhanced in place where a parallel component would fragment the API: Combobox-style workflows live in searchable `Select`/`Autocomplete` plus shared `SearchableList`; DataTable capability is folded into `VirtualizedTable`; TextView/document needs are covered by `Text`, `SelectableText`, `CodeBlock`, and the native Docs markdown renderer; WindowExt/TitleBar/WindowBorder capability is covered by `WindowFrame`/`TitleBar`; resizable panel capability is covered by `Splitter`; Toggle-style toolbar and view-mode workflows are covered by `Switch`, `Segmented`, and button-style selections; CodeEditor advanced work remains in the existing `CodeEditor` extension surface.
- ✅ Explicitly not collected where it conflicts with project boundaries or duplicates existing coverage: WebView, WASM gallery, browser runtime paths, and basic controls already present in Liora.


### prompt-p22-gpui-component-harvest-md-0006-2bb8d4878b9d

<!-- ctx-migration source=".prompt/P22-gpui-component-harvest.md" unit="prompt-p22-gpui-component-harvest-md-0006-2bb8d4878b9d" sha256="2bb8d4878b9d01afeecfd66f4b11e78751016848033aed44b7a79bb560afb00a" -->

## Required pattern for any future follow-up

1. Add reusable component source under `crates/liora-components/src/<name>.rs`.
2. Export from `crates/liora-components/src/lib.rs` and keep module docs covered by the public-doc regression.
3. Add Gallery coverage under `apps/liora-gallery/src/demos/` and register it.
4. Add Docs page, external snippet, snippet compile-check import, and live demo mapping.
5. Add focused unit/source tests for non-visual behavior.
6. Run targeted checks before broad workspace gates.


### prompt-p22-gpui-component-harvest-md-0007-da0ec63db61d

<!-- ctx-migration source=".prompt/P22-gpui-component-harvest.md" unit="prompt-p22-gpui-component-harvest-md-0007-da0ec63db61d" sha256="da0ec63db61db9ed44787303ff5742467c754d6eb3a9ace1920eb492ebbfbb56" -->

## Next recommended work

No remaining P22 collection backlog. Future requests that resemble gpui-component items should be treated as ordinary Liora maintenance: enhance the existing closest component in place, keep Gallery/Docs canonical, and do not revive standalone `Combobox` or browser/WebView/WASM paths.

### prompt-p3-popper-feedback-md-0001-46283413ae2c

<!-- ctx-migration source=".prompt/P3-popper-feedback.md" unit="prompt-p3-popper-feedback-md-0001-46283413ae2c" sha256="46283413ae2cc579e84512609d103d02c28866c9efb3b7f66162a671ebdd6f1b" -->

# P3 Popper + Feedback — 弹出层与反馈

> 上游: `.prompt/P2-form-controls.md`


### prompt-p3-popper-feedback-md-0002-4cbf5bbecbbd

<!-- ctx-migration source=".prompt/P3-popper-feedback.md" unit="prompt-p3-popper-feedback-md-0002-4cbf5bbecbbd" sha256="4cbf5bbecbbde175b5b42e86ad6fd12752f164cf8baac14d00e69ad507f5f786" -->

## 目标

攻克原生 GUI 最大的工程难题：弹出层基建 (Popper/Portal) + 全部反馈组件。


### prompt-p3-popper-feedback-md-0003-d7a675871b5f

<!-- ctx-migration source=".prompt/P3-popper-feedback.md" unit="prompt-p3-popper-feedback-md-0003-d7a675871b5f" sha256="d7a675871b5fe5701cbd784a9af5ba0c1ba73b4f64fb39e8367ed8e2d7a693d6" -->

## 弹出层基建 (先于任何 Popup 组件)

| 模块 | 说明 |
|------|------|
| **AnchorPosition** | 锚点定位引擎 (top/bottom/left/right + 12 种偏移对齐) |
| **Portal** | 渲染元素到窗口根节点 (脱离布局流) |
| **ViewportBoundary** | 边缘溢出检测 + 自动翻转方向 |
| **ZIndexStack** | 全局 Z-Index 栈 (popup=+100, modal=+200, notification=+300, tooltip=+400) |
| **ClickOutside** | 点击外部检测关闭 |
| **FocusTrap** | Tab 键焦点锁定 (弹窗内循环) |


### prompt-p3-popper-feedback-md-0004-87bc61a179e6

<!-- ctx-migration source=".prompt/P3-popper-feedback.md" unit="prompt-p3-popper-feedback-md-0004-87bc61a179e6" sha256="87bc61a179e6363310fb8e3b7134cd9083ff7223464a16513120023d787987b5" -->

## 组件清单 (13)

1. **Tooltip** — 文字提示 ⚠️ Popper
2. **Popover** — 气泡卡片 ⚠️ Popper
3. **Popconfirm** — 气泡确认 ⚠️ Popper
4. **Dialog** — 模态对话框 (遮罩、FocusTrap、ESC 关闭)
5. **Drawer** — 抽屉面板 (左/右/上/下)
6. **Message** — 全局消息提示 (顶部居中)
7. **Notification** — 通知 (右上角弹出)
8. **Alert** — 警示提示 (4 种主题)
9. **Loading** — 加载状态 (全屏/局部指令)
10. **MessageBox** — 消息弹窗 (confirm/prompt)
11. **Dropdown** — 下拉菜单 ⚠️ Popper
12. **Card** — 卡片 (header/body/footer)
13. **Collapse** — 折叠面板 (手风琴模式)


### prompt-p3-popper-feedback-md-0005-f81394379f12

<!-- ctx-migration source=".prompt/P3-popper-feedback.md" unit="prompt-p3-popper-feedback-md-0005-f81394379f12" sha256="f81394379f12b1dadbb28573975cef0182d17ccbcdd3199c7159bfb0e8b1c401" -->

## 推荐开发顺序

Popper基建(Anchor+Portal+ZIndex) → Tooltip → Popover → Dialog → Drawer → Message → Notification → Alert → Loading → Card → Collapse → Dropdown → Popconfirm → MessageBox

### prompt-p4-nav-data-md-0001-6256b7296f41

<!-- ctx-migration source=".prompt/P4-nav-data.md" unit="prompt-p4-nav-data-md-0001-6256b7296f41" sha256="6256b7296f41cc25b1fe1d9f1b60aae5ecbac9520b39acab8e6f3ef121ab263e" -->

# P4 Nav + Data — 导航与数据展示

> 上游: `.prompt/P3-popper-feedback.md`


### prompt-p4-nav-data-md-0002-533ee666457e

<!-- ctx-migration source=".prompt/P4-nav-data.md" unit="prompt-p4-nav-data-md-0002-533ee666457e" sha256="533ee666457e9712d804e577ca858358512840214c305cd9454efa34bdc8bceb" -->

## 目标

完成导航组件 9 个 + 数据展示组件 11 个核心 (共 20 个)。


### prompt-p4-nav-data-md-0003-470f45d8f430

<!-- ctx-migration source=".prompt/P4-nav-data.md" unit="prompt-p4-nav-data-md-0003-470f45d8f430" sha256="470f45d8f4300ad514bfe5104f55c71920e7a5a03aac72c0a17e0648c5ccf19d" -->

## Navigation 导航组件 (9)

1. **Menu** — 导航菜单 (垂直/水平, 折叠, router 模式)
2. **Tabs** — 标签页 (下划线动画跟随, 关闭, 新增)
3. **Breadcrumb** — 面包屑 (分隔符, 图标)
4. **Steps** — 步骤条 (横向/纵向, 状态切换, 图标)
5. **PageHeader** — 页头 (标题, 副标题, 返回)
6. **Affix** — 固钉 (滚动吸顶, offset 偏移)
7. **Backtop** — 回到顶部 (滚动监听, 动画)
8. **Anchor** — 锚点链接 (滚动高亮, 容器)
9. **Dropdown** — 已在 P3 完成


### prompt-p4-nav-data-md-0004-88b87a08a31b

<!-- ctx-migration source=".prompt/P4-nav-data.md" unit="prompt-p4-nav-data-md-0004-88b87a08a31b" sha256="88b87a08a31b4fe509b8736c5463f37159b6e1480f86a05f0f1cc955c108aa01" -->

## Data 数据展示 (11)

1. **Progress** — 进度条 (线形/环形, 百分比/状态)
2. **Skeleton** — 骨架屏 (段落/列表/卡片占位)
3. **Empty** — 空状态 (图片 + 描述 + 操作)
4. **Result** — 结果页 (success/warning/error/info)
5. **Descriptions** — 描述列表 (带边框/无边框, 列数)
6. **Timeline** — 时间线 (实心/空心节点, 自定义颜色)
7. **Tree** — 树形控件 (展开/折叠, 勾选, 拖拽)
8. **Pagination** — 分页 (页码, 跳转, 总数)
9. **Statistic** — 统计数值 (前缀/后缀, 动画计数)
10. **Segmented** — 分段控制器 (单选切换, block)
11. **Tag** / **Avatar** / **Badge** — 已在 P1/P2 完成 (基础 Data 组件前移)

### prompt-p5-advanced-md-0001-2d1d540e2885

<!-- ctx-migration source=".prompt/P5-advanced.md" unit="prompt-p5-advanced-md-0001-2d1d540e2885" sha256="2d1d540e28856ddffd701d20a2a3ac1b3413b3cb835308335e9d6496f6dfcb7b" -->

# P5 Advanced — 重型组件

> 上游: `.prompt/P4-nav-data.md`


### prompt-p5-advanced-md-0002-3feb33cb5118

<!-- ctx-migration source=".prompt/P5-advanced.md" unit="prompt-p5-advanced-md-0002-3feb33cb5118" sha256="3feb33cb5118e159051dddd55c126e93dfdaf0c66478802dffa8c52c38756c3c" -->

## 目标

完成最复杂的 20 个组件，含全部企业级必备组件。


### prompt-p5-advanced-md-0003-0e0ea04a3cb3

<!-- ctx-migration source=".prompt/P5-advanced.md" unit="prompt-p5-advanced-md-0003-0e0ea04a3cb3" sha256="0e0ea04a3cb3a4720ff636aaa3d5004a82237a091a43592ccda23ed685c882c8" -->

## 组件清单


### prompt-p5-advanced-md-0004-5f256fd04e3e

<!-- ctx-migration source=".prompt/P5-advanced.md" unit="prompt-p5-advanced-md-0004-5f256fd04e3e" sha256="5f256fd04e3ea21b829288b70fbc006cb229022b56035066c59ea12ca4a8eda0" -->

### 重型 ⚠️ (优先但较慢)
1. **Table** — 表格 (固定表头/列, 排序, 筛选, 选择, 展开行, 合并, 虚拟滚动)
2. **DatePicker** — 日期选择 (日历面板, 范围选择, 快捷选项)
3. **TimePicker** — 时间选择 (固定/任意时间)
4. **DateTimePicker** — 日期时间组合
5. **Upload** — 上传 (拖拽, 列表, 图片预览, 进度)
6. **Cascader** — 级联选择 (多级, 动态加载, 可搜索)
7. **Transfer** — 穿梭框 (左/右列表, 搜索, 拖拽排序)


### prompt-p5-advanced-md-0005-a50c61c6670d

<!-- ctx-migration source=".prompt/P5-advanced.md" unit="prompt-p5-advanced-md-0005-a50c61c6670d" sha256="a50c61c6670df536b07f5d6a5994cfbdcc73f3def61d5e8e0226a7560925fb86" -->

### 中等
8. **ColorPicker** — 颜色选择 (预设色板, 自定义, 透明度)
9. **Carousel** — 走马灯 (自动播放, 指示器, 方向)
10. **Image** — 图片 (懒加载, 预览, 错误占位)
11. **Calendar** — 日历 (日程标记, 范围)
12. **TreeSelect** — 树形选择 (弹出树, 单选/多选)
13. **Autocomplete** — 自动补全 (输入建议, 远程搜索)


### prompt-p5-advanced-md-0006-f6492f9b678b

<!-- ctx-migration source=".prompt/P5-advanced.md" unit="prompt-p5-advanced-md-0006-f6492f9b678b" sha256="f6492f9b678b1761fd6c19a769f919284ebafc85df282d79c524b58c2bffa1a2" -->

### 轻量
14. **InputTag** — 标签输入 (添加/删除标签)
15. **Mention** — @提及 (下拉建议)
16. **Watermark** — 水印 (文字/图片, 密度, 旋转)
17. **Tour** — 漫游引导 (步骤式, 高亮定位)
18. **Scrollbar** — 自定义滚动条 (已在 P1)
19. **Splitter** — 分隔面板 (已在 P1)
20. **VirtualizedTable/VirtualizedTree** — 虚拟化变体 (延后)


### prompt-p5-advanced-md-0007-1b5efdd1854c

<!-- ctx-migration source=".prompt/P5-advanced.md" unit="prompt-p5-advanced-md-0007-1b5efdd1854c" sha256="1b5efdd1854c0d749ad6d585b8489b0f9f223e0de1ba5877c36f1b17778a6817" -->

## Table 组件特别规划

Table 是 Liora 最复杂的单一组件。参考 Element-Plus el-table 的能力矩阵:

| 能力 | 优先级 |
|------|--------|
| 基础渲染 (列定义+行数据) | P0 |
| 固定表头 (sticky header) | P0 |
| 空数据占位 | P0 |
| Loading 状态 | P0 |
| 边框/斑马纹 | P0 |
| 固定列 (左/右) | P1 |
| 排序 (单列/多列) | P1 |
| 筛选 (列过滤) | P1 |
| 选择 (单选/多选) | P1 |
| 展开行 | P2 |
| 合并行/列 | P2 |
| 树形数据 | P2 |
| 虚拟滚动 (万级数据) | P2 |
| 拖拽排序 (行/列) | P3 |
| 编辑单元格 | P3 |
| 导出 CSV | P3 |
| 汇总行 | P3 |



### prompt-p5-advanced-md-0008-59531741a2cf

<!-- ctx-migration source=".prompt/P5-advanced.md" unit="prompt-p5-advanced-md-0008-59531741a2cf" sha256="59531741a2cf0f87b4b0a1857ad877300827bed1ebfa18460100f3a6e1f872f8" -->

## 当前收尾状态（2026-05-10）

P5 当前用户请求范围已结束。已完成的 P5 subset 包括 Table、DatePicker、TimePicker、DateTimePicker、Upload、Cascader、Transfer、ColorPicker、Image、Autocomplete，以及 P1 已完成的 Scrollbar/Splitter。

以下 P5 条目已移入 `.prompt/P9-deferred-advanced.md`，后续需要时再补充：

- Carousel
- Calendar
- TreeSelect
- InputTag
- Mention
- Watermark
- Tour
- VirtualizedTable
- VirtualizedTree

### prompt-p6-builtin-id-md-0001-aeb13ec31fb0

<!-- ctx-migration source=".prompt/P6-builtin-id.md" unit="prompt-p6-builtin-id-md-0001-aeb13ec31fb0" sha256="aeb13ec31fb081916efdc3cb07a0d02c19145c09672b4c010255e6c22ae8dee8" -->

# P6 Built-in Unique ID — 控件内置唯一 ID 规范

> 上游: `.prompt/P5-advanced.md`


### prompt-p6-builtin-id-md-0002-1018f9f81453

<!-- ctx-migration source=".prompt/P6-builtin-id.md" unit="prompt-p6-builtin-id-md-0002-1018f9f81453" sha256="1018f9f814532dd44e98520dd6de2022f67487477eff96263202c7722251d027" -->

## 目标

确保全库每个控件都有默认的内置唯一 ID，事件冲突防护应由组件库自身保证，而非依赖使用者凭良心设置。


### prompt-p6-builtin-id-md-0003-a54265f0f26d

<!-- ctx-migration source=".prompt/P6-builtin-id.md" unit="prompt-p6-builtin-id-md-0003-a54265f0f26d" sha256="a54265f0f26d2d131618a05bcc97e8a0034ce11f85e31cbe4d6ec7af13f4d7cc" -->

## 动机

根据 P0-P5 阶段积累的经验，GPUI 中多个控件实例共用相同的 Element ID 会导致：
- 交互事件冲突（点击无反应、hover 穿透、状态错乱）
- 多个实例只需一个能正常工作
- 开发者需手写 ID 前缀避免冲突，容易遗漏

**组件库的职责是开箱即用、零配置无冲突，ID 唯一性规范必须内建到每个控件中。**


### prompt-p6-builtin-id-md-0004-3b764f958e80

<!-- ctx-migration source=".prompt/P6-builtin-id.md" unit="prompt-p6-builtin-id-md-0004-3b764f958e80" sha256="3b764f958e80e3aaa08d1d1175697dac02de3ee5bb161bddcd535a69f2fb2b7e" -->

## 要求


### prompt-p6-builtin-id-md-0005-71014d33399e

<!-- ctx-migration source=".prompt/P6-builtin-id.md" unit="prompt-p6-builtin-id-md-0005-71014d33399e" sha256="71014d33399e661a61c8c0de72888b947f6b5acf68d5b6260c764bf553bfab74" -->

### 1. 每个控件必须有默认内置唯一 ID

```rust
// ✅ 正确 — Button 内部自动生成全局唯一 ID
Button::new("Save").primary()

// ❌ 禁止 — 依赖开发者手动设置唯一 ID
Button::new("Save").primary().id("my-unique-id")
```


### prompt-p6-builtin-id-md-0006-ed9fce6b465f

<!-- ctx-migration source=".prompt/P6-builtin-id.md" unit="prompt-p6-builtin-id-md-0006-ed9fce6b465f" sha256="ed9fce6b465f69b49b5768f6c8ea5fd7be46122d9498b49537019267f7176fd3" -->

### 2. 全局唯一，非仅局部唯一

- 不能仅基于调用位置 (`track_caller`) 生成 ID — 同一个 helper 函数中循环创建多个实例时 ID 相同
- 不能仅基于组件名称 — 同类型组件多实例冲突
- 必须结合运行时唯一标识（如 UUID、atomic counter、EntityId 等）


### prompt-p6-builtin-id-md-0007-531b86abffcc

<!-- ctx-migration source=".prompt/P6-builtin-id.md" unit="prompt-p6-builtin-id-md-0007-531b86abffcc" sha256="531b86abffcc099354a030c004a10534b6cf9e77a81aea2e870b2373ababa875" -->

### 3. 实现策略（分层）

| 优先级 | 策略 | 适用场景 |
|--------|------|---------|
| A | 基于 `EntityId` 生成 | 有状态组件（View-based），每个 entity 有天然唯一 ID |
| B | 基于 `std::sync::atomic::AtomicU64` 全局递增计数器 | 无状态组件（RenderOnce），每次构造分配唯一序号 |
| C | 基于 UUID (`uuid::Uuid`) | 需要跨会话/跨窗口唯一性的场景 |
| D | 暴露 `.id(impl Into<SharedString>)` 作为可覆盖项 | 用户需要显式指定 ID 时 |


### prompt-p6-builtin-id-md-0008-d95f6e8072f2

<!-- ctx-migration source=".prompt/P6-builtin-id.md" unit="prompt-p6-builtin-id-md-0008-d95f6e8072f2" sha256="d95f6e8072f257c6881df3c215bf3dbbc8e4c14902dcf6f0d69fe5f33df28560" -->

### 4. 组件内交互子元素 ID 前缀

每个组件内部的交互子元素（按钮、图标、输入框等）必须以组件唯一 ID 为前缀：

```rust
let component_id = format!("button-{}", self.uid);
// 内部子元素:
//   "{component_id}-icon-start"
//   "{component_id}-icon-end"
//   "{component_id}-text"
```


### prompt-p6-builtin-id-md-0009-671b60a874d1

<!-- ctx-migration source=".prompt/P6-builtin-id.md" unit="prompt-p6-builtin-id-md-0009-671b60a874d1" sha256="671b60a874d1a0a4506a5bda3d16e0a5a31ece64dcc042e1ec2586b1297bed16" -->

### 5. 全局计数器基础设施

在 `liora-core` 中新增全局 ID 生成器：

```rust
// crates/liora-core/src/lib.rs 或新建 crates/liora-core/src/unique_id.rs
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// 生成全局唯一递增序号
pub fn next_unique_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// 生成带前缀的唯一 ID 字符串
pub fn unique_id(prefix: &str) -> SharedString {
    format!("{}-{}", prefix, next_unique_id()).into()
}
```


### prompt-p6-builtin-id-md-0010-776ce74d58dd

<!-- ctx-migration source=".prompt/P6-builtin-id.md" unit="prompt-p6-builtin-id-md-0010-776ce74d58dd" sha256="776ce74d58dd7cd8d700e8eed9b55a986cb0243deefe1a6241b8a1576c60f3d3" -->

## 检查清单

对 `crates/liora-components/src/` 下每一个组件文件，逐项检查：

- [ ] 是否有默认内置唯一 ID？
- [ ] 内部交互子元素 ID 是否以组件 ID 为前缀？
- [ ] 多实例共存时是否存在 ID 冲突？
- [ ] `#[track_caller]` 默认 ID 是否在循环/helper 函数中能保持唯一？
- [ ] 是否提供 `.id(custom_id)` 覆盖入口（非强制，但建议）？


### prompt-p6-builtin-id-md-0011-ed28f4744570

<!-- ctx-migration source=".prompt/P6-builtin-id.md" unit="prompt-p6-builtin-id-md-0011-ed28f4744570" sha256="ed28f4744570a9b5440ff6e0d60d575d2a72aa675faa905239cbb0a4bdfa10ad" -->

## 涉及文件范围

- `crates/liora-core/src/` — 新增全局 ID 生成器
- `crates/liora-components/src/*.rs` — 所有现有组件逐一改造
- `crates/liora-components/src/lib.rs` — 确保 unique_id 模块可访问


### prompt-p6-builtin-id-md-0012-97f4c2cb12b5

<!-- ctx-migration source=".prompt/P6-builtin-id.md" unit="prompt-p6-builtin-id-md-0012-97f4c2cb12b5" sha256="97f4c2cb12b5bd7ffda163b55a35bbf404e6df45b4b90dce09136183833c7a83" -->

## 验证标准

1. `cargo check` 0 errors, 0 warnings
2. Gallery 中同类型多实例控件交互互不干扰
3. 同一个 demo helper 函数中循环创建的控件每个都正常工作

### prompt-p7-demo-self-contained-md-0001-9090f0340d23

<!-- ctx-migration source=".prompt/P7-demo-self-contained.md" unit="prompt-p7-demo-self-contained-md-0001-9090f0340d23" sha256="9090f0340d23f980098ff619773a68cc471548981885e6fff87343dd47252eab" -->

# P7 Demo Self-Contained — Demo 完全自举

> 上游: `.prompt/P6-builtin-id.md`


### prompt-p7-demo-self-contained-md-0002-32b191cd9e5c

<!-- ctx-migration source=".prompt/P7-demo-self-contained.md" unit="prompt-p7-demo-self-contained-md-0002-32b191cd9e5c" sha256="32b191cd9e5c62b2db39c8c316eacbbb24468b5cb6f345f15839a958ef95cb74" -->

## 目标

Gallery Demo 完全使用 Liora 组件库自身控件构建，避免在 Demo 中直接使用 GPUI 原生组件。若发现需要的控件缺失，应自行新增到组件库中。


### prompt-p7-demo-self-contained-md-0003-aea0ea803eec

<!-- ctx-migration source=".prompt/P7-demo-self-contained.md" unit="prompt-p7-demo-self-contained-md-0003-aea0ea803eec" sha256="aea0ea803eec4b03c6bee6dd47986c5801ac4d69d713d72e5989800b6bef1b87" -->

## 动机

当前 Gallery Demo 中存在大量直接使用 `div()`、`div().flex()` 等 GPUI 原生 API 的情况，这导致：
- **Demo 无法展示组件库真实能力** — 用户看到的是 GPUI 用法，不是 Liora 用法
- **组件库缺失信号** — Demo 中手写的布局/样式模式没有沉淀为可复用组件
- **风格不一致** — 混用原生 GPUI 和 Liora 控件导致 Demo 外观不统一
- **"吃自己的狗粮"缺失** — 组件库自身不用自己的组件，难以发现 API 问题


### prompt-p7-demo-self-contained-md-0004-3b764f958e80

<!-- ctx-migration source=".prompt/P7-demo-self-contained.md" unit="prompt-p7-demo-self-contained-md-0004-3b764f958e80" sha256="3b764f958e80e3aaa08d1d1175697dac02de3ee5bb161bddcd535a69f2fb2b7e" -->

## 要求


### prompt-p7-demo-self-contained-md-0005-b8644de1e5fe

<!-- ctx-migration source=".prompt/P7-demo-self-contained.md" unit="prompt-p7-demo-self-contained-md-0005-b8644de1e5fe" sha256="b8644de1e5fec3bdf47efd680a62d72eb7c2c2ff97680c85c5b788165e8283a4" -->

### 1. Demo 中禁止直接使用 GPUI 原生布局/样式 API

```rust
// ❌ 禁止 — 直接使用 div() 构建 demo 布局
fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
    div().flex().flex_col().gap_4()
        .child(div().text_xl().font_weight(FontWeight::BOLD).child("Title"))
        .child(div().child("Content"))
}

// ✅ 正确 — 使用 Liora 组件构建完整 demo
fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
    Container::new()
        .child(Title::new("Title").level(2))
        .child(Paragraph::new().with_text("Content"))
}
```


### prompt-p7-demo-self-contained-md-0006-1ad81f912a9b

<!-- ctx-migration source=".prompt/P7-demo-self-contained.md" unit="prompt-p7-demo-self-contained-md-0006-1ad81f912a9b" sha256="1ad81f912a9ba6ab3556ae7df3b0580ecc5f79f4b22f06e85023fe7d25a89f92" -->

### 2. 缺失控件应新增到组件库

如果 Demo 需要某种布局/样式模式但组件库中没有对应控件，应按以下优先级处理：

| 优先级 | 处理方式 | 适用场景 |
|--------|---------|---------|
| 1 | 使用现有 Liora 控件组合 | 能用 Space + Container + Text 等已有控件拼出来的效果 |
| 2 | 新增通用控件到组件库 | 确实缺失且具有复用价值的控件 |
| 3 | 扩展现有控件能力 | 现有控件已存在但缺少某个 builder 方法 |


### prompt-p7-demo-self-contained-md-0007-4698d97f0fd7

<!-- ctx-migration source=".prompt/P7-demo-self-contained.md" unit="prompt-p7-demo-self-contained-md-0007-4698d97f0fd7" sha256="4698d97f0fd75fc4f56739b75ee1bf59043f9801f93b1c79168c624a17d629b7" -->

### 3. Demo 辅助函数规范

Demo 中允许使用纯函数 helper，但这些 helper 必须：
- 返回 Liora 控件（`Container`, `Space`, `Text`, `Divider` 等）
- 不直接调用 `div()`, `px()`, `rgb()` 等 GPUI 原语

```rust
// ✅ 正确的 demo helper — 使用 Liora 控件
fn section_header(title: &str) -> Container {
    Container::new()
        .child(Title::new(title).level(3))
        .child(Divider::new())
}

// ❌ 错误的 demo helper — 直接使用 GPUI 原语
fn section_header(title: &str) -> Div {
    div().flex().flex_col()
        .child(div().text_lg().font_weight(FontWeight::BOLD).child(title))
        .child(div().h_1px().bg(rgb(0xdcdfe6)))
}
```


### prompt-p7-demo-self-contained-md-0008-ff940be83d01

<!-- ctx-migration source=".prompt/P7-demo-self-contained.md" unit="prompt-p7-demo-self-contained-md-0008-ff940be83d01" sha256="ff940be83d01960e72b847ff4107bc93ec58e22921c385a8e9b16b3cc7b6857d" -->

### 4. 现有 Demo 改造范围

逐文件审查 `apps/liora-gallery/src/demos/*_demo.rs`：

- [ ] `button_demo.rs`
- [ ] `link_demo.rs`
- [ ] `text_demo.rs`
- [ ] `title_demo.rs`
- [ ] `paragraph_demo.rs`
- [ ] `space_demo.rs`
- [ ] `divider_demo.rs`
- [ ] `row_demo.rs`
- [ ] `col_demo.rs`
- [ ] `container_demo.rs`
- [ ] `scrollbar_demo.rs`
- [ ] `splitter_demo.rs`
- [ ] `button_group_demo.rs`
- [ ] `input_demo.rs`
- [ ] `input_number_demo.rs`
- [ ] `textarea_demo.rs`
- [ ] `checkbox_demo.rs`
- [ ] `radio_demo.rs`
- [ ] `switch_demo.rs`
- [ ] `select_demo.rs`
- [ ] `slider_demo.rs`
- [ ] `form_demo.rs`
- [ ] `rate_demo.rs`
- [ ] `tooltip_demo.rs`
- [ ] `popover_demo.rs`
- [ ] `popconfirm_demo.rs`
- [ ] `dialog_demo.rs`
- [ ] `drawer_demo.rs`
- [ ] `message_demo.rs`
- [ ] `notification_demo.rs`
- [ ] `alert_demo.rs`
- [ ] `loading_demo.rs`
- [ ] `message_box_demo.rs`
- [ ] `dropdown_demo.rs`
- [ ] `card_demo.rs`
- [ ] `collapse_demo.rs`
- [ ] `menu_demo.rs`
- [ ] `tabs_demo.rs`
- [ ] `breadcrumb_demo.rs`
- [ ] `steps_demo.rs`
- [ ] `page_header_demo.rs`
- [ ] `affix_demo.rs`
- [ ] `backtop_demo.rs`
- [ ] `anchor_demo.rs`
- [ ] `progress_demo.rs`
- [ ] `skeleton_demo.rs`
- [ ] `empty_demo.rs`
- [ ] `result_demo.rs`
- [ ] `descriptions_demo.rs`
- [ ] `timeline_demo.rs`
- [ ] `tree_demo.rs`
- [ ] `pagination_demo.rs`
- [ ] `statistic_demo.rs`
- [ ] `segmented_demo.rs`
- [ ] `tag_demo.rs`
- [ ] `avatar_demo.rs`
- [ ] `badge_demo.rs`

以及核心框架文件：
- [ ] `apps/liora-gallery/src/main.rs`
- [ ] `apps/liora-gallery/src/category.rs`


### prompt-p7-demo-self-contained-md-0009-7bb6112ef95a

<!-- ctx-migration source=".prompt/P7-demo-self-contained.md" unit="prompt-p7-demo-self-contained-md-0009-7bb6112ef95a" sha256="7bb6112ef95a6a7849ed6cca0880fef8ad5f115ca2999026c4dae91c5f10d080" -->

### 5. 可能需要新增的控件

以下控件可能在 Demo 改造中发现缺失，需要新增：

| 潜在缺失控件 | 用途 | 优先级 |
|-------------|------|--------|
| `PageLayout` / `PageContainer` | Demo 页面级容器（header + body + footer 模式） | 高 |
| `FlexRow` / `FlexCol` | 语义化的 flex 布局容器（如果 Space 不足以覆盖） | 中 |
| `DemoBlock` | 代码演示用的卡片容器（title + description + preview） | 高 |
| `ColorSwatch` | 主题色板展示 | 低 |


### prompt-p7-demo-self-contained-md-0010-0817001055a9

<!-- ctx-migration source=".prompt/P7-demo-self-contained.md" unit="prompt-p7-demo-self-contained-md-0010-0817001055a9" sha256="0817001055a93f42241fa2c4a5d1413cb5672bf30e4ee4fc8f353ae5081c22ba" -->

### 6. The Order Of Components in demo

Must be Ordered by Component's name with dictionary, ASC


### prompt-p7-demo-self-contained-md-0011-ea83a4f68b8b

<!-- ctx-migration source=".prompt/P7-demo-self-contained.md" unit="prompt-p7-demo-self-contained-md-0011-ea83a4f68b8b" sha256="ea83a4f68b8b9801527fe2610074e53d3b5fbb62d675447a2971dcc274fc044c" -->

## 验证标准

1. `cargo check` 0 errors, 0 warnings
2. 搜索 `apps/liora-gallery/src/demos/` 下所有文件，无直接 `div().flex().flex_col()` 等 GPUI 布局原语（仅 `liora-components` 内部实现可保留）
3. Gallery 运行正常，所有 Demo 页面视觉一致
4. Gallery 的 `category.rs` 和 `main.rs` 也使用 Liora 控件

### prompt-p8-engineering-md-0001-d5d95c54df59

<!-- ctx-migration source=".prompt/P8-engineering.md" unit="prompt-p8-engineering-md-0001-d5d95c54df59" sha256="d5d95c54df5968e54e70b490b443fc99da4af834e2d1e25e20924583a269786c" -->

# P8 Engineering — Liora Docs 主程序

> 上游: `.prompt/P7-demo-self-contained.md`
> 下游: `.prompt/P9-deferred-advanced.md`（deferred backlog，非自动执行）


### prompt-p8-engineering-md-0002-38b91e67ba01

<!-- ctx-migration source=".prompt/P8-engineering.md" unit="prompt-p8-engineering-md-0002-38b91e67ba01" sha256="38b91e67ba01873a6f4cde878f11b9d38f799b4919cfa42d27b7d6176d532462" -->

## 目标

将 Liora Gallery 保持为“组件看板”，并把 Liora UI 的官方文档独立为 `liora-docs` 主程序。

P8 不再建设 VitePress/Web 文档站。新的技术路线是：所有文档、示例、导航、Markdown 渲染和活体组件 Demo 都运行在 **GPUI 原生视窗** 内。


### prompt-p8-engineering-md-0003-be819357d5c6

<!-- ctx-migration source=".prompt/P8-engineering.md" unit="prompt-p8-engineering-md-0003-be819357d5c6" sha256="be819357d5c6560cf3fbe3ba8eeb0d83821236c98b233664bbe3c1dcc39c22ad" -->

## 角色与上下文

你是一位 Rust 与 GPUI 原生框架架构师，负责从 0 到 1 建设 Liora Docs。Liora Docs 是 Liora UI（基于 GPUI 的原生企业级组件库）的官方文档主程序，Liora Gallery 则继续承担组件看板职责。


### prompt-p8-engineering-md-0004-e06f4e20e491

<!-- ctx-migration source=".prompt/P8-engineering.md" unit="prompt-p8-engineering-md-0004-e06f4e20e491" sha256="e06f4e20e49121cb31f9fc170fc65b5f5c12fe609486c6b2479a64e8d93a5ac6" -->

### 绝对禁令

- 本项目 100% 运行在 GPUI 原生窗口中。
- 严禁引入 Web 文档站、跨端转译、浏览器运行时或网页渲染路径。
- 文档渲染必须基于 Rust 数据结构、GPUI 元素树、Liora 组件、Flex 布局和原生渲染能力。
- 如遇 GPUI API 差异或废弃，优先查阅本地 `gpui` 源码和当前 repo 的既有用法，不凭记忆猜测。


### prompt-p8-engineering-md-0005-d119fe5e0d05

<!-- ctx-migration source=".prompt/P8-engineering.md" unit="prompt-p8-engineering-md-0005-d119fe5e0d05" sha256="d119fe5e0d055093d1453f70a9a054f3dd79f52f1fa552a9afc2fc4fef40d58e" -->

## 技术栈

| 层级 | 选择 | 说明 |
|------|------|------|
| UI Framework | `gpui`（沿用 workspace 当前版本/特性） | Liora Gallery 唯一渲染运行时 |
| Markdown AST | `pulldown-cmark` | 只负责 Markdown 解析与事件/AST 生成 |
| 文档渲染 | Liora Typography + Layout 组件 | 排版、折行、样式由 Liora 自举组件负责 |
| 文档内容 | `apps/liora-docs/content/pages/*.md` | 每个文档/控件一份 Markdown |
| 示例代码 | `apps/liora-docs/content/snippets/<page>/*.rs` | 代码与 Markdown 分离，按文件命名约定关联 |
| Live Demo 注入 | GPUI/Liora 真实 View Node | 特殊语法直接插入可交互组件 |
| Language | Rust（最低语义基线 2021；当前 workspace 保持现有 edition 2024） | 不因 P8 文档方案回退 Cargo edition |


### prompt-p8-engineering-md-0006-ce8fa152d13d

<!-- ctx-migration source=".prompt/P8-engineering.md" unit="prompt-p8-engineering-md-0006-ce8fa152d13d" sha256="ce8fa152d13d7f2ba95fa9b6e470b827cff2ad22ff5a65dae1771f864a66126a" -->

## Architecture Core: Bootstrapping（自举）

文档渲染的核心挑战是富文本折行（Word Wrap）。P8 采用“自举”策略：

1. `pulldown-cmark` 只负责 Markdown 解析，输出事件/AST。
2. Markdown renderer 不实现复杂外部排版引擎。
3. 所有排版、折行、颜色、粗细、行内样式和块级布局，必须交给 Liora 自己封装的 Typography/Layout 组件处理。
4. `LioraText` / `LioraParagraph` 是最核心的基础设施：多个不同样式的文本片段必须能在同一个段落容器中流式拼接、自动换行，且不可截断。

> 命名说明：现有代码已经有 `Text` / `Paragraph`。P8 实施时应优先评估是扩展现有组件，还是新增富文本专用类型；若新增，公共命名仍遵循 ADR-009（不加 `Liora` 前缀），可使用 `RichText` / `RichParagraph` 等不冲突名称。


### prompt-p8-engineering-md-0007-e1f7080eb397

<!-- ctx-migration source=".prompt/P8-engineering.md" unit="prompt-p8-engineering-md-0007-e1f7080eb397" sha256="e1f7080eb397c85c2b41a6c7beeddca11036511ef68e18f5dfdfedf19ec81015" -->

## 执行阶段

必须按以下 4 个子阶段顺序执行。每个子阶段完成后，必须确保 `cargo check` 无错误，并运行相关测试后再进入下一个子阶段。


### prompt-p8-engineering-md-0008-e6a5fe87154d

<!-- ctx-migration source=".prompt/P8-engineering.md" unit="prompt-p8-engineering-md-0008-e6a5fe87154d" sha256="e6a5fe87154d7e9a0eade16a8e938d3fe55c54276c5e06baaf74c73c5aa45876" -->

### Phase 1: 基础设施搭建 & Typography 组件（核心）

- [x] 确认项目结构包含并继续使用：
  - `crates/liora-components`
  - `apps/liora-gallery`
- [x] 在 `liora-components` 中实现/扩展富文本文本片段组件：
  - 封装 GPUI 文本样式：字体粗细、颜色、背景色、等宽字体、行内代码样式等。
  - 可作为段落内的 style run，而不是只能作为独立块元素。
- [x] 在 `liora-components` 中实现/扩展富文本段落组件：
  - 接收一个或多个文本片段作为子节点。
  - 底层必须使用 GPUI `StyledText` 或当前版本等价机制。
  - 不同样式文本片段拼接后，必须在同一容器内正确流式布局并自动换行。
  - 长文本不可被截断；如 GPUI API 限制存在，必须记录限制并提供最小可验证替代方案。
- [x] 为 Typography 行为添加最小回归测试：样式片段拼接、粗体/斜体/行内代码状态、长文本换行容器不截断。


### prompt-p8-engineering-md-0009-aab0e7c44b57

<!-- ctx-migration source=".prompt/P8-engineering.md" unit="prompt-p8-engineering-md-0009-aab0e7c44b57" sha256="aab0e7c44b571390fe648252c5c69b7a12f71477fe71a51053314c23b063ea0b" -->

### Phase 2: Markdown 解析引擎与状态机

- [x] 在 `liora-gallery` 中引入 `pulldown-cmark`。
- [x] 新建 `apps/liora-docs/src/markdown.rs`。
- [x] 实现：`render_markdown(md_text: &str) -> gpui::AnyElement`。
- [x] 使用 `Vec` 作为栈管理层级：
  - 遇到块级元素开始（Heading、Paragraph、List、BlockQuote 等）时，将对应 Liora 容器压入栈。
  - 遇到内联元素开始/结束（Strong、Emphasis、Code 等）时，更新当前文本样式上下文。
  - 遇到纯文本时，根据当前上下文生成文本片段，并添加到栈顶容器。
  - 遇到块级元素结束时，将栈顶容器弹出，并作为子节点添加到新的栈顶容器。
- [x] 添加 Markdown renderer 回归测试：标题、段落、粗体、斜体、列表、嵌套块级结构。


### prompt-p8-engineering-md-0010-adea328350ba

<!-- ctx-migration source=".prompt/P8-engineering.md" unit="prompt-p8-engineering-md-0010-adea328350ba" sha256="adea328350bafeaca610906d2afd8b3c3a51a6182abb91f5bd9ee57d19156f04" -->

### Phase 3: 代码块与样式打磨

- [x] 完善 `Start(Tag::CodeBlock)` 映射：
  - 使用 Liora 容器提供浅/深主题兼容的灰色背景。
  - 使用等宽字体。
  - 使用 `Scrollbar` 或现有 Liora 滚动容器提供水平滚动。
- [x] 完善行内代码：
  - 等宽字体。
  - 浅色背景/圆角/内边距。
  - 不破坏段落流式换行。
- [x] 构建 Liora Gallery 双栏文档窗口：
  - 左侧：文档导航树/组件目录。
  - 右侧：Markdown 渲染结果区。
  - 右侧支持垂直滚动。
  - 整体仍使用 Liora `Container` / `Menu` / `Flex` / `Scrollbar` 等原生组件。
- [x] 将文档正文拆分为 `apps/liora-docs/content/pages/<page>.md`：
  - 组件文档按单个控件拆分，例如 `button.md`、`input.md`、`switch.md`。
  - 非组件页面使用 snake_case，例如 `quick_start.md`、`live_demo.md`。
- [x] 将代码示例拆分为外部 `.rs` 片段：
  - 统一路径：`apps/liora-docs/content/snippets/<page>/<case>.rs`。
  - Markdown 代码块通过 fenced info 引用：<code>```rust src="button/types.rs"</code>。
  - `src` 路径相对于 `content/snippets/`，由 `markdown.rs` 使用编译期 `include_str!` 映射加载。


### prompt-p8-engineering-md-0011-8ba30bcf7917

<!-- ctx-migration source=".prompt/P8-engineering.md" unit="prompt-p8-engineering-md-0011-8ba30bcf7917" sha256="8ba30bcf79177111c92c336b479fbb97a167c087a9023a1bc9371c5bd9a3e93c" -->

### Phase 4: 活体组件注入（Live Demo）

- [x] 在 Markdown 事件处理过程中识别特殊语法：
  - `::LioraDemo{component="Button"}::`
- [x] 识别到该语法时，不渲染为普通文字。
- [x] 根据 `component` 实例化真实 Liora 组件，例如 `Button::new("Button").primary()`。
- [x] 将真实 GPUI/Liora view node 直接插入文档流。
- [x] Live Demo 必须保留 hover/click 等真实交互能力。
- [x] 添加回归测试：特殊语法不会出现在最终文本中，并能映射到对应 demo 节点。


### prompt-p8-engineering-md-0012-b150feb624db

<!-- ctx-migration source=".prompt/P8-engineering.md" unit="prompt-p8-engineering-md-0012-b150feb624db" sha256="b150feb624dba3f40b1bcbfecdc5f8f7fb8a2dabdad0ff89cc127140791a9557" -->

## 保留的工程化任务

原 P8 的工程化事项仍保留，但顺序调整到原生文档大屏基础完成之后：

- [ ] Gallery 主题切换按钮（light/dark 一键切换）
- [ ] 组件搜索（输入过滤）
- [ ] 窗口标题：`Liora Gallery — Native Component Library`
- [ ] 测试体系：单元测试、组件测试、`cargo test`
- [ ] CI/CD：`cargo check` / `cargo clippy` / `cargo test` / `cargo doc`
- [ ] 发布：CHANGELOG、SemVer、LICENSE、crates.io 发布策略
- [ ] 社区文档：CONTRIBUTING、CODE_OF_CONDUCT、README


### prompt-p8-engineering-md-0013-94046a4148ae

<!-- ctx-migration source=".prompt/P8-engineering.md" unit="prompt-p8-engineering-md-0013-94046a4148ae" sha256="94046a4148aefe9d38808b542f96648d5c784a214358032714b69f68c73c0e4f" -->

## 明确废弃的旧方案

- [x] 不再搭建 VitePress/Web 文档站；`apps/liora-docs` 保持 GPUI 原生主程序。
- [x] 不再把官方文档作为网页产物维护。
- [x] 不再将 Markdown 渲染外包给网页/浏览器/跨端运行时。

### prompt-p9-deferred-advanced-md-0001-d376532d3757

<!-- ctx-migration source=".prompt/P9-deferred-advanced.md" unit="prompt-p9-deferred-advanced-md-0001-d376532d3757" sha256="d376532d3757cbddd27abc01bcb25b15dcdc9b03ea8807a91e210dce44a428f1" -->

# P9 Deferred Advanced — 延后高级组件补全

> 上游: `.prompt/P8-engineering.md`
> 来源: P5 Advanced 中用户明确要求跳过/延后的组件；本阶段是最新阶段，作为后续需要时补充的 backlog。


### prompt-p9-deferred-advanced-md-0002-2d680f0c80c9

<!-- ctx-migration source=".prompt/P9-deferred-advanced.md" unit="prompt-p9-deferred-advanced-md-0002-2d680f0c80c9" sha256="2d680f0c80c9255ca4f3b2674e7a9b5f9f176683c410a18455d0cd76fa142344" -->

## 目标

在 P5 当前请求范围完成后，集中补全被跳过或延后的高级组件。此阶段暂不自动执行；只有用户明确要求回到这些组件时才启动。


### prompt-p9-deferred-advanced-md-0003-84d59402f13b

<!-- ctx-migration source=".prompt/P9-deferred-advanced.md" unit="prompt-p9-deferred-advanced-md-0003-84d59402f13b" sha256="84d59402f13bd9f972fdc838fd087a1db4bb71816b31e0ff59929f9183b4ce49" -->

## 启动条件

- 用户明确要求实现本阶段任一组件；或
- P6/P7/P8 工程化、Demo 自包含、文档/API 梳理后，需要补齐剩余高级组件；或
- 产品需求重新确认这些组件进入当前交付范围。


### prompt-p9-deferred-advanced-md-0004-c9366a1697b7

<!-- ctx-migration source=".prompt/P9-deferred-advanced.md" unit="prompt-p9-deferred-advanced-md-0004-c9366a1697b7" sha256="c9366a1697b7532f61738711eb06916c8363915f894d39dc64cb2cd9c21dba14" -->

## 组件清单

| # | Component | 中文名 | 来源/状态 | 后续补充方向 |
|---|-----------|--------|-----------|--------------|
| 1 | Carousel | 走马灯 | P5 用户要求跳过，已识别 | 自动播放、手动切换、指示器、方向、暂停/恢复、Demo |
| 2 | Calendar | 日历 | P5 deferred | 月视图、日期单元格、事件/日程标记、范围/禁用日期、Demo |
| 3 | TreeSelect | 树形选择 | P5 deferred | 弹出树、单选/多选、搜索、禁用节点、默认值、Demo |
| 4 | InputTag | 标签输入 | P5 deferred | 输入生成标签、删除、限制数量、重复校验、禁用、Demo |
| 5 | Mention | @提及 | P5 deferred | 触发符、建议列表、键盘选择、插入文本、Demo |
| 6 | Watermark | 水印 | P5 deferred | 文字/图片水印、密度、旋转、透明度、覆盖区域、Demo |
| 7 | Tour | 漫游引导 | P5 deferred | 步骤、目标定位、高亮遮罩、下一步/上一步、关闭、Demo |
| 8 | VirtualizedTable | 虚拟表格 | P5 deferred | 大数据行虚拟化、固定表头、滚动性能、与 Table API 对齐 |
| 9 | VirtualizedTree | 虚拟树 | P5 deferred | 大树虚拟化、展开/折叠、滚动性能、与 Tree API 对齐 |


### prompt-p9-deferred-advanced-md-0005-99860f5d3309

<!-- ctx-migration source=".prompt/P9-deferred-advanced.md" unit="prompt-p9-deferred-advanced-md-0005-99860f5d3309" sha256="99860f5d330927da90e5170ee0e4aeeb3b341f3f77a3f4e96d11f888c5a53a50" -->

## 实施规则

1. **不要把 P9 当作当前自动执行阶段。** 它是最新的 deferred backlog，等待后续用户明确要求。
2. 每个组件仍遵守 Liora 组件流程：
   - `crates/liora-components/src/<name>.rs`
   - `crates/liora-components/src/lib.rs` 注册 `pub mod` / `pub use`
   - `apps/liora-gallery/src/demos/<name>_demo.rs`
   - `apps/liora-gallery/src/demos/mod.rs` 注册 DemoEntry
   - 测试覆盖核心状态/过滤/边界逻辑
3. 优先复用现有组件能力：Input、Popover/Portal、Tree、Table、Scrollbar、Button、Icon。
4. 对复杂组件先做最小可用版本，再扩展高级 API；避免一次性引入不可验证的大实现。
5. 每完成一个 P9 组件，更新：
   - `.memory/inventory.md`
   - `.memory/state.md`
   - `.memory/sessions.md`
   - 如有设计取舍，更新 `.memory/decisions.md`


### prompt-p9-deferred-advanced-md-0006-d14fd56143d8

<!-- ctx-migration source=".prompt/P9-deferred-advanced.md" unit="prompt-p9-deferred-advanced-md-0006-d14fd56143d8" sha256="d14fd56143d8f3a0281f77646ba76311418ebd4f9e40ca6b012b6abf2a52c20a" -->

## 当前状态

- P9 created on 2026-05-10.
- 所有条目均为 deferred / identified for later。
- P5 requested subset 已完成；P9 等待后续补充。

### prompt-md-0001-09dbcb135635

<!-- ctx-migration source="prompt.md" unit="prompt-md-0001-09dbcb135635" sha256="09dbcb13563506f0daf3673ee835b5fc81b2e840bf89ccabaa15446861ae1b8f" -->

# Liora UI — AI Development Prompt

> **用途**: 本文件供任何 AI 开发工具（OpenCode / Claude / Gemini / Codex / Cursor 等）在接手 Liora 项目时读取，确保上下文连贯、开发风格一致。

---


### prompt-md-0002-1dfc7a3b98bf

<!-- ctx-migration source="prompt.md" unit="prompt-md-0002-1dfc7a3b98bf" sha256="1dfc7a3b98bf843cc9336453602e055dac322a04f22bd769da67d74b3f4721f0" -->

## 1. 项目速览

**Liora** 是一套基于 [GPUI](https://github.com/zed-industries/zed) 的企业级 Rust Native 组件库，参照 [Element-Plus](https://element-plus.org/zh-CN/) 的 API 规范和组件分类体系。目标是为 Rust 桌面应用提供开箱即用的高级 UI 控件。

| 属性 | 值 |
|------|-----|
| 语言 | Rust edition 2024 |
| UI 框架 | GPUI（官方 `zed-industries/zed` git 依赖，按 `Cargo.toml`/`Cargo.lock` pin 到明确 revision） |
| 参考规范 | Element-Plus 2.x (https://element-plus.org/zh-CN/) |
| 架构 | Cargo Workspace Monorepo |
| 目标 | ~76+ 个企业级组件, 分阶段交付；P9 作为延后高级组件补全 backlog；P10 原生统计图组件；P11 系统托盘/进程常驻阶段；P12 原生安装器打包阶段；P13 高级控件扩展阶段；P14 延后高级组件补全阶段；P15 质量收口阶段；P16 对外采用准备阶段；P17 真实 Dashboard Dogfooding 阶段；P18 Dashboard Polish/API Ergonomics 阶段；P19 Dashboard State/Data Flow 阶段；P20 Theme/Interaction Polish 阶段；P21 Release Candidate Readiness 阶段 |

---


### prompt-md-0003-c9a230b4fa5b

<!-- ctx-migration source="prompt.md" unit="prompt-md-0003-c9a230b4fa5b" sha256="c9a230b4fa5b5d626c5eabd13de02631ecf49e30bc931cda54d2bac536afeb7b" -->

## 2. 关键文档索引

| 文档 | 路径 | 用途 |
|------|------|------|
| 架构设计 | `design/architecture-design.md` | 完整技术方案、组件清单、Token 体系、里程碑 |
| Release Candidate Checklist | `docs/release-candidate-checklist.md` | P21 0.2.x RC 本地 gate、metadata audit 与 protected release 边界 |
| 工程结构 | `structure.txt` | Workspace 目录树速览 |
| 初始调研 | `chat.txt` | Gemini 技术可行性分析 |
| **记忆库** | `.memory/` | 当前状态、架构决策、组件清单、会话历史 |
| **阶段提示词** | `.prompt/` | 各阶段开发指令，链式继承 |
| **P9 延后高级组件** | `.prompt/P9-deferred-advanced.md` | P5 跳过/延后的高级组件 backlog，后续需要时补充 |
| **P10 统计图组件** | `.prompt/P10-charts.md` | 原生 GPUI 统计图控件：Line/Area/Bar/Pie/Ring/Sparkline/Axis/Grid/Legend/Tooltip |
| **P11 托盘常驻** | `.prompt/P11-tray.md` | 已完成：`liora-tray` 跨平台系统托盘、动态图标、CheckBox/N 级菜单、关闭隐藏驻留与 GPUI 命令桥接 |
| **P12 原生打包** | `.prompt/P12-packaging.md` | ✅ 仓库内闭环完成：`liora-packager` / `xtask package` / `packaging/` / CI installer pipeline / release-readiness gate / signing policy / LicenseRef-Liora policy 已落地；真实签名凭据和系统级安装执行由受保护 release 环境提供 |
| **P13 高级控件扩展** | `.prompt/P13-component-expansion.md` | 已实现：二维码、代码编辑器、信号图、热力/比例条、拖动列表、Timer、Label/Operation，以及 BarChart 独立迷你柱样式与 Chart/Button/Tag/Radio/Checkbox 增强 |
| **P14 延后高级组件补全** | `.prompt/P14-deferred-advanced.md` | 已完成：Carousel、Calendar、InputTag、Mention、Watermark、TreeSelect、Tour、VirtualizedTable、VirtualizedTree 全部补齐 |
| **P15 质量收口** | `.prompt/P15-quality-hardening.md` | ✅ 已完成：CI/验证门禁、API 一致性、主题视觉、交互键盘、性能和 Docs 完整性 hardening |
| **P16 对外采用准备** | `.prompt/P16-adoption-readiness.md` | ✅ 已完成：README、CONTRIBUTING、CHANGELOG、minimal setup guidance、Rustdoc 入口、Docs Adoption Guide、采用性回归测试 |
| **P17 Dashboard Dogfooding** | `.prompt/P17-dogfood-dashboard.md` | ✅ 已完成并回流：独立 dashboard app 已移除，真实组合/主题/过滤/toast 等能力进入 Gallery/Docs |
| **P18 Dashboard Polish/API Ergonomics** | `.prompt/P18-dashboard-polish-and-api-ergonomics.md` | ✅ 已完成：Gallery shell polish、暗色主题切换、Dashboard Patterns 文档和回归测试；dashboard/sample 专用代码不进入核心组件库 |
| **P19 Dashboard State/Data Flow** | `.prompt/P19-dashboard-state-and-data-flow.md` | ✅ 已完成：Dashboard 数据模型、过滤、刷新、状态分支、Dashboard State 文档和回归测试 |
| **P20 Theme/Interaction Polish** | `.prompt/P20-theme-and-interaction-polish.md` | ✅ 已完成：System/Light/Dark 主题入口、dark subtle token、浮层/Loading mask token 化、Theme Gallery/Docs 页面和回归测试 |
| **P21 Release Candidate Readiness** | `.prompt/P21-release-candidate-readiness.md` | ✅ 已完成：0.2.x RC checklist、显式 package metadata、README/CHANGELOG/prompt/memory 同步和 release-boundary 回归测试 |
| **P22 gpui-component 采集阶段** | `.prompt/P22-gpui-component-harvest.md` | ✅ 已完成：`.prompt/P22-gpui-component-harvest.md` 全量闭环；候选项已通过独立组件、现有控件增强或明确不采集决策处理完毕 |

---


### prompt-md-0004-d174955185bb

<!-- ctx-migration source="prompt.md" unit="prompt-md-0004-d174955185bb" sha256="d174955185bbdf98cde4ac8fdd84d2fd99e8dda350dd1fbc4fc7fb2286a1ad7b" -->

## 3. 工程结构

```
liora/
├── Cargo.toml                       # [workspace] root
├── crates/
│   ├── liora-core/       lib.rs      # Global 配置、ElementExt trait、Z-Index
│   ├── liora-theme/      lib.rs      # Design Tokens、亮/暗主题、ButtonVariant/Size
│   ├── liora-components/ src/        # 全部业务组件 (button.rs, input.rs, ...)
│   ├── liora-tray/       src/        # 系统托盘 facade (tray-icon + muda)
│   │   └── lib.rs
│   ├── liora-packager/   src/        # P12 打包领域逻辑：metadata/format/checksum/manifest/backend config
│   └── liora-icons/      lib.rs      # Icon trait、图标函数
├── apps/
│   ├── liora-gallery/    src/        # 组件看板 (GPUI 窗口)
│       ├── main.rs
│       ├── category.rs
│       └── demos/
│           ├── mod.rs               # Demo 注册表 registry()
│           └── *_demo.rs            # 各组件 Demo 页面
│   └── liora-docs/                   # 官方原生文档主程序 (GPUI 窗口)
│       ├── content/
│       │   ├── pages/               # 每个文档/控件一份 Markdown
│       │   └── snippets/            # 外部 .rs 代码片段
│       └── src/
│           ├── main.rs
│           └── markdown.rs          # P8: Markdown AST → Liora 原生元素树
├── xtask/                            # P12 统一工程命令入口：cargo run -p xtask -- package ...
├── packaging/                        # P12 icons、desktop、metainfo、entitlements、Windows installer resources
├── .github/workflows/ci.yml           # P15 通用质量门禁
├── .github/workflows/package.yml      # P12 Linux/macOS/Windows packaging matrix
├── .memory/                          # 🧠 记忆库 (跨会话状态)
│   ├── state.md                     # 当前阶段 + 进度
│   ├── decisions.md                 # 架构决策记录
│   ├── inventory.md                 # 组件清单与完成状态
│   └── sessions.md                  # 会话历史
├── .prompt/                          # 📋 阶段提示词链
│   ├── P0-foundation.md
│   ├── P1-basic-elements.md
│   ├── P2-form-controls.md
│   ├── P3-popper-feedback.md
│   ├── P4-nav-data.md
│   ├── P5-advanced.md
│   ├── P6-builtin-id.md
│   ├── P7-demo-self-contained.md
│   ├── P8-engineering.md
│   ├── P9-deferred-advanced.md
│   ├── P10-charts.md
│   ├── P11-tray.md
│   ├── P12-packaging.md
│   ├── P13-component-expansion.md
│   ├── P14-deferred-advanced.md
│   ├── P15-quality-hardening.md
│   ├── P16-adoption-readiness.md
│   ├── P17-dogfood-dashboard.md
│   ├── P18-dashboard-polish-and-api-ergonomics.md
│   ├── P19-dashboard-state-and-data-flow.md
│   ├── P20-theme-and-interaction-polish.md
│   └── P21-release-candidate-readiness.md
├── prompt.md                         # 📌 本文件 (AI 入口)
├── design/architecture-design.md
└── structure.txt
```

---


### prompt-md-0005-877213684917

<!-- ctx-migration source="prompt.md" unit="prompt-md-0005-877213684917" sha256="877213684917edc42770a29140a6a3bcf8c0d8fe1f2132e4011bc242fb14dec1" -->

## 4. 开发工作流 🔄


### prompt-md-0006-8695c577c67f

<!-- ctx-migration source="prompt.md" unit="prompt-md-0006-8695c577c67f" sha256="8695c577c67f781d3c605ebde1e3c0eb7aa487cc2a984621fe0660aa0050d93c" -->

### 4.1 每次对话开始

```
1. 阅读本文件 (prompt.md)
2. 阅读 .memory/state.md  了解当前阶段和进度
3. 阅读 .prompt/<current-phase>.md  了解当前阶段任务
4. 开始工作
```



### prompt-md-0007-58d61506f269

<!-- ctx-migration source="prompt.md" unit="prompt-md-0007-58d61506f269" sha256="58d61506f2695a2622cb5528a2a2070a9c69b48d2ff57db10b0edc9649f3b934" -->

### 4.1.1 阶段状态提示

- P5 当前请求范围已结束；Carousel、Calendar、TreeSelect、InputTag、Mention、Watermark、Tour、VirtualizedTable、VirtualizedTree 已移入 `.prompt/P9-deferred-advanced.md`。
- P8 当前技术路线已调整为 **Liora Docs 主程序**：官方文档在 GPUI 原生窗口中渲染，且独立为 `liora-docs` 主程序；`liora-gallery` 保持组件看板，不再承担官方文档入口。
- P9 deferred backlog 已迁移并由 P14 完成；不要再把 Carousel/Calendar/TreeSelect/InputTag/Mention/Watermark/Tour/Virtualized* 当作等待项。
- P10 原生统计图阶段已完成：Line/Area/Bar/Pie/Ring/Sparkline、scale/axis/grid/legend、降采样，以及 Line/Area Overlay/Bar/Pie/Ring 原生 hover tooltip / hit testing 均已落地。后续缓存策略只有在真实 profiling 证据出现时再作为独立性能阶段推进。
- P11 系统托盘/进程常驻阶段已完成：`liora-tray` 采用 `tray-icon` + `muda`，支持动态图标、CheckBox 菜单、二级/三级/N 级菜单、关闭时隐藏到托盘和 Gallery/Docs 丰富用例。
- P12 原生打包阶段仓库内闭环已完成：`liora-packager` + `xtask package` + `packaging/` + CI workflow + runner-safe smoke/install-smoke + `release-readiness` gate + signing policy + `LicenseRef-Liora` policy 均已落地。真实签名凭据、notarization、系统级安装执行和正式 `v*` release 由受保护 release 环境提供；缺少凭据时 release gate 会阻止误发布。应用必须保持纯 Rust + GPUI native，严禁转成 Tauri 或引入 WebView/HTML/CSS/DOM/browser runtime。
- P13 主体已实现：二维码、代码编辑器、信号图、热力/比例条、横向/纵向拖动列表、Timer、Label、Operation，以及 Chart/Button/Tag/Radio/Checkbox 等高自定义能力；详见 `.prompt/P13-component-expansion.md`。
- P14 延后高级组件补全阶段已完成：Carousel、Calendar、InputTag、Mention、Watermark、TreeSelect、Tour、VirtualizedTable、VirtualizedTree 均已实现并有 Gallery/Docs/snippets/tests。详见 `.prompt/P14-deferred-advanced.md`。
- P15 质量收口阶段已完成：CI/验证门禁、API 一致性、主题视觉、交互键盘、性能和 Docs 完整性均已收口。详见 `.prompt/P15-quality-hardening.md`。
- P16 对外采用准备阶段已完成：根 README、CONTRIBUTING、CHANGELOG、crate-level Rustdoc、native Docs Adoption Guide 和相关回归测试已落地；独立 `examples/minimal-app` 已移除，采用说明融合进 Gallery/Docs。详见 `.prompt/P16-adoption-readiness.md`。
- P17 Dashboard Dogfooding 阶段已完成并已回流：原 `examples/dashboard-app` 独立样例已移除，其验证出的搜索/过滤、主题切换、toast、组合布局和状态说明融合进 Gallery/Docs。详见 `.prompt/P17-dogfood-dashboard.md`。
- P18 Dashboard Polish/API Ergonomics 阶段；P19 Dashboard State/Data Flow 阶段已完成并已回流：Dashboard app-shell 能力放在 Gallery/Docs；dashboard/sample 专用 helper、业务 sample/model 不进入核心组件库；Docs 保留 `Dashboard Patterns`/`Dashboard State` 作为 app-layer 模式说明。详见 `.prompt/P18-dashboard-polish-and-api-ergonomics.md`。
- P20 Theme/Interaction Polish 阶段已完成：System/Light/Dark 成为正式主题入口，dark subtle token 改为透明 overlay，Dialog/Drawer/Tour/Loading/CodeEditor/WindowFrame 等关键路径完成 token 化，并新增 Gallery Theme demo 与 Docs Theme 页面。详见 `.prompt/P20-theme-and-interaction-polish.md`。
- P21 Release Candidate Readiness 阶段已完成：新增 `docs/release-candidate-checklist.md` 作为 0.2.x RC checklist，显式补齐 package metadata，刷新 README/CHANGELOG/prompt/memory，并用回归测试锁住 Gallery/Docs canonical app、LicenseRef-Liora、纯 Rust + GPUI native 和 protected release 边界。详见 `.prompt/P21-release-candidate-readiness.md`。
- P22 gpui-component Harvest 阶段已完成：`.prompt/P22-gpui-component-harvest.md` 中的候选能力已全量闭环；适合独立暴露的已成为 Liora 控件，重叠能力已并入现有控件增强，WebView/WASM/browser runtime 等违反边界项明确不采集。独立 `Combobox` 不再推进，相关能力由 `Select`/`Autocomplete`/`SearchableList` 覆盖。详见 `.prompt/P22-gpui-component-harvest.md`。



### prompt-md-0008-7bc4c5ff57ed

<!-- ctx-migration source="prompt.md" unit="prompt-md-0008-7bc4c5ff57ed" sha256="7bc4c5ff57ed118a1ac27fb0c2af309415923248f38540fa69510c7a96c1b820" -->

### 4.1.4 README 同步红线（永久规范）

- 以后任何新功能开发、问题修复、依赖调整、平台行为调整、发布/打包流程调整，都必须显式检查是否需要同步 `README.md` 和 `README.zh-CN.md`。
- 如果改动影响 public crate 名称、features、依赖方式、GPUI revision / `[patch.crates-io]` 策略、初始化 API、主题/字体/图标/窗口/托盘行为、组件清单、组件主要 API、updater、packaging、release assets、CI 命令、MSRV 或常见问题排查，必须在同一个变更中更新中英文 README。
- 如果确认 README 不需要变化，最终总结中必须明确说明“README 不需要同步修改”，不能默认省略。
- README 中的示例应尽量可复制、可编译，并以仓库当前 API、Gallery/Docs 和官方 Zed GPUI revision 为准；禁止凭记忆写过期 API 或野路子示例。


### prompt-md-0009-cffd70271b73

<!-- ctx-migration source="prompt.md" unit="prompt-md-0009-cffd70271b73" sha256="cffd70271b7370d8b98c07310e4fb53f0b1ac8c750592f578cf81d1a469d5b02" -->

### 4.1.3 官方来源优先红线（永久规范）

- 涉及 GPUI / Zed、平台集成、窗口行为、打包安装、CI 发布、更新器、第三方 API 或系统能力时，必须先查看官方仓库源码、官方示例、官方文档或官方 release notes，再开始实现。
- 证据优先级：官方 `zed-industries/zed` 当前依赖 revision 的源码与 examples > 官方文档/release notes > 本地可复现实验。禁止仅凭记忆猜 API、猜平台行为或用“野路子”绕实现。
- Liora 只能依赖官方 GPUI / Zed 来源；禁止漂移到 `open-gpui` 等非官方 fork。临时本地 patch 只能用于 Gallery/Docs app-only 验证，并且不得耦合进发布到 crates.io 的 SDK 包。
- 每次升级或适配 GPUI/平台 API 时，必须确认 `Cargo.toml`/`Cargo.lock` 的真实来源与 revision，并用编译、测试或最小运行验证证明实现有效。
- 项目保持纯 Rust + GPUI native；禁止引入 Tauri/WebView/HTML/CSS/DOM/browser runtime。


### prompt-md-0010-959241696f81

<!-- ctx-migration source="prompt.md" unit="prompt-md-0010-959241696f81" sha256="959241696f814405ccfd11ee9242fc9e95f4a00a08307d4d1bbc2474c1de3d22" -->

### 4.1.2 应用与示例边界红线

- 不再新增独立 `examples/*-app` 作为 sample 应用；采用、dogfooding、真实组合验证统一进入 `apps/liora-gallery` 和 `apps/liora-docs`。
- `liora-components` 只放可复用组件、组合 helper、基础能力；禁止把业务 sample、DashboardSample、mock model 等示例屏幕塞进核心组件库。
- Gallery/Docs 中如果反复出现原生 GPUI glue（窗口壳、布局、交互、状态样板等），应优先抽象为 Liora 控件/ helper，而不是在应用层继续堆 `gpui::div()/px()/WindowOptions`。
- Gallery 是视觉 dogfooding 表面；Docs 是采用/说明表面。两者必须保持纯 Rust + GPUI native，但不应变成 raw GPUI 用法示例仓库。


### prompt-md-0011-6922d0141615

<!-- ctx-migration source="prompt.md" unit="prompt-md-0011-6922d0141615" sha256="6922d01416155f2cd3a1f1881716e5dde6315720f0fb6afc29019251b8ee4b1a" -->

### 4.2 每个组件/功能开发流程

```
┌─────────────────────────────────────────────────┐
│ 1. 编码                                          │
│    └── 创建/修改 crates/liora-components/src/<name>.rs
│    └── 在 lib.rs 中 pub mod + pub use            │
│                                                  │
│ 2. Demo (必须)                                    │
│    └── 创建 apps/liora-gallery/src/demos/<name>_demo.rs
│    └── render() -> AnyElement   │
│    └── 在 demos/mod.rs 注册表添加 DemoEntry      │
│                                                  │
│ 3. 验证 (必须)                                    │
│    └── cargo check  (0 errors, 0 warnings)       │
│                                                  │
│ 4. 提交 (通过后)                                   │
│    └── git add -A                                │
│    └── git commit -m "✨ component: add <Name>"  │
│    └── git push origin master                    │
│                                                  │
│ 5. 记忆更新 (必须)                                 │
│    └── 更新 .memory/inventory.md 组件状态         │
│    └── 更新 .memory/sessions.md 会话记录          │
│    └── 里程碑完成时更新 .memory/state.md           │
└─────────────────────────────────────────────────┘
```


### prompt-md-0012-1dbfdc647075

<!-- ctx-migration source="prompt.md" unit="prompt-md-0012-1dbfdc647075" sha256="1dbfdc647075c4984ebbdf875cdc0ec1597421369eb956b0ac11c32fed8b4b95" -->

### 4.3 阶段完成时

```
1. 更新 .memory/state.md   (标记阶段 done, 更新 next)
2. 审查 .prompt/<next-phase>.md   (确保上下文准确)
3. 如有架构决策，更新 .memory/decisions.md
4. Git commit + push
```


### prompt-md-0013-078a92085722

<!-- ctx-migration source="prompt.md" unit="prompt-md-0013-078a92085722" sha256="078a92085722e087e65a807f1c317c5e563d69d0a82ee8e34fe383357a2cb323" -->

### 4.4 阶段回退/调整时

```
1. 更新 .memory/state.md   (回退 phase status)
2. 更新 .prompt/<affected-phase>.md  (调整任务描述)
3. 更新所有后续 .prompt/ 文件 (级联影响)
4. Git commit + push
```

---


### prompt-md-0014-b241b49e5aae

<!-- ctx-migration source="prompt.md" unit="prompt-md-0014-b241b49e5aae" sha256="b241b49e5aaeb327088ce8e65fbbf842bc9827da39d0582bc534a1b4c2cd62c2" -->

## 5. 关键架构约束


### prompt-md-0015-2e4bdcdb6d19

<!-- ctx-migration source="prompt.md" unit="prompt-md-0015-2e4bdcdb6d19" sha256="2e4bdcdb6d193c072e91daece5be9666338fd4d0a0d925bb5091fdea0888e3e7" -->

### 5.1 组件 API 风格（codex 范式）

```rust
// ✅ 正确 — RenderOnce + IntoElement，主题从 cx.global 自动读取
Button::new("Save").primary().large()
Icon::new(IconName::House).size(24.0)
CodeBlock::new("cargo run -p liora-docs").shell().copyable(true)

// 实现范式
impl RenderOnce for MyComponent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        // ...
    }
}
impl IntoElement for MyComponent {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}

// ❌ 禁止 — .build(theme) 传参模式
```


### prompt-md-0016-58dd2406dd04

<!-- ctx-migration source="prompt.md" unit="prompt-md-0016-58dd2406dd04" sha256="58dd2406dd04e2e3075a42f1c9ad9e4c5f2475f8197694a9cea555ca231fcec6" -->

### 5.2 类型和 Context

```rust
// 官方 Zed GPUI 当前 pinned revision 的关键类型
gpui::App              // 应用全局
gpui::Context<'_, V>   // 视图上下文 (可读 Global)
gpui::Window           // 窗口句柄
gpui::AnyElement       // 类型擦除的 Element (Demo 注册表用)
gpui::IntoElement      // 渲染目标 trait
gpui::InteractiveElement // hover/on_mouse_up 等交互 trait

// 读取主题
fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    let theme = _cx.liora(); // ContextExt trait
    // ...
}
```


### prompt-md-0017-a0e2a086ebbd

<!-- ctx-migration source="prompt.md" unit="prompt-md-0017-a0e2a086ebbd" sha256="a0e2a086ebbdba7da96aafc72b644b8832cd0a499899dc9f8dee0c3c8e8485f7" -->

### 5.3 GPUI Features

```toml

### prompt-md-0018-770eec4eaec1

<!-- ctx-migration source="prompt.md" unit="prompt-md-0018-770eec4eaec1" sha256="770eec4eaec1a3910e5120a4aadade8b7fd25472092465f50abb2f8f2558779e" -->

# 库 crate — 不启用平台 feature
liora-core/Cargo.toml:     gpui = { workspace = true }
liora-theme/Cargo.toml:    gpui = { workspace = true }


### prompt-md-0019-e3f42ce8f431

<!-- ctx-migration source="prompt.md" unit="prompt-md-0019-e3f42ce8f431" sha256="e3f42ce8f43189b057d5c11d1eb45f2f39ff821ac764bd42fb2b3a4d8a441ac7" -->

# App — 显式启用平台 feature
liora-gallery/Cargo.toml:
  gpui = { workspace = true, features = ["wayland", "x11", "font-kit"] }
  gpui_platform = { workspace = true, features = ["wayland", "x11", "font-kit"] }
  ```


### prompt-md-0020-e30001904db4

<!-- ctx-migration source="prompt.md" unit="prompt-md-0020-e30001904db4" sha256="e30001904db442712db03cc3ded4d84c76c3a1e0830c531375c6273117d4362b" -->

### 5.4 不在当前官方 Zed GPUI pinned revision 中的 API

```
❌ RenderOnce    → 使用 Render
❌ ViewContext   → 使用 Context<'_, V>
❌ WindowContext → 使用 Window + App
❌ .when()       → 使用 if/else 手动构建
❌ .when_some()  → 使用 if let 手动构建
❌ .active()     → 需要先 .id() → Stateful<Div>, 普通 Div 无此方法
❌ .on_click()   → 需要 StatefulInteractiveElement (先 .id())
✅ .on_mouse_up(MouseButton::Left, ...) → InteractiveElement 上可用
✅ .hover(|style| ...) → InteractiveElement 上可用
✅ .cursor_pointer()   → 在 hover 闭包内使用
```

---


### prompt-md-0021-c83d45d6b1b1

<!-- ctx-migration source="prompt.md" unit="prompt-md-0021-c83d45d6b1b1" sha256="c83d45d6b1b1857914c67cadd20dce4a8d32420646d85a4534cf75982c0919ff" -->

## 6. P8 Liora Docs 主程序规约

P8 的目标不是搭建网页文档站，而是把 `liora-gallery` 维持组件看板，文档另以 `liora-docs` 独立主程序交付。


### prompt-md-0022-7f75ec2b8146

<!-- ctx-migration source="prompt.md" unit="prompt-md-0022-7f75ec2b8146" sha256="7f75ec2b8146c831e9bcbc0666a13165b903372fc16f7ed0fda12bec34153307" -->

### 6.1 绝对边界

- 100% GPUI 原生窗口运行。
- 文档渲染基于 Rust、GPUI 元素树、Liora 组件、Flex 布局和原生滚动容器。
- 禁止引入 Web 文档站、浏览器渲染路径、跨端转译运行时或网页排版模型。
- Markdown 只允许作为输入文本格式；解析后必须映射为 Liora/GPUI 原生节点。


### prompt-md-0023-b7ac8e5c2e2e

<!-- ctx-migration source="prompt.md" unit="prompt-md-0023-b7ac8e5c2e2e" sha256="b7ac8e5c2e2eff46462264a528ee969d73e2f0e0fbf558511ae475c396079346" -->

### 6.2 Markdown 自举架构

- `pulldown-cmark` 只负责 Markdown AST/Event 解析。
- 富文本折行、样式、段落布局由 Liora Typography 组件负责。
- P8 需要优先补齐富文本文本片段与段落能力：多样式片段在同一段落中流式拼接、自动换行、不截断。
- `apps/liora-docs/src/markdown.rs` 负责 `render_markdown(md_text: &str) -> gpui::AnyElement`。
- Renderer 使用 `Vec` 栈管理块级容器，使用文本样式上下文管理 strong/emphasis/code 等内联状态。
- 文档内容按单页文件维护：`apps/liora-docs/content/pages/<page>.md`；组件页使用 `<component>.md`。
- 代码示例与 Markdown 分离：`apps/liora-docs/content/snippets/<page>/<case>.rs`。
- Markdown fenced code 通过 `src` 引用外部片段，例如 <code>```rust src="button/types.rs"</code>，`src` 路径相对于 `content/snippets/`。


### prompt-md-0024-f9ad363401d7

<!-- ctx-migration source="prompt.md" unit="prompt-md-0024-f9ad363401d7" sha256="f9ad363401d750cf092a8480ecc9e7963e76f50d923e590c9b84af9ce83159af" -->

### 6.3 Live Demo 注入

Markdown 中的特殊语法：

```text
::Demo{component="Button"}::
```

必须被解析为真实 Liora/GPUI view node，而不是普通文本。插入后的组件必须保留 hover、click 等真实交互能力。

---



### prompt-md-0025-e5b5ea005e43

<!-- ctx-migration source="prompt.md" unit="prompt-md-0025-e5b5ea005e43" sha256="e5b5ea005e43c392d3305d81ded7d7f5f632e62f80ad776b313155609e18e70c" -->

## 6.5 P10 统计图组件阶段规约

P10 目标是在 `liora-components` 中新增企业级统计图控件，全部运行在 GPUI 原生渲染路径中。严禁引入 ECharts、Canvas/WebView、SVG DOM、HTML/CSS、WASM 或跨端图表运行时。

技术路线：

- 首选 GPUI 官方能力：`canvas(...)`、`PathBuilder`、`Window::paint_path`、`Window::paint_quad`、TextSystem/`Text`/`Paragraph`。
- 图表绘制基础设施沉淀在组件库内，例如 `chart.rs` / `chart_scale.rs` / `chart_axis.rs` / `chart_shape.rs`。
- `https://github.com/vicanso/zedis` 的 Metrics 页面可作为 GPUI 图表案例参考：它通过 `canvas` 绘制 Area/Line/Bar，并将 scale、axis、grid、shape 拆层；但 Liora 必须实现自己的 API、主题、测试与文档。
- 主题颜色优先来自 `Theme` 的语义色，必要时新增 chart palette token。

首批交付控件：

1. `LineChart` — 折线图，支持多 series、平滑/直线、点标记、空数据。
2. `AreaChart` — 面积图，支持填充透明度、堆叠后续扩展。
3. `BarChart` — 柱状图，支持竖向柱、分组后续扩展。
4. `PieChart` / `RingChart` — 饼图/环图，支持百分比、legend。
5. `Sparkline` — 迷你趋势图，用于 Statistic/Card 中嵌入。
6. 基础设施：linear/band/point scales、axis、grid、legend、tooltip/hover hit test。

每个图表必须：新增组件文件、导出 API、Gallery demo、Docs 页面与 snippet、单元测试、`cargo check/test/run` 验证后提交推送。



### prompt-md-0026-0d574965d7fb

<!-- ctx-migration source="prompt.md" unit="prompt-md-0026-0d574965d7fb" sha256="0d574965d7fbf0c2d08bedfbebdf8908164388500388f2540f39ce9d8520234d" -->

## 6.6 P12 原生安装器打包规约

P12 目标是为 `liora-gallery`、`liora-docs` 以及未来 Liora GPUI 主程序建立跨平台原生安装器/发布产物流水线。


### prompt-md-0027-2b61e9a7049c

<!-- ctx-migration source="prompt.md" unit="prompt-md-0027-2b61e9a7049c" sha256="2b61e9a7049cd62fa8e126c0cc60d519186e2ec8b0a29c086bb1475469bff942" -->

### 6.6.1 绝对边界

- Liora app 必须保持 **纯 Rust + GPUI native**。
- 严禁把 `liora-gallery`、`liora-docs` 或未来 Liora 主程序改造成 Tauri 应用。
- 严禁引入 WebView、HTML/CSS/DOM、browser runtime 或前端构建链作为应用运行时。
- 可以使用独立 packaging tools，但它们只能处理产物打包，不能改变应用架构。


### prompt-md-0028-fb8833067e32

<!-- ctx-migration source="prompt.md" unit="prompt-md-0028-fb8833067e32" sha256="fb8833067e3267fb2e6b907f50a6046ccb0107ad04e4d37edf650e8a70027050" -->

### 6.6.2 当前架构

| 模块 | 职责 |
|---|---|
| `crates/liora-packager` | 打包领域逻辑：app metadata、format enum、checksum、manifest、cargo-packager config、RPM metadata |
| `xtask` | 统一入口：`cargo run -p xtask -- package ...` / `cargo run -p xtask -- package ci ...` |
| `packaging/` | 静态平台资源：icons、Linux desktop/metainfo、macOS entitlements、Windows nsis/wix folders |
| `.github/workflows/package.yml` | Linux/macOS/Windows packaging matrix，dry-run workflow_dispatch，`v*` tag 真实打包并自动发布 GitHub Release |


### prompt-md-0029-b5656529731e

<!-- ctx-migration source="prompt.md" unit="prompt-md-0029-b5656529731e" sha256="b5656529731e4f629dd8ac6a99546cf1361a98dbad542b3c0398d4f79e5fbc7a" -->

### 6.6.3 已完成能力

- `cargo run -p xtask -- package validate`
- `cargo run -p xtask -- package build --app <gallery|docs>`
- `cargo run -p xtask -- package --app <gallery|docs> --format <format>`
- `cargo run -p xtask -- package ci --all-apps --format platform-defaults`
- `cargo run -p xtask -- package smoke --all-apps --format platform-defaults`
- `--dry-run --skip-build` 生成后端配置并打印真实命令。
- `cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run` 生成 runner-safe install/uninstall 计划，不依赖真实 artifact。
- `cargo-packager` config generation：AppImage、deb、app、dmg、NSIS、MSI/WiX。
- `cargo-generate-rpm` supplemental backend config generation：RPM。
- Liora supplemental portable `.tar.gz` backend：收集 release binary、icons、Linux desktop/metainfo、README、启动脚本，并输出 `<package>-<version>-<platform>-<target-triple>.tar.gz`。
- Linux deb/rpm runtime dependency metadata：GTK3、Ayatana/AppIndicator、X11/Wayland、xkbcommon、fontconfig/freetype、Vulkan、ALSA、xdg-utils。
- 真实打包后生成：`package-manifest.json`、`checksums.txt`、`release-notes.md`，manifest 包含 version、platform、target triple、git sha。
- GitHub Actions 在 `main` push 上构建 preview 包（`<base>-preview.<run_number>.<short_sha>`），在 `v*` tag 上自动下载各平台 release artifacts，基于 git commits 按类型分组收集 changelog，创建/更新 GitHub Release 并上传全部构建产物。
- main Liora logo 已选择第 3 套 ribbon，落到 `packaging/icons/liora.*`。


### prompt-md-0030-5a329bf6cab7

<!-- ctx-migration source="prompt.md" unit="prompt-md-0030-5a329bf6cab7" sha256="5a329bf6cab714cf983de8855d52fe6911e5fc21b3ffef98dc55e6461bca0822" -->

### 6.6.4 当前验证基线

```bash
cargo check -p xtask -p liora-packager
cargo test -p liora-packager
cargo run -p xtask -- package validate
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run
```

Dry-run 预期生成：

```text
target/liora-packager/Packager.gallery.toml
target/liora-packager/Packager.docs.toml
target/liora-packager/GenerateRpm.gallery.toml
target/liora-packager/GenerateRpm.docs.toml
```


### prompt-md-0031-abaddbed967b

<!-- ctx-migration source="prompt.md" unit="prompt-md-0031-abaddbed967b" sha256="abaddbed967b6b1e39dd6aa923e0d2f0373b6e13dd25da6300af991e4cbc81d3" -->

### 6.6.5 P12 剩余工作

GitHub preview runner `27613242837` / commit `5a3615d` 已通过 Linux/macOS/Windows packaging matrix：真实 package generation、artifact smoke、raw binary/package artifact upload 均成功。下一位开发者接手时按此优先级推进：

1. **Signing / notarization**：macOS `codesign`/`notarytool`/`stapler`，Windows `signtool`/timestamp，CI secrets 与 unsigned fallback。
2. **GitHub Release automation 后续增强**：基础自动 Release、push preview 构建、按提交类型分组 changelog 已接入；后续可补 release draft/prerelease 策略、非 tar 后端 artifact 命名清洗、按平台聚合 manifest。
3. **Install / uninstall smoke**：已补 artifact smoke 与 `xtask package install-smoke --dry-run` plan-only gate；portable `.tar.gz` 支持显式 `--execute-install` 安全解压/删除验证。仍待在 dedicated runners 和明确 policy 下放开真正系统级 deb/rpm/AppImage/macOS/Windows 安装/卸载执行 gate。
4. **License / metadata cleanup**：`LICENSE.md` 已明确当前 `LicenseRef-Liora` 策略；Cargo package metadata、RPM/backend metadata 和 RC checklist 均保持一致。只有 owner 正式改为 OSS/commercial license 时才调整。
5. **CI release-tag iteration**：preview runner 已通过；release tag 已校验为 `vX.Y.Z` 且匹配 packager version；仍需在真实 `v*` tag 上验证 release job、GitHub Release asset 上传与 Windows MSI。

完整细节见 `.prompt/P12-packaging.md`。



### prompt-md-0032-337bc70446a9

<!-- ctx-migration source="prompt.md" unit="prompt-md-0032-337bc70446a9" sha256="337bc70446a94521102b156f7c0cbf118606d3b4361278faac128ba17e7c5724" -->

## 6.7 P13 高级控件扩展规约

P13 主体已实现，目标是补齐 Dashboard / 低代码配置 / 数据监控 / 内容编辑场景中的高级控件与既有控件高自定义能力。

已落地范围：

- 新增：`QrCode`、`CodeEditor`、`SignalMeter`、`HeatBar`、`SegmentRatioBar`、`HorizontalList`、`Timer`、`Label`、`Operation`。
- 增强：`RingChart` 外置 legend/value pattern、`LineChart` per-series 线型、`BarChart` 独立迷你柱样式与 value range colors、`RingProgress` gradient/completion color、`Button` gradient/custom color states、`Tag` flow layout、`Radio`/`Checkbox` option render customization、既有垂直列表 item drag。
- QR/CodeEditor 如需新增依赖，必须先做 dependency review；CodeEditor 第一阶段复用已有 `syntect` 高亮，语法检查只做 provider/diagnostics 扩展点，不硬绑定 LSP。
- 所有新增/增强控件必须同步 Gallery demo、Docs 页面、外部 snippets 和核心状态/计算测试；已有控件增强必须落在原组件、原 demo、原 docs/snippets 上，不另建平行控件。

完整实现与维护规约见 `.prompt/P13-component-expansion.md`。

执行原则：已有控件直接增强原实现（例如 Tag flow、Progress/RingProgress 渐变、LineChart 线型、BarChart 独立迷你柱样式/区间色、Button 自定义色、Radio/Checkbox option 自定义），不要新增替代控件或平行页面。


### prompt-md-0033-91e9faf09e54

<!-- ctx-migration source="prompt.md" unit="prompt-md-0033-91e9faf09e54" sha256="91e9faf09e54e57fc2fc44a1679ae307f24a681285db7cdae008495450f9b10c" -->

## 7. Gallery Demo 规约


### prompt-md-0034-f9077aa720ac

<!-- ctx-migration source="prompt.md" unit="prompt-md-0034-f9077aa720ac" sha256="f9077aa720ac577746b4a7e3c22d562a084aa62a662efc90c865e05763c8fe4e" -->

### 7.1 Demo 函数签名

```rust
// apps/liora-gallery/src/demos/<name>_demo.rs
pub fn render() -> gpui::AnyElement {
    gpui::Component::new(NameDemo).into_any_element()
}

struct NameDemo;

impl gpui::RenderOnce for NameDemo {
    fn render(self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        let theme = &cx.global::<liora_core::Config>().theme;
        div().flex().flex_col().gap_4()
            .child(section_header(theme, "Variants 变体"))
            .child(demo_row(vec![...]))
    }
}
```


### prompt-md-0035-c93028517f6d

<!-- ctx-migration source="prompt.md" unit="prompt-md-0035-c93028517f6d" sha256="c93028517f6de8c9a67e43624322703cd1280fc63a3c52b81cf0cb0ea32b7944" -->

### 7.2 注册表 (增量添加)

```rust
// apps/liora-gallery/src/demos/mod.rs
pub fn registry() -> Vec<DemoEntry> {
    vec![
        DemoEntry {
            name: "Button 按钮",
            category: Category::Basic,
            description: "常用的操作按钮",
            render: button_demo::render,  // ← 函数指针
        },
        // 👇 新增组件只需在这里加一项
        DemoEntry {
            name: "Link 链接",
            category: Category::Basic,
            description: "文字链接",
            render: link_demo::render,
        },
    ]
}
```


### prompt-md-0036-af07b520c857

<!-- ctx-migration source="prompt.md" unit="prompt-md-0036-af07b520c857" sha256="af07b520c85789db8b15e3f96ea122326d47529af995112f1b03433151ad0d37" -->

### 7.3 Category 分类

```rust
Category::Basic       // ⊞ Basic 基础
Category::Form        // ☰ Form 表单
Category::Data        // ⊟ Data 数据
Category::Navigation  // ☈ Navigation 导航
Category::Feedback    // ⚡ Feedback 反馈
Category::Others      // ⋯ Others 其他
```

---


### prompt-md-0037-f534ce4dd187

<!-- ctx-migration source="prompt.md" unit="prompt-md-0037-f534ce4dd187" sha256="f534ce4dd187cd8214c68c1b7e19d7260bb6ebf248d5e89e0fbb2f91b0826347" -->

## 8. 记忆系统 🧠


### prompt-md-0038-481a6ef4e2dc

<!-- ctx-migration source="prompt.md" unit="prompt-md-0038-481a6ef4e2dc" sha256="481a6ef4e2dce2e197aad935b873969eb401d9ca8f4c1344f5e7d5e50f8c4eea" -->

### 8.1 记忆库更新时机

| 事件 | 更新文件 |
|------|---------|
| 任何代码变更 | `.memory/sessions.md` (追加记录) |
| 组件完成 | `.memory/inventory.md` (标记 ✅) |
| 阶段完成 | `.memory/state.md` (更新 phase/next) |
| 架构决策 | `.memory/decisions.md` (追加 ADR) |
| 发现 API 差异 | `.memory/sessions.md` (Key Discoveries) |


### prompt-md-0039-1d2115c8d841

<!-- ctx-migration source="prompt.md" unit="prompt-md-0039-1d2115c8d841" sha256="1d2115c8d841bc634904b6351a9dda036dfc2eaee11e57ffb9240d7dea13701e" -->

### 8.2 记忆库格式

所有 .memory/ 文件使用 Markdown，保持简洁、结构化、可追加。新条目追加在文件末尾或对应位置。

---


### prompt-md-0040-b1e556dd053b

<!-- ctx-migration source="prompt.md" unit="prompt-md-0040-b1e556dd053b" sha256="b1e556dd053b934b3c553a21c25875f6b6e71fbf11b01dc69dfae022e8187afa" -->

## 9. Git 提交规范


### prompt-md-0041-7e721bfb1a97

<!-- ctx-migration source="prompt.md" unit="prompt-md-0041-7e721bfb1a97" sha256="7e721bfb1a97c65545a31b3d672e8136b9dd01f07ab3bd4c495a52b43821cdd7" -->

### 9.1 Commit Message 格式

```
<emoji> <scope>: <subject>

<body — 可选，多行详细说明>

<footer — 可选，关联 issue>
```


### prompt-md-0042-4b528cf52e35

<!-- ctx-migration source="prompt.md" unit="prompt-md-0042-4b528cf52e35" sha256="4b528cf52e35b4d9a0f8184e8267ea9bf4498a73359a45680885113ef42365c3" -->

### 9.2 Emoji 参考

| Emoji | 用途 |
|-------|------|
| ✨ `:sparkles:` | 新组件/新功能 |
| 🎨 `:art:` | 样式/主题/Token 调整 |
| 🏗️ `:building_construction:` | 架构/结构变更 |
| 🐛 `:bug:` | Bug 修复 |
| ♻️ `:recycle:` | 重构 |
| 📝 `:memo:` | 文档 |
| 🧪 `:test_tube:` | 测试 |
| 🔧 `:wrench:` | 配置/工具 |
| 🚀 `:rocket:` | 发布/CI |
| 🧠 `:brain:` | 记忆库更新 |
| 📋 `:clipboard:` | 阶段提示词更新 |


### prompt-md-0043-36378919d9b6

<!-- ctx-migration source="prompt.md" unit="prompt-md-0043-36378919d9b6" sha256="36378919d9b66e82ac6ff7f452de0082667ee15727bbb3ab2b1811da608c4e28" -->

### 9.3 示例

```
✨ button: add icon_start/icon_end support

- Add .icon_start(AnyElement) and .icon_end(AnyElement) builder methods
- Update demo with icon examples
- Register in gallery registry

Closes #P1-button-icons
```

---


### prompt-md-0044-7676332419b9

<!-- ctx-migration source="prompt.md" unit="prompt-md-0044-7676332419b9" sha256="7676332419b9436359c7b64bc5dfb0b4e3cef0cb08967005f9d78bdca23e9fa2" -->

## 10. 阶段导航

```
当前阶段 → 读取 .memory/state.md 获取
├── P0 Foundation          ✅ → .prompt/P0-foundation.md
├── P1 Basic Elements      ⬜ → .prompt/P1-basic-elements.md
├── P2 Form Controls       ⬜ → .prompt/P2-form-controls.md
├── P3 Popper + Feedback   ⬜ → .prompt/P3-popper-feedback.md
├── P4 Nav + Data          ⬜ → .prompt/P4-nav-data.md
├── P5 Advanced            ⬜ → .prompt/P5-advanced.md
├── P6 Built-in Unique ID  ⬜ → .prompt/P6-builtin-id.md
├── P7 Demo Self-Contained ⬜ → .prompt/P7-demo-self-contained.md
├── P8 Native Docs App ✅ → .prompt/P8-engineering.md
├── P9 Deferred Advanced ⏸️ → .prompt/P9-deferred-advanced.md
├── P10 Charts ✅ → .prompt/P10-charts.md
├── P11 Tray ✅ → .prompt/P11-tray.md
├── P12 Packaging ✅ → .prompt/P12-packaging.md
├── P13 Component Expansion ✅ → .prompt/P13-component-expansion.md
└── P14 Deferred Advanced 🧭 → .prompt/P14-deferred-advanced.md
```

---


### prompt-md-0045-58f1dc74d999

<!-- ctx-migration source="prompt.md" unit="prompt-md-0045-58f1dc74d999" sha256="58f1dc74d999ed223fb93718c8b95e94fe3735434c67598f12f3fa9a604b860d" -->

## 11. 快速命令

```bash

### prompt-md-0046-28679fbbcee5

<!-- ctx-migration source="prompt.md" unit="prompt-md-0046-28679fbbcee5" sha256="28679fbbcee5405b47e037427ff90bdfb1ca4bb57a7bedd11703a6ea178e711f" -->

# 编译检查
cargo check


### prompt-md-0047-f99de98c89fe

<!-- ctx-migration source="prompt.md" unit="prompt-md-0047-f99de98c89fe" sha256="f99de98c89fe1c5e68c09d9ff5273e5ae1d1e245b84a199d7de38fdb9f73a53c" -->

# 运行组件看板
cargo run -p liora-gallery


### prompt-md-0048-b05178cd704f

<!-- ctx-migration source="prompt.md" unit="prompt-md-0048-b05178cd704f" sha256="b05178cd704fc7a2d1f3edb3bf61b0b07bf2ab20e8a3ae86051fc2d906e00426" -->

# 编译所有 crate
cargo build


### prompt-md-0049-a6833c1e9c12

<!-- ctx-migration source="prompt.md" unit="prompt-md-0049-a6833c1e9c12" sha256="a6833c1e9c124cf1d50139863a4ce9ba9715498c5189291af93cc672685a38ae" -->

# 运行测试 (如果有)
cargo test


### prompt-md-0050-93ed2d4e9926

<!-- ctx-migration source="prompt.md" unit="prompt-md-0050-93ed2d4e9926" sha256="93ed2d4e99260821ac9b1f968ac96ff3749f62f0b9ea2635a0643c6323654903" -->

# P12 打包资源校验
cargo run -p xtask -- package validate


### prompt-md-0051-cb2da70375a2

<!-- ctx-migration source="prompt.md" unit="prompt-md-0051-cb2da70375a2" sha256="cb2da70375a2b6a74aa434af80d0ab8323f3907a8c779a5d6831fa5c8ca1d33c" -->

# P12 打包 dry-run（生成后端配置，不真实产物）
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build


### prompt-md-0052-37c07d272d6c

<!-- ctx-migration source="prompt.md" unit="prompt-md-0052-37c07d272d6c" sha256="37c07d272d6c868c2aff6379fc90178ea1dc3638860e1df1138b7b23b14f93cc" -->

# P12 install/uninstall plan-only gate（不需要真实产物）
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run


### prompt-md-0053-05fb6e3cec6b

<!-- ctx-migration source="prompt.md" unit="prompt-md-0053-05fb6e3cec6b" sha256="05fb6e3cec6b2e044f7035999d9fef8c4999ee7fc841a1c0ac27e62cc06b1933" -->

# P12 当前平台真实打包（需要先安装后端工具）
cargo run -p xtask -- package ci --all-apps --format platform-defaults


### prompt-md-0054-a537c4615b2f

<!-- ctx-migration source="prompt.md" unit="prompt-md-0054-a537c4615b2f" sha256="a537c4615b2f63fe47745005cb16b1022de1bcbd1a7fb86e53597032e0688509" -->

# 清理构建
cargo clean
```

---


### prompt-md-0055-63a8084ab275

<!-- ctx-migration source="prompt.md" unit="prompt-md-0055-63a8084ab275" sha256="63a8084ab2754625b738bba1ea923a8d8f43e8b7327bdae83d3e0462fd6138a5" -->

## 12. 启动检查清单 ⚡

接手本项目时的最小行动集：

- [ ] 读取 `prompt.md` (本文件)
- [ ] 读取 `.memory/state.md` (当前阶段)
- [ ] 读取 `.prompt/<current-phase>.md` (当前任务)
- [ ] 运行 `cargo check` 确认编译基线
- [ ] 运行 `cargo run -p liora-gallery` 确认看板基线
- [ ] 开始工作, 按 §4.2 流程推进

<!-- ctx-managed-legacy-migration:end -->
