# Timer

`Timer` 用于展示正向计时、倒计时和单位换算结果。当前控件是受控展示组件：调用方传入已耗时和总时长，组件负责格式化展示，并提供 `snapshot`、`elapsed_as`、`remaining_as` 等结果读取 API。

## 正向计时

### 效果

::AuraDemo{component="TimerCountUp"}::

### 代码

```rust src="timer/count_up.rs"
```

## 倒计时

### 效果

::AuraDemo{component="TimerCountDown"}::

### 代码

```rust src="timer/count_down.rs"
```

## 单位与紧凑模式

### 效果

::AuraDemo{component="TimerUnits"}::

### 代码

```rust src="timer/units.rs"
```

## 读取计时结果

### 代码

```rust src="timer/result.rs"
```

## 时钟格式

### 效果

::AuraDemo{component="TimerClock"}::

### 代码

```rust src="timer/clock.rs"
```
