from enum import Enum
from typing import Optional, TypedDict
from pace import pace

from packages.optimizer.src.pace_glue.asset_data_provider import AssetDataProvider
from packages.optimizer.src.pace_glue.metrics import EquityMetric
from packages.optimizer.src.pace_glue.source import SourceKind


def run_relative_strength_index_strategy(asset_data_provider: AssetDataProvider):
    return pace.run_relative_strength_index_strategy(asset_data_provider._asset_data_provider)


class StrategyConfig(TypedDict):
    on_bar_close: bool
    continous: bool
    # metrics: Optional[EquityMetric]


class StrategyResult(TypedDict):
    metrics: Optional[EquityMetric]


class RelativeStrengthIndexStrategyConfig(TypedDict):
    threshold_oversold: float
    threshold_overbought: float


class RelativeStrengthIndexIndicatorConfig(TypedDict):
    length: int
    src: SourceKind


class StrategyRunner():
    def _create_runner(self, config: StrategyConfig):
        return pace.PyStrategyRunner(config)

    def run_relative_strength_index(
        self,
        asset_data_provider: AssetDataProvider,
        strategy_config: StrategyConfig,
        rsi_strategy_config: RelativeStrengthIndexStrategyConfig,
        rsi_indicator_config: RelativeStrengthIndexIndicatorConfig
    ) -> StrategyResult:
        runner = self._create_runner(strategy_config)
