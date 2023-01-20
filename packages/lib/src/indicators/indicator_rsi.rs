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

use RecursiveRSIResult as IndicatorRSIResult;

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
