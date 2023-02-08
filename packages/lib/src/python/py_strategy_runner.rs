use std::sync::Arc;

use crate::{
    base::{
        components::{component_context::ComponentContext, python::ComponentFromPyDict},
        strategy::{
            strategy_execution_context::{
                StrategyExecutionContext, StrategyExecutionContextConfig,
            },
            strategy_runner::StrategyRunner,
        },
    },
    content::{
        relative_strength_index_indicator::{
            RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig,
        },
        relative_strength_index_strategy::{
            RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
        },
    },
    python::py_asset_data_provider::PyAssetDataProvider,
};
use pyo3::{prelude::*, types::PyDict};

use super::cast::{FromPyRef, PyAnyCast};

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn run_relative_strength_index_strategy(
    asset_data_provider: PyRef<'_, PyAssetDataProvider>,
) -> String {
    return asset_data_provider.get_asset_name();
}

pub struct PyStrategyRunnerConfig {
    on_bar_close: bool,
    continous: bool,
}

impl PyStrategyRunnerConfig {
    pub fn from_dict(config: &PyDict) -> Self {
        let on_bar_close = config
            .get_item("on_bar_close")
            .unwrap()
            .extract::<bool>()
            .unwrap();

        let continous = config
            .get_item("continous")
            .unwrap()
            .extract::<bool>()
            .unwrap();

        // let track_metrics = config
        //     .get_item("track_metrics")
        //     .unwrap()
        //     .extract::<bool>()
        //     .unwrap();

        return Self {
            on_bar_close,
            continous,
            // track_metrics,
        };
    }
}

#[pyclass(name = "StrategyRunner")]
pub struct PyStrategyRunner {
    config: PyStrategyRunnerConfig,
}

#[pymethods]
impl PyStrategyRunner {
    #[new]
    pub fn new(config: &PyDict) -> Self {
        let config = PyStrategyRunnerConfig::from_dict(config);
        return Self { config };
    }

    // fn create_ctx(self, wrapper: PyRef<'_, PyAssetDataProvider>) -> ComponentContext {
    //     let ctx = ComponentContext::from_asset_data_provider(Arc::from(wrapper.asset));
    //     return context;
    // }

    pub fn run_relative_strength_index(
        &self,
        asset_data_provider: PyRef<'_, PyAssetDataProvider>,
        i_config: &PyDict,
        s_config: &PyDict,
    ) -> String {
        let ctx = ComponentContext::from_py_ref(asset_data_provider);

        let i_config = RelativeStrengthIndexIndicatorConfig {
            length: i_config.get_item("length").unwrap().to_usize(),
            src: i_config.get_item("src").unwrap().to_src(ctx.clone()),
        };
        let s_config = RelativeStrengthIndexStrategyConfig {
            threshold_overbought: s_config.get_item("threshold_oversold").unwrap().to_f64(),
            threshold_oversold: s_config.get_item("threshold_overbought").unwrap().to_f64(),
        };

        let indicator = &mut RelativeStrengthIndexIndicator::new(ctx.clone(), i_config);
        let strategy = &mut RelativeStrengthIndexStrategy::new(ctx.clone(), s_config);

        let mut strategy_runner = StrategyRunner::new(
            ctx.clone(),
            StrategyExecutionContext::new(
                ctx.clone(),
                StrategyExecutionContextConfig {
                    on_bar_close: false,
                    continous: false,
                },
            ),
            || {
                let i = indicator.next();
                let s = strategy.next(i);
                return s;
            },
        );

        // for cctx in ctx {
        //     let ctx = cctx.get();
        //     let i = indicator.next();
        //     let s = strategy.next(i);
        // }
        // let mut context = ComponentContext::new();
        // context.register_asset_data_provider(asset_data_provider.asset);

        // let strategy = Strategy::new(id, &mut context);
        // strategy.run();

        return "Hello".to_string();
        // return Ok(());
    }
}
