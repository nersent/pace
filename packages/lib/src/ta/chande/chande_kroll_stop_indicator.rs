use crate::{
    components::{
        component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO, source::Source,
        sum::recursive_sum::RecursiveSum,
        value_cache::fixed_value_cache_component::FixedValueCacheComponent,
    },
    ta::{
        bars::utils::BarUtils,
        moving_average::{ma::MovingAverageKind, ma_component::MovingAverageComponent},
        true_range::atr_component::AverageTrueRangeComponent,
    },
};

pub struct ChandeKrollStopIndicatorConfig {
    pub p: usize,
    pub q: usize,
    pub x: f64,
}

pub struct ChandeKrollStopIndicatorResult {
    pub first_high_stop: Option<f64>,
    pub first_low_stop: Option<f64>,
    pub stop_long: Option<f64>,
    pub stop_short: Option<f64>,
}

pub struct ChandeKrollStopIndicator {
    pub config: ChandeKrollStopIndicatorConfig,
    ctx: ComponentContext,
    atr: AverageTrueRangeComponent,
    first_high_stop_highest_cache: FixedValueCacheComponent,
    first_low_stop_lowest_cache: FixedValueCacheComponent,
}

impl ChandeKrollStopIndicator {
    pub fn new(ctx: ComponentContext, config: ChandeKrollStopIndicatorConfig) -> Self {
        return ChandeKrollStopIndicator {
            ctx: ctx.clone(),
            atr: AverageTrueRangeComponent::new(ctx.clone(), config.p),
            first_high_stop_highest_cache: FixedValueCacheComponent::new(ctx.clone(), config.q),
            first_low_stop_lowest_cache: FixedValueCacheComponent::new(ctx.clone(), config.q),
            config,
        };
    }

    pub fn next(&mut self) -> ChandeKrollStopIndicatorResult {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let atr = self.atr.next();

        if atr.is_none() {
            self.first_low_stop_lowest_cache.next(None);
            self.first_high_stop_highest_cache.next(None);
            return ChandeKrollStopIndicatorResult {
                first_high_stop: None,
                first_low_stop: None,
                stop_long: None,
                stop_short: None,
            };
        }

        let atr = atr.unwrap();

        let (first_high_stop, first_low_stop) = if ctx.at_length(self.config.p) {
            let first_high_stop =
                BarUtils::highest(ctx.prev_highs(self.config.p)).unwrap() - self.config.x * atr;
            let first_low_stop =
                BarUtils::lowest(ctx.prev_lows(self.config.p)).unwrap() + self.config.x * atr;

            (Some(first_high_stop), Some(first_low_stop))
        } else {
            (None, None)
        };

        self.first_high_stop_highest_cache.next(first_high_stop);
        self.first_low_stop_lowest_cache.next(first_low_stop);

        if !ctx.at_length(self.config.q) {
            return ChandeKrollStopIndicatorResult {
                first_high_stop,
                first_low_stop,
                stop_long: None,
                stop_short: None,
            };
        }

        let stop_short = BarUtils::highest(self.first_high_stop_highest_cache.all());
        let stop_long = BarUtils::lowest(self.first_low_stop_lowest_cache.all());

        return ChandeKrollStopIndicatorResult {
            first_high_stop,
            first_low_stop,
            stop_short,
            stop_long,
        };
    }
}
