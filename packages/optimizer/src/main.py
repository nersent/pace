import json
from math import sqrt
import sys
import time
from os import path
from typing import TypedDict
from matplotlib import pyplot as plt
from tqdm import tqdm
from pace_glue.asset_data_provider import AssetDataProvider
from packages.optimizer.src.base.optimization.ga_optimizer import GeneticAlgorithmOptimizer
from packages.optimizer.src.models.relative_strength_index_model import RelativeStrengthIndexModel, RelativeStrengthIndexModelParams
from packages.optimizer.src.pace_glue.strategy_runner import StrategyRunnerConfig, StrategyRunnerResult, TradeDirection
from packages.optimizer.src.pace_glue.timeframe import Timeframe
from packages.optimizer.src.utils.unwrap import unwrap_or
import numpy as np
import pandas as pd
from deepmerge import always_merger


def pinescript_declare_array_from(name: str, values: list[int], type: str = "int"):
    joined_items = ", ".join([str(value) for value in values])
    ps = f"{type}[] {name} = array.from({joined_items})\n"
    return ps


def export_strategy_to_pinescript(
    result: StrategyRunnerResult,
    start_tick: int = 0,
    title: str = "Untitled",
    initial_capital: float = 1000.0,
    overlay: bool = True,
    pyramiding: float = 0,
    default_qty_type: float = 0.0,
    default_qty_value: float = 100,
    risk_free_rate: float = 0.0,
    currency: str = 'USD',
    render_returns: bool = False,
    render_capital: bool = False,
) -> str:
    current_metrics = result["metrics"]
    end_tick: int = current_metrics["tick"]
    trades = result["trades"]

    source: str = ""

    version = "//@version=5"
    header = f"strategy(title='{title}', initial_capital={initial_capital}, currency='{currency}', overlay=false, pyramiding=0, risk_free_rate = {risk_free_rate})"

    source += f"{version}\n{header}\n\n"

    source += f"import EliCobra/CobraMetrics/1 as table\n\n"
    source += f"disp_ind = input.string(\"Equity\", title=\"Display\",options=[\"Strategy\", \"Equity\", \"Open Profit\", \"Gross Profit\", \"Net Profit\"])\n"
    source += f"table.cobraTable()\n"
    source += f"plot(table.curve(disp_ind))\n\n"

    offset = -1

    source += f"// --------- CONSTANTS ----------- \n"
    source += f"int __offset = {offset}\n"
    source += f"int __start_tick = {start_tick} + __offset\n"
    source += f"int __end_tick = {end_tick} + __offset\n"

    long_entries: list[int] = []
    long_exits: list[int] = []
    short_entries: list[int] = []
    short_exits: list[int] = []

    for trade in trades:
        if trade["direction"] == TradeDirection.LONG.value:
            long_entries.append(trade["entry_tick"] + offset)
            # long_exits.append(trade["exit_timestamp"])
        else:
            short_entries.append(trade["entry_tick"] + offset)
            # short_exits.append(trade["exit_timestamp"])

    ps_long_entries = pinescript_declare_array_from(
        "__long_entries", long_entries)
    ps_short_entries = pinescript_declare_array_from(
        "__short_entries", short_entries)
    source += f"\n{ps_long_entries}{ps_short_entries}\n\n"

    if render_capital:
        capital_history = np.array(list(
            (map(lambda r: r["equity"], result["metrics_history"]))))
        source += pinescript_declare_array_from(
            "__capital_history", capital_history, "float")
        source += "__capital = bar_index + __offset >= __start_tick + __offset ? bar_index <= __end_tick ? array.get(__capital_history, bar_index - __start_tick) : na : na\n"

    if render_returns:
        returns_history = list(
            (map(lambda r: r["returns"], result["metrics_history"])))
        source += pinescript_declare_array_from(
            "__returns_history", returns_history, "float")
        source += "__returns = bar_index + __offset >= __start_tick + __offset ? bar_index <= __end_tick ? array.get(__returns_history, bar_index - __start_tick) : na : na\n"
    source += f"// ------------------------------- \n"

    source += f"if array.indexof(__long_entries, bar_index) != -1\n"
    source += f"    strategy.entry(\"long\", strategy.long)\n"
    source += f"if array.indexof(__short_entries, bar_index) != -1\n"
    source += f"    strategy.entry(\"short\", strategy.short)\n"

    if render_capital:
        source += "plot(__capital / strategy.initial_capital, title=\"Capital\", color=#ff0000, linewidth=1)\n"
    if render_returns:
        source += "plot(__returns, title=\"Returns\", color=#00ff00, linewidth=1)\n\n"

    print(source)


