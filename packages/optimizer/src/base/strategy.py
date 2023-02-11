from abc import ABC
from packages.optimizer.src.pace_glue.strategy_runner import StrategyRunnerConfig


class StrategyModel(ABC):
    def __init__(self, config: StrategyRunnerConfig):
        self.config = config
