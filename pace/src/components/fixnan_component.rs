use super::{component::Component, component_context::ComponentContext};

/// Fixes NaN values by replacing them with the last non-NaN value.
pub struct FixNanComponent {
    pub ctx: ComponentContext,
    last_non_nan_value: Option<f64>,
}

impl FixNanComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        return FixNanComponent {
            ctx: ctx.clone(),
            last_non_nan_value: None,
        };
    }
}

impl Component<Option<f64>, Option<f64>> for FixNanComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
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
