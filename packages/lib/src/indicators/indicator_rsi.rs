use crate::base::{
    component_context::ComponentContext,
    implicit::{
        recursive::recursive_rsi::{RecursiveRSI, RecursiveRSIResult},
        source::Source,
    },
};

pub struct IndicatorRSIConfig {
    pub length: usize,
    pub src: Source,
}

pub struct IndicatorRSI {
    config: IndicatorRSIConfig,
    ctx: ComponentContext,
    rsi: RecursiveRSI,
}

pub type IndicatorRSIResult = RecursiveRSIResult;

pub static INDICATOR_RSI_MAX_VALUE: f64 = 100.0;
pub static INDICATOR_RSI_MIN_VALUE: f64 = 0.0;

impl IndicatorRSI {
    pub fn new(ctx: ComponentContext, config: IndicatorRSIConfig) -> Self {
        return IndicatorRSI {
            ctx: ctx.clone(),
            rsi: RecursiveRSI::new(ctx.clone(), config.length),
            config,
        };
    }

    pub fn next(&mut self) -> IndicatorRSIResult {
        self.ctx.assert();
        let src = self.config.src.get();
        return self.rsi.next(src);
    }
}
