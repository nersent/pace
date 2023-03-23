# **PACE** documentation

## 0. Abstract

The core feature of Pace is an incremental architecture, which means that all components (indicators/strategies) are computed in O(N) linear time and updated in O(1) contant time. This requires for all components to work in a certain way.

## 1. Load data

Firstly, you need to create [`data provider`](/pace/src/core/data_provider.rs). You can load a CSV or parquet file using Polars library.

```rust
let data_path = Path::new("example/fixtures/btc_1d.csv");
let df = read_df(&data_path);
let data_provider = InMemoryDataProvider::from_df(&df).to_arc();
```

`DataProvider.to_arc()` wraps the data provider into `Arc,` so data can be accessed from thread

## 2. Context

Then, you need to create a [`Context`](/pace/src/core/context.rs).

```rust
let ctx = Context::new(data_provider);
```

Context has a special method, `Context.clone()`, which creates a new instance of context, but it's internal data is shared. This is useful for passing context to new owner, without having to pass a reference to original context every time.

```rust
let ctx = Context::new(data_provider);
let ctx_clone = ctx.clone();

let is_equal = ctx.bar.close() == ctx_clone.bar.close(); // true
```

## 3. Incremental components

All [`incremental`](/pace/src/core/incremental.rs) components share [`Context`](/pace/src/core/context.rs), which allows easy access to OHLCV and provides utility functions.

[`Incremental`](/pace/src/core/incremental.rs) is a trait that all incremental components must implement.

`Incremental.next()` - computes the next value of the component. It can optionally take a value and optionally output new value.

`Incremental` trait requires two generic types - **`INPUT`**, **`OUTPUT`**. If you don't need to pass any value to the component, you can use `()` as input type, or as output type.

Here is an example of `float64` as input and output.

```rust
struct MyComponent {
    previous_value: f64,
}

impl MyComponent {
    fn new(ctx: Context) -> Self {
        Self {
            ctx,
            previous_value: 0.0,
        }
    }
}


impl Incremental<f64, f64> for MyComponent {
    fn next(&mut self, current_value: f64) -> f64 {
        let diff = current_value - self.previous_value;
        self.previous_value = current_value;
        return result;
    }
}
```

Here is an example of an empty input, but with `float64` as an output.

```rust
impl Incremental<(), f64> for MyComponent {
    fn next(&mut self, _: ()) -> f64 {
        let current_value = self.ctx.bar.close().unwrap();
        let diff = current_value - self.previous_value;
        self.previous_value = current_value;
        return result;
    }
}
```

## 4. Executing context

There are a few ways to run context with components, but all of them require you to call `.next()` method on all components, except on `ctx` itself.

```rust
let ctx = Context::new(...)
let component = MyComponent::new(ctx.clone());
...
```

- ## 4.1 - `ctx` iterator

  The least performant, but the most convenient way. Updates bar tick automatically.

  ```rust
  for tick in ctx.clone() {
      let output = component.next(ctx.bar.close());
  }
  ```

- ## 4.2 - For loop

  The most performant, but requires you to set bar index manually and keep execution within the range of bars.

  ```rust
  for tick in ctx.first_bar_index..=ctx.last_bar_index {
      ctx.bar.index.set(i);
      target.next(ctx.bar.close());
  }
  ```

## 5. Passing component to another component

You can pass data from one component to another component in `next` method or you can pass the ownership of the entire component. It's also useful for indicators that usually don't accept any input values.

`Incremental.wrap()` - wraps the component into `Box`, which allows the component to be passed around without knowing the type.

```rust
struct MyComponentConfig {
    src: AnySrc,
    ma: AnyMa,
}

struct MyComponent {
    ctx: Context,
    prev_src: Option<f64>,
}

impl MyComponent {
    fn new(ctx: Context, config: MyComponentConfig) -> Self {
        Self {
            ctx,
            config,
            prev_src: None,
        }
    }
}

impl Incremental<(), f64> for MyComponent {
    fn next(&mut self, _: ()) -> f64 {
        let src = self.config.src.next(self.ctx.bar.close());
        let ma = self.config.ma.next(src);
        let diff = ps_diff(src - ma);
        self.prev_src = Some(src);
        return diff;
    }
}
```

```rust
let src: AnySrc = Src::new(ctx.clone(), SrcKind::OHLC4).to_box();
let ma: AnyMa = Ma::new(ctx.clone(), MaKind::EMA, 12).to_box();

let component = MyComponent::new(ctx.clone(), MyComponentConfig { src, ma });

for _ in ctx.clone() {
    let output = component.next(());
}
```

`AnySrc` and `AnyMa` are provided by Pace, but they are basically an alias type for `Incremental` trait, wrapped in `Box`.

```rust
pub type AnySrc = Box<dyn Incremental<(), Option<f64>>>
pub type AnyMa = Box<dyn Incremental<Option<f64>, Option<f64>>>;
```

## 6. Creating a strategy

Creating a new strategy is pretty straitforward. There isn't any special trait that you need to implement, apart from `Incremental` trait.

You can look at how Pace default strategies are implemented. RSI strategy example is [here](/pace/src/content/relative_strength_index.rs).

## 7. Executing a strategy

Create a [`Strategy`](/pace/src/strategy/strategy.rs) component that handles all the internal logic like managing trades, calculating PnL, equity, etc.

```rust
let strategy = Strategy::new(ctx.clone(), StrategyConfig {
    initial_capital: 1000.0,
    ..StrategyConfig::default()
});
```

You need to call `strategy.next(trade: Option<TradeDirection>)` method, so it acknowledges the signal.

```rust
for _ in ctx.clone() {
    strategy.next(Some(TradeDirection::Long));
}
```

```rust
enum TradeDirection {
    Long,
    Short,
}
```

You can easily chain an indicator, strategy and main strategy component together.

```rust
let mut rsi_indicator = RelativeStrengthIndex::new(
    ctx.clone(),
    RelativeStrengthIndexConfig::default(ctx.clone()),
);
let mut rsi_strategy = RelativeStrengthIndexStrategy::new(
    ctx.clone(),
    RelativeStrengthIndexStrategyConfig::default(),
);

for _ in ctx.clone() {
    let rsi = rsi_indicator.next(());
    let rsi_signal = rsi_strategy.next(rsi);
    strategy.next(rsi_signal);
}
```

## 8. Strategy metrics

Pace provides basic strategy metrics accessible through `strategy.metrics`. If you want to use more sophisticated metrics, you can create your own metrics component by implementing `Incremental` trait or use built-in metrics ported from TradingView.

```rust
let mut tradingview_metrics = TradingViewMetrics::new(ctx.clone(), &strategy, TradingViewMetricsConfig::default());

for _ in ctx.clone() {
    tradingview_metrics.next(&strategy);
}

tradingview_metrics.data.print_summary("USD");
```
