from enum import Enum
import sys
from typing import Optional, TypedDict
from pace import pace

from packages.optimizer.src.pace_glue.asset_data_provider import AssetDataProvider
from packages.optimizer.src.pace_glue.metrics import EquityMetric
from packages.optimizer.src.pace_glue.source import SourceKind


class OmegaRatioMetricConfig(TypedDict):
    risk_free_rate: float


class StrategyRunnerMetricsConfig(TypedDict):
    omega_ratio: Optional[OmegaRatioMetricConfig]


class StrategyRunnerConfig(TypedDict):
    continous: Optional[bool]
    on_bar_close: Optional[bool]
    buy_with_equity: Optional[bool]
    initial_capital: Optional[float]
    print: Optional[bool]
    metrics: StrategyRunnerMetricsConfig
    start_tick: Optional[int]
    end_tick: Optional[int]


class StrategyRunnerMetrics(TypedDict):
    tick: int
    time: Optional[int]
    track: Optional[bool]
    equity: float
    open_profit: float
    net_profit: float
    returns: float
    sharpe_ratio: Optional[float]
    omega_ratio: Optional[float]
    total_closed_trades: int


class TradeDirection(int, Enum):
    LONG = 0
    SHORT = 1


class Trade(TypedDict):
    direction: TradeDirection
    is_closed: bool
    entry_tick: Optional[float]
    entry_price: Optional[float]
    exit_tick: Optional[float]
    exit_price: Optional[float]


class StrategyRunnerResult(TypedDict):
    metrics: StrategyRunnerMetrics
    metrics_history: Optional[list[StrategyRunnerMetrics]]
    trades: list[Trade]


def cast_to_strategy_metrics(res) -> StrategyRunnerMetrics:
    return {
        "tick": res.tick,
        "time": res.time,
        "equity": res.equity,
        "open_profit": res.open_profit,
        "net_profit": res.net_profit,
        "returns": res.returns,
        "sharpe_ratio": res.sharpe_ratio,
        "omega_ratio": res.omega_ratio,
        "total_closed_trades": res.total_closed_trades
    }


def cast_to_strategy_runner_result(res) -> StrategyRunnerResult:
    return {
        "metrics": cast_to_strategy_metrics(res.metrics),
        "metrics_history": list(map(cast_to_strategy_metrics, res.metrics_history)) if res.metrics_history is not None else None,
        "trades": list(map(cast_to_trade, res.trades)) if res.trades is not None else None,
    }


def cast_to_trade(res) -> Trade:
    return {
        "direction": TradeDirection(res.direction),
        "is_closed": res.is_closed,
        "entry_tick": res.entry_tick,
        "entry_price": res.entry_price,
        "exit_tick": res.exit_tick,
        "exit_price": res.exit_price,
    }
