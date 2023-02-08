from typing import TypedDict


class StrategyMetricConfig(TypedDict):
    track: bool


class EquityMetricConfig(StrategyMetricConfig):
    initial_capital: float


class EquityMetric():
    _RS_ID = "equity"

    def __init__(self, config: EquityMetricConfig):
        self.config = config

    def value(self):
        return 0
