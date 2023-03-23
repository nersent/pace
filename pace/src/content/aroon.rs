use crate::{
    core::{context::Context, incremental::Incremental},
    strategy::trade::TradeDirection,
    ta::{cross::Cross, highest_bars::HighestBars, lowest_bars::LowestBars},
};

pub static AROON_MIN_VALUE: f64 = 0.0;
pub static AROON_MAX_VALUE: f64 = 100.0;

pub struct AroonData {
    pub up: Option<f64>,
    pub down: Option<f64>,
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
        if !self.ctx.bar.at_length(self.config.length) {
            return AroonData {
                up: None,
                down: None,
            };
        }

        let high = self.highest_bars.next(());
        let low = self.lowest_bars.next(());

        let length = self.config.length as f64;

        let up = high.map(|high| (high as f64 + length) / length * 100.0);
        let down = low.map(|low| (low as f64 + length) / length * 100.0);

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

impl Incremental<&AroonData, Option<TradeDirection>> for AroonStrategy {
    fn next(&mut self, aroon: &AroonData) -> Option<TradeDirection> {
        self.data.up_trend_strength = match (aroon.up, aroon.down) {
            (Some(up), Some(down)) => {
                if up > 50.0 && down < 50.0 {
                    1.0 - (100.0 - up) / 50.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        };

        self.data.down_trend_strength = match (aroon.up, aroon.down) {
            (Some(up), Some(down)) => {
                if down > 50.0 && up < 50.0 {
                    1.0 - (100.0 - down) / 50.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        };

        let cross = self.cross.next((aroon.down, aroon.up));

        if cross.is_some() {
            self.data.cross_mode = true;
        }

        let mut up_trend_confirmation = false;
        let mut down_trend_confirmation = false;

        if self.data.cross_mode {
            if self.data.up_trend_strength >= 1.0 {
                up_trend_confirmation = true;
                self.data.cross_mode = false;
            } else if self.data.down_trend_strength >= 1.0 {
                down_trend_confirmation = true;
                self.data.cross_mode = false;
            }
        }

        let result = if up_trend_confirmation {
            Some(TradeDirection::Long)
        } else if down_trend_confirmation {
            Some(TradeDirection::Short)
        } else {
            None
        };

        return result;
    }
}
