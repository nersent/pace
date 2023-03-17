use super::component_context::ComponentContext;

pub trait ComponentDefault {
    fn default(ctx: ComponentContext) -> Self;
}
