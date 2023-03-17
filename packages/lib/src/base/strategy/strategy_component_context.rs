use std::{
    cell::{Ref, RefCell, RefMut},
    iter::Iterator,
    rc::Rc,
    sync::Arc,
};

use polars::prelude::DataFrame;
use pyo3::PyRef;

use crate::base::{
    asset::{
        asset_data_provider::{self, AssetDataProvider},
        in_memory_asset_data_provider::InMemoryAssetDataProvider,
        timeframe::Timeframe,
    },
    components::component_context::ComponentContext,
    execution_context::ExecutionContext,
};

use super::strategy_execution_context::StrategyExecutionContext;

pub struct StrategyComponentContext {
    ctx: ComponentContext,
    strategy_ctx: Rc<RefCell<StrategyExecutionContext>>,
}

impl StrategyComponentContext {
    pub fn new(ctx: ComponentContext, strategy_ctx: Rc<RefCell<StrategyExecutionContext>>) -> Self {
        return Self { ctx, strategy_ctx };
    }

    pub fn build(ctx: ComponentContext, execution_context: StrategyExecutionContext) -> Self {
        return Self::new(ctx, Rc::new(RefCell::new(execution_context)));
    }

    pub fn get(&self) -> Ref<StrategyExecutionContext> {
        return self.strategy_ctx.borrow();
    }

    pub fn get_mutable(&mut self) -> RefMut<StrategyExecutionContext> {
        return self.strategy_ctx.borrow_mut();
    }

    pub fn clone(&self) -> Self {
        return Self::new(self.ctx.clone(), Rc::clone(&self.strategy_ctx));
    }
}
