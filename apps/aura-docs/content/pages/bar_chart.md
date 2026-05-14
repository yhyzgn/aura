# BarChart

`BarChart` 使用 GPUI 原生 `canvas` 与 `paint_quad` 绘制分类柱状图，适合展示分类对比、分组对比和构成堆叠。它复用 P10 图表基础设施中的 `ChartSeries`、`ChartPoint`、比例尺、坐标轴和主题色板。

## 基础分组

### 效果

::AuraDemo{component="BarChartBasic"}::

### 代码

```rust src="bar_chart/basic.rs"
```

## 多序列分组

### 效果

::AuraDemo{component="BarChartGrouped"}::

### 代码

```rust src="bar_chart/grouped.rs"
```

## 堆叠柱状图

### 效果

::AuraDemo{component="BarChartStacked"}::

### 代码

```rust src="bar_chart/stacked.rs"
```

## 颜色、间距与标签内容

### 效果

::AuraDemo{component="BarChartCustom"}::

### 代码

```rust src="bar_chart/custom.rs"
```