if (__name__ == "__main__"):
    #     'length': 20, 'src': 6, 'threshold_overbought': 53.137874603271484, 'threshold_oversold': 49.97772979736328}
    # {'tick': 4140, 'time': 1674172800000, 'equity': {'capital': 85879.20000000003, 'returns': 0.004761782193508424, 'trade_pnl': 3961.0}, 'sharpe_ratio': 0.023463764129504204, 'omega_ratio': 79.06954295476659}

    asset_path = path.abspath(
        path.join(path.dirname(__file__), "../../lib/src/ml/fixtures/btc_1d.csv"))

    df = pd.read_csv(asset_path)

    asset = AssetDataProvider(
        asset_path, "BTC_USD", Timeframe.ONE_DAY)

    runner_config: StrategyRunnerConfig = {
        "on_bar_close": False,
        "continous": True,
        "print": False,
        "initial_capital": 1000.0,
        "buy_with_equity": False,
        "metrics": {
            "omega_ratio": {
                "risk_free_rate": 0.0,
            },
            "sharpe_ratio": {
                "risk_free_rate": 0.0,
            },
        },
        "start_tick": 2327,
    }

    generations = 100000
    rsi_model = RelativeStrengthIndexModel(runner_config)

    pbar = tqdm(total=generations)

    def run(params):
        # score = unwrap_or(rsi_model.run(
        #     asset, params)["metrics"]["equity"], 0.0)
        res = rsi_model.run(asset, params)
        metrics = res["metrics"]

        omega = unwrap_or(metrics["omega_ratio"], 0.0)
        closed_trades = metrics["total_closed_trades"]
        omega = min(omega, 100.0) * sqrt(min(closed_trades, 30))

        score = omega

        # score = unwrap_or(rsi_model.run(
        #     asset, params)["metrics"]["sharpe_ratio"], 0.0)
        return score

    strategy_optimizer = GeneticAlgorithmOptimizer(
        params=RelativeStrengthIndexModelParams(),
        generations=generations,
        criterion=lambda params: run(params),
        # criterion=lambda params: unwrap_or(rsi_model.run(
        #     asset, params)["metrics"]["equity"]["capital"], 0.0),
        on_generation=lambda _, __: pbar.update(1),
        # save_solutions=True,
        print_genome=True,
    )

    best_params = strategy_optimizer.run()
    # best_params = {'length': 94, 'src': 0,
    #                'threshold_overbought': 52.0, 'threshold_oversold': 44.0}

    rsi_model = RelativeStrengthIndexModel(always_merger.merge(runner_config, {
        "metrics": {
            "track": True,
        }
    }))

    result = rsi_model.run(asset, best_params)

    print(best_params)
    print(result['metrics'])

    timestamps = np.array(list(
        (map(lambda r: r["time"], result["metrics_history"]))))
    timestamps = pd.to_datetime(timestamps, unit="ms")

    equity_history = np.array(list(
        (map(lambda r: r["equity"], result["metrics_history"]))))

    returns_history = list(
        (map(lambda r: r["returns"], result["metrics_history"])))

    omega_ratio_history = np.array(list(
        (map(lambda r: r["omega_ratio"], result["metrics_history"]))))

    sharpe_ratio_history = np.array(list(
        (map(lambda r: r["sharpe_ratio"], result["metrics_history"]))))

    with open("history_dupsko.json", "w") as file:
        json.dump(result, file, indent=4)

    plt.plot(timestamps, equity_history)
    plt.xlabel("Time")
    plt.ylabel("Equity Curve")
    plt.savefig("plot_capital.png")
    plt.clf()

    plt.plot(timestamps, returns_history)
    plt.xlabel("Time")
    plt.ylabel("Returns")
    plt.savefig("plot_returns.png")
    plt.clf()

    plt.plot(timestamps, omega_ratio_history)
    plt.xlabel("Time")
    plt.ylabel("Omega Ratio")
    plt.savefig("plot_omega_ratio.png")
    plt.clf()

    plt.plot(timestamps, sharpe_ratio_history)
    plt.xlabel("Time")
    plt.ylabel("Sharpe Ratio")
    plt.savefig("plot_sharpe_ratio.png")
    plt.clf()

    plt.close()

    trades = result["trades"]

    res: StrategyRunnerResult = json.load(open("history_dupsko.json"))
    export_strategy_to_pinescript(
        res, title="RSI", initial_capital=1000.0, start_tick=0, render_returns=False, render_capital=True)

    # strategy_optimizer.ga_instance.plot_fitness()
    # strategy_optimizer.ga_instance.plot_result()
    # strategy_optimizer.ga_instance.plot_genes()
    # strategy_optimizer.ga_instance.plot_new_solution_rate()
