# Pace

[![Discord](https://discordapp.com/api/guilds/307605794680209409/widget.png?style=shield)](https://discord.gg/P7Vn4VX)
[![Github](https://img.shields.io/github/followers/xnerhu.svg?style=social&label=Follow)](https://github.com/xnerhu)

Pace is a technical analysis library written in Rust, compatible with PineScript, designed to be as close to bare metal as possible.

Fast, memory-safe and cross-platform, but with very high learning curve.

The core feature of Pace is incremental architecture, which allows O(1) constant-time updates of indicators, making it ideal for time sensitive low-frequency live trading.

> Note: Pace is in early development stage. Expect breaking changes.

## PineScript

Pace indicators have been tested against TradingView PineScript indicators, meaning you should expect the same results.

See [migration from PineScript](#pinescript-migration)

## Roadmap

- [ ] Improve performance
- [ ] Release stable version
- [ ] Additionaly support vectorized calculations

## Features

- PineScript compatible

- TradingView strategy runner compatible

- Default technical indicators and strategies from TradingView
  - Aroon
  - Average True Range
  - Awesome Oscillator
  - Balance of Power
  - Bollinger Bands %B
  - Bollinger Bands Width
  - Chaikin Money Flow
  - Chande Kroll Stop
  - Choppiness Index
  - Commodity Channel Index
  - Connors Relative Strength Index
  - Coppock Curve
  - Cross
  - Deviation
  - Directional Movement Index
  - Donchian Channels
  - Exponential Moving Average
  - Highest Bars
  - Lowest Bars
  - Lowest Bars
  - MACD
  - Percent Rank
  - Price Oscillator
  - Rate of Change
  - Relative Strength Index
  - Relative Vigor Index
  - Relative Volatility Index
  - Simple Moving Averageq
  - Standard Deviation
  - Stoch Relative Strength Index
  - Stochastic
  - Sum
  - Symmetrically Weighted Moving Average
  - True Range
  - Ultimate Oscillator
  - Volume Oscillator
  - Vortex Indicator
  - Weighted Moving Average
  - Williams %R

## Getting Started

You can use already configured [boilerplate project](https://github.com/nersent/pace-starter) with example indicator and strategy.

## Installation

1. Download Rust and Cargo

- https://doc.rust-lang.org/book/ch01-01-installation.html

2. Add Pace to your Cargo.toml

```bash
$ cargo add nersent-pace
```

## Documentation

Visit [docs](/docs/index.md) to view the full documentation.

## Motivation

1. State-of-the art

   One of the SOTA technical analysis libraries is the industry golden standard [talib](https://ta-lib.org/), which is written in C and is tailored towards vector calculations. The problem with talib is that it's old (started in 1999) and unmaintained. Adding new features without sacrificing the performance is a great challenge and requires C expertise.

2. PineScript

   No TA lib tries to be compatible with PineScript, which is the most popular way of creating trading strategies. This is crucial as the majority of trading strategies are written in PineScript. Users are limited by TradingView capabilities as there is no way to efficiently optimize strategy parameters.

3. Architecture

   There are different ways of calculating technical analysis indicators such as:

   a. Plain loop

   For each indicator, a loop is performed over the entire dataset. This is the most straightforward way of calculating indicators. It is easy to implement, but it is very slow and memory inefficient.

   b. Vectorization

   For each indicator, a loop is performed over the entire dataset, but the loop is vectorized, meaning CPU instructions are executed in parallel, leading to a significant performance boost. talib is an example of this approach. Vectorization is the best for batch processing, but it is hard to implement and maintain, as any new feature requires to be vectorized as well, which is not always possible. Also, if you want to compute multiple indicators, you have to perform multiple loops.

   c. GPU

   Similar to vectorization, but the calculations are performed on GPU, which is even faster than CPU. it's even harder to implement and maintain than vectorization. Keeping in mind, that there are tools and libraries that don't require you to write custom GPU kernels such as cupy/PyTorch, GPU TA still comes with it's own set of problems. It requires you to have GPU supporting software like CUDA and VRAM large enough. Also copying data from CPU to GPU and back is a performance bottleneck, which may be problematic for parameter optimization.

   d. Incremental

   There is only one main loop, which iterates over the entire dataset. For every loop tick, each indicator is updated in constant O(1) time. This is the best approach if you care about computation delay, as it is fast and memory efficient. In the most cases, it's the easiest way to implement new features. Also, PineScript is incremental by design, so it's easy to port PineScript code to similar architecture.

   Incremental architecture, has it's drawback - it is slower than vectorization, but not that much. For 1M bars and SMA(14) the difference between Pace and talib is 5ms.

   See [benchmarks](#benchmarks) section below for more details.

## Benchmarks

We performed multiple benchmarks for popular indicators across different technical analysis libraries:

- [TA-Lib](https://github.com/TA-Lib/ta-lib-python)
- [Pandas TA](https://github.com/twopirllc/pandas-ta)
- [TALIpp](https://github.com/nardew/talipp)
- [FinTA](https://github.com/peerchemist/finta)
- [spectre](https://github.com/Heerozh/spectre)
- [TA](https://github.com/bukosabino/ta)

### Details:

- AMD Ryzen 5 3600
- 16GB RAM
- RTX 2060 Super 8GB VRAM
- Windows 11
- Rustc 1.67.0
- Python 3.9
- Pace benchmark uses [mimalloc allocator](https://github.com/purpleprotocol/mimalloc_rust) and has been compiled with [cargo pgo](https://github.com/Kobzol/cargo-pgo)

### Interpretation:

- talib is the fastest library across all benchmarks, thanks to vectorization
- Pace is the second fastest library, despite not being designed towards vector calculations
- On smaller datasets, the gap between talib and Pace is not significant

### Mean time (ms):

> Note: The less the better

**Linear scale**:

![1k_bars_log10](/static/benchmarks/1000_mean_time.png)

![1m_bars_log10](/static/benchmarks/1000000_mean_time.png)

**Logarithmic scale**:

![1k_bars_log10](/static/benchmarks/1000_mean_time_log10.png)

![1m_bars_log10](/static/benchmarks/1000000_mean_time_log10.png)

### Mean time difference compared to Pace (ms):

> Note: The more the better

**Linear scale**:

![1k_bars_log10](/static/benchmarks/1000_time_diff.png)

![1m_bars_log10](/static/benchmarks/1000000_time_diff.png)

---

Made by [Nersent](https://nersent.com)
