use crate::base::{
    asset::source::{Source, SourceKind},
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::{
        dev_component::DeviationComponent, ma::MovingAverageKind,
        sma_component::SimpleMovingAverageComponent,
    },
};

pub struct CommodityChannelIndexIndicatorConfig {
    pub length: usize,
    pub src: Source,
    pub ma_kind: MovingAverageKind,
}

impl ComponentDefault for CommodityChannelIndexIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            length: 20,
            src: Source::from_kind(ctx.clone(), SourceKind::HLC3),
            ma_kind: MovingAverageKind::SMA,
        }
    }
}

pub struct CommodityChannelIndexIndicator {
    pub config: CommodityChannelIndexIndicatorConfig,
    ctx: ComponentContext,
    sma: SimpleMovingAverageComponent,
    dev: DeviationComponent,
}

impl CommodityChannelIndexIndicator {
    pub fn new(ctx: ComponentContext, config: CommodityChannelIndexIndicatorConfig) -> Self {
        return CommodityChannelIndexIndicator {
            ctx: ctx.clone(),
            sma: SimpleMovingAverageComponent::new(ctx.clone(), config.length),
            dev: DeviationComponent::new(ctx.clone(), config.length),
            config,
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let src = self.config.src.get();
        let ma = self.sma.next(src);
        let dev = self.dev.next(src);

        let cci = match (src, ma, dev) {
            (Some(src), Some(ma), Some(dev)) => {
                if dev == 0.0 {
                    None
                } else {
                    Some((src - ma) / (0.015 * dev))
                }
            }
            _ => None,
        };

        return cci;
    }
}
