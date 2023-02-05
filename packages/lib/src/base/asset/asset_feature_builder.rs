use std::collections::HashMap;

use crate::base::{components::component_context::ComponentContext, features::feature::Feature};

pub struct AssetFeature {
    pub time: f64,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub volume: Option<f64>,
}

impl Feature for AssetFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            ("time".to_string(), Some(self.time)),
            ("open".to_string(), self.open),
            ("high".to_string(), self.high),
            ("low".to_string(), self.low),
            ("close".to_string(), self.close),
            ("volume".to_string(), self.volume),
        ]);
        return map;
    }
}

pub struct AssetFeatureBuilder {
    ctx: ComponentContext,
}

impl AssetFeatureBuilder {
    pub fn next(&mut self) -> AssetFeature {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let time = ctx.time().unwrap().as_secs_f64();
        let open = ctx.open();
        let high = ctx.high();
        let low = ctx.low();
        let close = ctx.close();
        let volume = ctx.volume();

        return AssetFeature {
            time,
            open,
            high,
            low,
            close,
            volume,
        };
    }
}

impl AssetFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return AssetFeatureBuilder { ctx: ctx.clone() };
    }
}
