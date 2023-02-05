use crate::base::{
    asset::source::{Source, SourceKind},
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::{
        sma_component::SimpleMovingAverageComponent, stdev_component::StandardDeviationComponent,
    },
};

pub struct BollingerBandsPercentBIndicatorConfig {
    pub length: usize,
    pub source: Source,
    pub mult: f64,
}

impl ComponentDefault for BollingerBandsPercentBIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            length: 20,
            source: Source::from_kind(ctx.clone(), SourceKind::Close),
            mult: BOLLINGER_BANDS_PERCENT_B_MULT,
        }
    }
}

pub struct BollingerBandsPercentBIndicator {
    pub config: BollingerBandsPercentBIndicatorConfig,
    ctx: ComponentContext,
    basis: SimpleMovingAverageComponent,
    stdev: StandardDeviationComponent,
}

pub static BOLLINGER_BANDS_PERCENT_B_MULT: f64 = 2.0;

impl BollingerBandsPercentBIndicator {
    pub fn new(ctx: ComponentContext, config: BollingerBandsPercentBIndicatorConfig) -> Self {
        return BollingerBandsPercentBIndicator {
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

        let src = src.unwrap();
        let basis = basis.unwrap();
        let dev = dev.unwrap() * self.config.mult;
        let upper = basis + dev;
        let lower = basis - dev;
        let upper_lower_diff = upper - lower;

        if upper_lower_diff == 0.0 {
            return None;
        }

        let bbr = (src - lower) / upper_lower_diff;

        return Some(bbr);
    }
}
