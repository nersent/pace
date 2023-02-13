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
    execution_context::ExecutionContext,
};

pub struct ComponentContextConfig {
    pub intermittent: bool,
}

pub struct ComponentContext {
    pub config: ComponentContextConfig,
    tick: Option<usize>,
    last_computation_tick: Option<usize>,
    execution_context: Rc<RefCell<ExecutionContext>>,
}

impl ComponentContext {
    pub fn new(execution_context: Rc<RefCell<ExecutionContext>>) -> Self {
        return Self {
            execution_context,
            tick: None,
            last_computation_tick: None,
            config: ComponentContextConfig {
                intermittent: false,
            },
        };
    }

    pub fn new_intermittent(execution_context: Rc<RefCell<ExecutionContext>>) -> Self {
        return Self {
            execution_context,
            tick: None,
            last_computation_tick: None,
            config: ComponentContextConfig { intermittent: true },
        };
    }

    pub fn build(execution_context: ExecutionContext) -> Self {
        return Self::new(Rc::new(RefCell::new(execution_context)));
    }

    pub fn build_from_df(df: &DataFrame, asset_name: &str, timeframe: Timeframe) -> Self {
        let execution_context = ExecutionContext::from_asset(Arc::from(
            InMemoryAssetDataProvider::from_df(df, asset_name, timeframe),
        ));
        return Self::build(execution_context);
    }

    pub fn from_asset_data_provider(
        asset_data_provider: Arc<dyn AssetDataProvider + 'static + Send + Sync>,
    ) -> Self {
        let execution_context = ExecutionContext::from_asset(asset_data_provider);
        return Self::build(execution_context);
    }

    pub fn get(&self) -> Ref<ExecutionContext> {
        return self.execution_context.borrow();
    }

    pub fn get_mutable(&mut self) -> RefMut<ExecutionContext> {
        return self.execution_context.borrow_mut();
    }

    pub fn clone(&self) -> Self {
        return Self::new(Rc::clone(&self.execution_context));
    }

    #[cfg_attr(not(debug_assertions), allow(dead_code))]
    pub fn assert(&mut self) {
        let current_tick = self.get().current_tick;
        if let Some(last_computation_tick) = self.last_computation_tick {
            assert!(
                last_computation_tick + 1 == current_tick,
                "Component tries to compute value for {}, but last computation was for {}",
                current_tick,
                last_computation_tick
            );
        }
        self.last_computation_tick = Some(current_tick);
    }

    #[cfg_attr(not(debug_assertions), allow(dead_code))]
    fn _assert(&mut self, current_tick: usize) {
        if !self.config.intermittent {
            if let Some(tick) = self.tick {
                assert!(
                    tick + 1 == current_tick,
                    "Component tries to compute value for {}, but last computation was for {}",
                    current_tick,
                    tick
                );
            }
        }
    }

    pub fn on_next(&mut self) {
        let current_tick = self.get().current_tick;
        self._assert(current_tick);
        self.tick = Some(current_tick);
    }

    pub fn at_length(&self, length: usize) -> bool {
        return self.tick.unwrap() >= length - 1;
    }
}

impl Iterator for ComponentContext {
    type Item = ComponentContext;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ctx = self.clone();
        if ctx.get_mutable().next() {
            return Some(ctx);
        }
        return None;
    }
}
