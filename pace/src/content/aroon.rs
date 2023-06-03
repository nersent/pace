use std::collections::HashMap;

use crate::{
    core::features::{FeatureValue, Features, IncrementalFeatureBuilder},
    core::{context::Context, incremental::Incremental, trend::Trend},
    statistics::normalization::rescale,
    strategy::trade::{StrategySignal, TradeDirection},
    ta::{
        cross::{Cross, CrossMode},
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
    },
    utils::float::OptionFloatUtils,
};

pub static AROON_MIN_VALUE: f64 = 0.0;
pub static AROON_MAX_VALUE: f64 = 100.0;

pub struct AroonData {
    pub up: f64,
    pub down: f64,
}

pub struct AroonConfig {
    pub length: usize,
}

impl Default for AroonConfig {
    fn default() -> Self {
        Self { length: 14 }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000501801
pub struct Aroon {
    pub config: AroonConfig,
    pub ctx: Context,
    highest_bars: HighestBars,
    lowest_bars: LowestBars,
}

impl Aroon {
    pub fn new(ctx: Context, config: AroonConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            highest_bars: HighestBars::new(ctx.clone(), config.length),
            lowest_bars: LowestBars::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Incremental<(), AroonData> for Aroon {
    fn next(&mut self, _: ()) -> AroonData {
        let high = self.highest_bars.next(self.ctx.bar.high());
        let low = self.lowest_bars.next(self.ctx.bar.low());

        let length = self.config.length as f64;

        let up = (high.unwrap_nan() + length) / length * 100.0;
        let down = (low.unwrap_nan() + length) / length * 100.0;

        // let up = high.map(|high| (high as f64 + length) / length * 100.0);
        // let down = low.map(|low| (low as f64 + length) / length * 100.0);

        return AroonData { up, down };
    }
}

pub struct AroonStrategyData {
    pub up_trend_strength: f64,
    pub down_trend_strength: f64,
    pub cross_mode: bool,
}

/// Custom Aroon Strategy. May be incorrect.
pub struct AroonStrategy {
    pub ctx: Context,
    pub data: AroonStrategyData,
    cross: Cross,
    up_trend_confirmation: bool,
    down_trend_confirmation: bool,
}

impl AroonStrategy {
    pub fn new(ctx: Context) -> Self {
        return AroonStrategy {
            ctx: ctx.clone(),
            cross: Cross::new(ctx.clone()),
            up_trend_confirmation: false,
            down_trend_confirmation: false,
            data: AroonStrategyData {
                up_trend_strength: 0.0,
                down_trend_strength: 0.0,
                cross_mode: false,
            },
        };
    }
}

impl Incremental<&AroonData, StrategySignal> for AroonStrategy {
    fn next(&mut self, aroon: &AroonData) -> StrategySignal {
        self.data.up_trend_strength = if aroon.up > 50.0 && aroon.down < 50.0 {
            1.0 - (100.0 - aroon.up) / 50.0
        } else {
            0.0
        };

        self.data.down_trend_strength = {
            if aroon.down > 50.0 && aroon.up < 50.0 {
                1.0 - (100.0 - aroon.down) / 50.0
            } else {
                0.0
            }
        };

        let cross = self.cross.next((aroon.up, aroon.down));

        if let Some(cross) = cross {
            if cross == CrossMode::Over {
                return StrategySignal::Long;
            } else if cross == CrossMode::Under {
                return StrategySignal::Short;
            }
        }

        // if cross == Some(CrossMOde::Long) {
        //     self.data.cross_mode = true;
        // } else if cross == Some(TradeDirection::Short) {
        //     self.data.cross_mode = true;
        // }

        // if cross.is_some() {
        //     self.data.cross_mode = true;
        // }

        // let mut up_trend_confirmation = false;
        // let mut down_trend_confirmation = false;

        // if self.data.cross_mode {
        //     if self.data.up_trend_strength >= 1.0 {
        //         up_trend_confirmation = true;
        //         self.data.cross_mode = false;
        //     } else if self.data.down_trend_strength >= 1.0 {
        //         down_trend_confirmation = true;
        //         self.data.cross_mode = false;
        //     }
        // }

        // if up_trend_confirmation {
        //     return StrategySignal::Long;
        // }

        // if down_trend_confirmation {
        //     return StrategySignal::Short;
        // }

        return StrategySignal::Hold;
    }
}
