use crate::{
    components::{
        component::Component,
        component_context::ComponentContext,
        component_default::ComponentDefault,
        src_component::{AnySrcComponent, SrcComponent},
        src_kind::SrcKind,
    },
    pinescript::common::{ps_abs, ps_diff, ps_max, ps_min},
    ta::sum_component::SumComponent,
};

pub static CMO_MAX_VALUE: f64 = 100.0;

pub struct CmoIndicatorConfig {
    pub length: usize,
    pub src: AnySrcComponent,
}

impl ComponentDefault for CmoIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            length: 9,
            src: Box::new(SrcComponent::new(ctx.clone(), SrcKind::Close)),
        }
    }
}

pub struct CmoIndicator {
    pub config: CmoIndicatorConfig,
    pub ctx: ComponentContext,
    prev_src: Option<f64>,
    prev_m1: Option<f64>,
    prev_m2: Option<f64>,
    sm1: SumComponent,
    sm2: SumComponent,
}

impl CmoIndicator {
    pub fn new(ctx: ComponentContext, config: CmoIndicatorConfig) -> Self {
        assert!(
            config.length > 1,
            "ChandeMomentumOscillatorIndicator length must be greater than 1"
        );
        return Self {
            ctx: ctx.clone(),
            prev_src: None,
            prev_m1: None,
            prev_m2: None,
            sm1: SumComponent::new(ctx.clone(), config.length),
            sm2: SumComponent::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Component<(), Option<f64>> for CmoIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());
        let momm = ps_diff(src, self.prev_src);

        let m1 = ps_max(Some(0.0), momm);
        let m2 = ps_abs(ps_min(Some(0.0), momm));

        let sm1 = self.sm1.next(m1);
        let sm2 = self.sm2.next(m2);

        let chande_mo: Option<f64> = match (sm1, sm2) {
            (Some(sm1), Some(sm2)) => {
                if sm1 == -sm2 {
                    None
                } else {
                    Some(100.0 * (sm1 - sm2) / (sm1 + sm2))
                }
            }
            _ => None,
        };

        self.prev_src = src;

        return chande_mo;
    }
}
