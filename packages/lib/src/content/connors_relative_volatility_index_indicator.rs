use crate::base::{
    asset::source::{Source, SourceKind},
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    pinescript::utils::ps_nz,
    ta::{
        prank_component::PercentRankComponent, roc_component::RateOfChangeComponent,
        rsi_component::RelativeStrengthIndexComponent,
    },
};

pub struct ConnorsRelativeStrengthIndexIndicatorConfig {
    pub length_rsi: usize,
    pub length_up_down: usize,
    pub length_roc: usize,
    pub src: Source,
}

impl ComponentDefault for ConnorsRelativeStrengthIndexIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        return ConnorsRelativeStrengthIndexIndicatorConfig {
            length_rsi: 3,
            length_up_down: 2,
            length_roc: 100,
            src: Source::from_kind(ctx, SourceKind::Close),
        };
    }
}

pub struct ConnorsRelativeStrengthIndexIndicator {
    config: ConnorsRelativeStrengthIndexIndicatorConfig,
    ctx: ComponentContext,
    prev_src: Option<f64>,
    prev_ud: Option<f64>,
    rsi: RelativeStrengthIndexComponent,
    up_down_rsi: RelativeStrengthIndexComponent,
    percent_rank: PercentRankComponent,
    roc: RateOfChangeComponent,
}

impl ConnorsRelativeStrengthIndexIndicator {
    pub fn new(ctx: ComponentContext, config: ConnorsRelativeStrengthIndexIndicatorConfig) -> Self {
        return ConnorsRelativeStrengthIndexIndicator {
            ctx: ctx.clone(),
            rsi: RelativeStrengthIndexComponent::new(ctx.clone(), config.length_rsi),
            up_down_rsi: RelativeStrengthIndexComponent::new(ctx.clone(), config.length_up_down),
            percent_rank: PercentRankComponent::new(ctx.clone(), config.length_roc),
            roc: RateOfChangeComponent::new(ctx.clone(), 1),
            config,
            prev_src: None,
            prev_ud: None,
        };
    }

    fn compute_up_down(
        src: Option<f64>,
        prev_src: Option<f64>,
        prev_ud: Option<f64>,
    ) -> Option<f64> {
        if prev_src == src {
            return Some(0.0);
        }
        let prev_ud = ps_nz(prev_ud);
        if src.is_some() && prev_src.is_some() && src.unwrap() > prev_src.unwrap() {
            if prev_ud <= 0.0 {
                return Some(1.0);
            } else {
                return Some(prev_ud + 1.0);
            }
        } else if prev_ud >= 0.0 {
            return Some(-1.0);
        } else {
            return Some(prev_ud - 1.0);
        }
    }

    pub fn next(&mut self) -> Option<f64> {
        self.ctx.assert();
        let src = self.config.src.get();

        let rsi = self.rsi.next(src);

        let up_down = Self::compute_up_down(src, self.prev_src, self.prev_ud);
        let up_down_rsi = self.up_down_rsi.next(up_down);

        let roc = self.roc.next(src);
        let percent_rank = self.percent_rank.next(roc);

        let crsi = match (rsi, up_down_rsi, percent_rank) {
            (Some(rsi), Some(up_down_rsi), Some(percent_rank)) => {
                Some((rsi + up_down_rsi + percent_rank) / 3.0)
            }
            _ => None,
        };

        self.prev_ud = up_down;
        self.prev_src = src;

        return crsi;
    }
}
