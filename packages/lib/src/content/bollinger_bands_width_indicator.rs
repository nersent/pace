use crate::base::{
    asset::source::{Source, SourceKind},
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::{
        sma_component::SimpleMovingAverageComponent, stdev_component::StandardDeviationComponent,
    },
};

pub struct BollingerBandsWidthIndicatorConfig {
    pub length: usize,
    pub source: Source,
    pub mult: f64,
}

impl ComponentDefault for BollingerBandsWidthIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            length: 20,
            source: Source::from_kind(ctx.clone(), SourceKind::Close),
            mult: BOLLINGER_BANDS_WIDTH_MULT,
        }
    }
}

pub struct BollingerBandsWidthIndicator {
    pub config: BollingerBandsWidthIndicatorConfig,
    ctx: ComponentContext,
    basis: SimpleMovingAverageComponent,
    stdev: StandardDeviationComponent,
}

pub static BOLLINGER_BANDS_WIDTH_MULT: f64 = 2.0;

impl BollingerBandsWidthIndicator {
    pub fn new(ctx: ComponentContext, config: BollingerBandsWidthIndicatorConfig) -> Self {
        return BollingerBandsWidthIndicator {
            ctx: ctx.clone(),
            basis: SimpleMovingAverageComponent::new(ctx.clone(), config.length),
            stdev: StandardDeviationComponent::new(ctx.clone(), config.length, true),
            config,
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        self.ctx.assert();

        let ctx = self.ctx.get();

        let src = self.config.source.get();
        let basis = self.basis.next(src);
        let dev = self.stdev.next(src);

        if src.is_none() || basis.is_none() || dev.is_none() {
            return None;
        }

        let basis = basis.unwrap();

        if basis == 0.0 {
            return None;
        }

        let src = src.unwrap();
        let dev = dev.unwrap() * self.config.mult;
        let upper = basis + dev;
        let lower = basis - dev;
        let bbw = (upper - lower) / basis;

        return Some(bbw);
    }
}
