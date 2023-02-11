from packages.optimizer.src.base.optimization.parameter import FloatParam, IntegerParam
from packages.optimizer.src.base.strategy import StrategyModel
from packages.optimizer.src.pace_glue.asset_data_provider import AssetDataProvider

from packages.optimizer.src.pace_glue.source import SOURCE_KINDS, SourceKind
from packages.optimizer.src.pace_glue.strategy_runner import StrategyRunnerResult, cast_to_strategy_runner_result
from pace import pace


class RelativeStrengthIndexModelParams():
    length = IntegerParam(14).min_max(2, 400).step(2)

    src = IntegerParam(SourceKind.CLOSE).list(SOURCE_KINDS)

    threshold_oversold = FloatParam(30.0).min_max(0, 50.0).step(1.0)
    threshold_overbought = FloatParam(70.0).min_max(50.0, 100.0).step(1.0)


class RelativeStrengthIndexModel(StrategyModel):
    def run(
        self,
        asset_data_provider: AssetDataProvider,
        params: dict,
    ) -> StrategyRunnerResult:
        config = {
            "indicator": {
                "length": params["length"],
                "src": params["src"],
            },
            "strategy": {
                "threshold_oversold": params["threshold_oversold"],
                "threshold_overbought": params["threshold_overbought"],
            }
        }
        return cast_to_strategy_runner_result(pace.run_relative_strength_index(asset_data_provider.get(), self.config, config))
