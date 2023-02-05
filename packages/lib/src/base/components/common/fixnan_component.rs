use crate::base::components::component_context::ComponentContext;

pub struct FixNanComponent {
    ctx: ComponentContext,
    last_non_nan_value: Option<f64>,
}

impl FixNanComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        return FixNanComponent {
            ctx: ctx.clone(),
            last_non_nan_value: None,
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();

        match value {
            Some(value) => {
                self.last_non_nan_value = Some(value);
                return Some(value);
            }
            None => {
                return self.last_non_nan_value;
            }
        }
    }
}
