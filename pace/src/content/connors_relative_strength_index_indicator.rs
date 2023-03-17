use crate::{
    components::{
        component::Component,
        component_context::ComponentContext,
        component_default::ComponentDefault,
        src_component::{AnySrcComponent, SrcComponent},
        src_kind::SrcKind,
    },
    pinescript::math::ps_nz,
    ta::{
        prank_component::PrankComponent, roc_component::RocComponent, rsi_component::RsiComponent,
    },
};

pub static CRSI_MIN_VALUE: f64 = 0.0;
pub static CRSI_MAX_VALUE: f64 = 100.0;

pub struct CrsiIndicatorConfig {
    pub length_rsi: usize,
    pub length_up_down: usize,
    pub length_roc: usize,
    pub src: AnySrcComponent,
}

impl ComponentDefault for CrsiIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        return Self {
            length_rsi: 3,
            length_up_down: 2,
            length_roc: 100,
            src: SrcComponent::build(ctx.clone(), SrcKind::Close),
        };
    }
}

pub struct CrsiIndicator {
    pub config: CrsiIndicatorConfig,
    pub ctx: ComponentContext,
    prev_src: Option<f64>,
    prev_ud: Option<f64>,
    rsi: RsiComponent,
    up_down_rsi: RsiComponent,
    percent_rank: PrankComponent,
    roc: RocComponent,
}

impl CrsiIndicator {
    pub fn new(ctx: ComponentContext, config: CrsiIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            rsi: RsiComponent::new(ctx.clone(), config.length_rsi),
            up_down_rsi: RsiComponent::new(ctx.clone(), config.length_up_down),
            percent_rank: PrankComponent::new(ctx.clone(), config.length_roc),
            roc: RocComponent::new(ctx.clone(), 1),
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
}

impl Component<(), Option<f64>> for CrsiIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());

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
