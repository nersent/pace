use crate::{
    components::{component::Component, component_context::ComponentContext},
    statistics::{mean_component::MeanComponent, stdev_component::StdevComponent},
};

pub struct ReturnsData {
    pub delta: f64,
    pub stdev: f64,
    pub mean: f64,
}

impl Default for ReturnsData {
    fn default() -> Self {
        return Self {
            delta: 0.0,
            stdev: 0.0,
            mean: 0.0,
        };
    }
}

pub struct ReturnsComponent {
    pub ctx: ComponentContext,
    pub data: ReturnsData,
    prev_value: f64,
    stdev: StdevComponent,
    mean: MeanComponent,
}

impl ReturnsComponent {
    pub fn new(ctx: ComponentContext, initial_value: f64) -> Self {
        return Self::build(ctx, initial_value, false);
    }

    pub fn build(ctx: ComponentContext, initial_value: f64, fast: bool) -> Self {
        return Self {
            ctx: ctx.clone(),
            data: ReturnsData::default(),
            prev_value: initial_value,
            stdev: StdevComponent::build(ctx.clone(), fast),
            mean: MeanComponent::new(ctx.clone()),
        };
    }
}

impl Component<f64, ()> for ReturnsComponent {
    fn next(&mut self, value: f64) {
        self.data.delta = value - self.prev_value;
        self.data.stdev = self.stdev.next(self.data.delta);
        self.data.mean = self.mean.next(self.data.delta);
    }
}
