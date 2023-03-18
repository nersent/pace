use crate::{
    components::{
        component::Component, component_context::ComponentContext,
        fixnan_component::FixNanComponent,
    },
    pinescript::common::{ps_diff, ps_div},
    ta::{rma_component::RmaComponent, tr_component::TrComponent},
};

pub static DMI_MIN_VALUE: f64 = 0.0;
pub static DMI_MAX_VALUE: f64 = 100.0;

pub struct DmiIndicatorConfig {
    pub length: usize,
    pub lensig: usize,
}

impl Default for DmiIndicatorConfig {
    fn default() -> Self {
        Self {
            length: 14,
            lensig: 14,
        }
    }
}

pub struct DmiIndicatorRes {
    pub plus: Option<f64>,
    pub minus: Option<f64>,
    pub adx: Option<f64>,
}

/// Directional Movement Index Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000502250
pub struct DmiIndicator {
    pub config: DmiIndicatorConfig,
    pub ctx: ComponentContext,
    true_range: TrComponent,
    true_range_rma: RmaComponent,
    plus_dm_rma: RmaComponent,
    minus_dm_rma: RmaComponent,
    plus_fix_nan: FixNanComponent,
    minus_fix_nan: FixNanComponent,
    adx: RmaComponent,
}

impl DmiIndicator {
    pub fn new(ctx: ComponentContext, config: DmiIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            true_range: TrComponent::new(ctx.clone(), false),
            true_range_rma: RmaComponent::new(ctx.clone(), config.length),
            plus_dm_rma: RmaComponent::new(ctx.clone(), config.length),
            minus_dm_rma: RmaComponent::new(ctx.clone(), config.length),
            plus_fix_nan: FixNanComponent::new(ctx.clone()),
            minus_fix_nan: FixNanComponent::new(ctx.clone()),
            adx: RmaComponent::new(ctx.clone(), config.lensig),
            config,
        };
    }
}

impl Component<(), DmiIndicatorRes> for DmiIndicator {
    fn next(&mut self, _: ()) -> DmiIndicatorRes {
        let high = self.ctx.high();
        let low = self.ctx.low();
        let prev_high = self.ctx.prev_high(1);
        let prev_low = self.ctx.prev_low(1);

        let up = ps_diff(high, prev_high);
        let down = ps_diff(prev_low, low);

        let plus_dm = match (up, down) {
            (Some(up), Some(down)) => {
                if up > down && up > 0.0 {
                    Some(up)
                } else {
                    Some(0.0)
                }
            }
            _ => None,
        };

        let minus_dm = match (up, down) {
            (Some(up), Some(down)) => {
                if down > up && down > 0.0 {
                    Some(down)
                } else {
                    Some(0.0)
                }
            }
            _ => None,
        };

        let true_range = self.true_range.next(());
        let true_range_rma = self.true_range_rma.next(true_range);

        let plus_dm_rma = self.plus_dm_rma.next(plus_dm);
        let minus_dm_rma = self.minus_dm_rma.next(minus_dm);

        let plus = ps_div(plus_dm_rma, true_range_rma).map(|x| x * 100.0);
        let minus = ps_div(minus_dm_rma, true_range_rma).map(|x| x * 100.0);

        let plus = self.plus_fix_nan.next(plus);
        let minus = self.minus_fix_nan.next(minus);

        let adx: Option<f64> = match (plus, minus) {
            (Some(plus), Some(minus)) => {
                Some((plus - minus).abs() / (if plus == -minus { 0.0 } else { plus + minus }))
            }
            _ => None,
        };
        let adx = self.adx.next(adx).map(|x| x * 100.0);

        return DmiIndicatorRes { plus, minus, adx };
    }
}
