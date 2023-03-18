import json
from os import listdir
import sys
from typing import Optional, Tuple
from matplotlib import patches
import matplotlib.pyplot as plt
from matplotlib.ticker import FormatStrFormatter, MultipleLocator
import numpy as np

from benchmarks.common import BenchmarkJsonData

lib_labels: dict[str, str] = {
    "pace": "Pace",
    "talib": "TA-Lib",
    "pandas_ta": "Pandas TA",
    "pandas_ta_talib": "Pandas TA (TA-Lib)",
    "ta": "TA",
    "finta": "FinTA",
    "talipp": "TALIpp",
    "spectre_cpu": "spectre (CPU)",
    "spectre_gpu": "spectre (GPU)"
}

lib_colors: dict[str, str] = {
    "pace": "red",
    "talib": "green",
    "pandas_ta_talib": "cyan",
    "pandas_ta": "blue",
    "ta": "yellow",
    "finta": "magenta",
    "talipp": "black",
    "spectre_cpu": "orange",
    "spectre_gpu": "purple"
}

benchmark_labels: dict[str, str] = {
    "sma_14": "SMA(14)",
    "ema_14": "EMA(14)",
    "rsi_14": "RSI(14)",
    "stoch_14": "STOCH(14)",
    "macd_12_26": "MACD(12, 26)",
    "macd_12_26_rsi_14": "MACD(12, 26) + RSI(14)",
    "macd_12_26_rsi_14_aroon_14": "MACD(12, 26) + RSI(14) + AROON(14)",
    "dmi_14": "DMI(14, 14)",
    "aroon_14": "AROON(14)",
    "coppock_curve_length_10_long_14_short_11": "COPPOCK(10, 11, 14)",
}

benchmark_whitelist: list[str] = [
    "sma_14",
    "ema_14",
    "rsi_14",
    "stoch_14",
    "macd_12_26",
    "macd_12_26_rsi_14"
]

benchmark_whitelist_labels = [benchmark_labels[x] for x in benchmark_whitelist]

time_diff_whitelist: list[str] = [
    "talib",
    "pandas_ta_talib",
]


def generate_plot(items: list[BenchmarkJsonData], bars_count: int, log_scale: bool = True) -> Tuple[plt.Axes, plt.Axes]:
    # structurize all lib names into a list, that will be used to index the benchmarks
    lib_ids: list[str] = list(lib_labels.keys())

    # structurize benchmarks to a dictionary, where key is the benchmark name, and value is a list of mean times for each library
    benchmarks_map: dict[str, list[float]] = {}
    min_mean_time = None
    max_mean_time = None

    for item in items:
        lib_name = item["id"]
        lib_id = lib_ids.index(lib_name)

        for benchmark in item["benchmarks"]:
            benchmark_id = benchmark["id"]

            if benchmark_id not in benchmark_whitelist:
                continue

            if benchmark_id not in benchmarks_map:
                benchmarks_map[benchmark_id] = [0] * len(lib_ids)

            benchmarks_map[benchmark_id][lib_id] = benchmark["mean"]

            if max_mean_time is None or benchmark["mean"] > max_mean_time:
                max_mean_time = benchmark["mean"]

            if min_mean_time is None or benchmark["mean"] < min_mean_time:
                min_mean_time = benchmark["mean"]

    benchmarks_mean_times = list(benchmarks_map.values())
    benchmark_ids = list(benchmarks_map.keys())

    lib_to_benchmark_mean_times_dict: dict[str, list[float]] = {
        lib_ids[i]: [x[i] for x in benchmarks_mean_times]
        for i in range(len(lib_ids))
    }

    bar_width = 0.1
    index = np.arange(len(benchmark_ids))
    mean_time_ax = None
    mean_time_fig, mean_time_ax = plt.subplots()

    for i, lib_id in enumerate(lib_to_benchmark_mean_times_dict):
        benchmark_mean_times = lib_to_benchmark_mean_times_dict[lib_id]
        lib_name = lib_labels[lib_id]
        lib_color = lib_colors[lib_id]

        if log_scale:
            benchmark_mean_times = np.log10(np.array(benchmark_mean_times) + 1)

        mean_time_ax.bar(
            index + i * bar_width,
            benchmark_mean_times,
            bar_width,
            label=lib_name,
            color=lib_color,
        )

    mean_time_ax.set_xticks(index + bar_width * (len(lib_ids) - 1) / 2)
    mean_time_ax.set_xticklabels(benchmark_whitelist_labels)

    _increment = 1

    if max_mean_time > 100:
        if not log_scale:
            _increment = 250
        else:
            _increment = 0.1
    elif max_mean_time > 5:
        if not log_scale:
            _increment = 0.25
        else:
            # _increment = 0.025
            _increment = 0.025

    # print(bars_count, max_mean_time, log_scale, _increment)

    mean_time_ax.yaxis.set_major_locator(MultipleLocator(_increment))

    _width = 30
    _ratio = 21 / 9
    mean_time_fig.set_size_inches(_width, int(_width * 1 / _ratio))

    if log_scale:
        mean_time_ax.set_ylabel('log10 Mean Time (ms)')
    else:
        mean_time_ax.set_ylabel('Mean Time (ms)')

    mean_time_ax.set_title(f"Mean time for {bars_count} bars")
    mean_time_ax.legend()

    time_diff_fig, time_diff_ax = plt.subplots()

    pace_benchmark_mean_times = np.array(
        lib_to_benchmark_mean_times_dict["pace"])

    time_diff_min = None
    time_diff_max = None

    _i = 0

    for i, lib_id in enumerate(lib_to_benchmark_mean_times_dict):
        if lib_id not in time_diff_whitelist:
            continue

        benchmark_mean_times = np.array(
            lib_to_benchmark_mean_times_dict[lib_id])

        time_diff: np.ndarray = benchmark_mean_times - pace_benchmark_mean_times

        _time_diff_min = np.min(time_diff)
        _time_diff_max = np.max(time_diff)

        if time_diff_min is None or _time_diff_min < time_diff_min:
            time_diff_min = _time_diff_min

        if time_diff_max is None or _time_diff_max > time_diff_max:
            time_diff_max = _time_diff_max

        lib_name = lib_labels[lib_id]
        lib_color = lib_colors[lib_id]

        time_diff_ax.bar(
            index + _i * bar_width,
            time_diff,
            bar_width,
            label=lib_name,
            color=lib_color,
        )

        _i += 1

    _increment = 2.5
    if time_diff_max < 10:
        _increment = 0.025

    time_diff_ax.yaxis.set_major_locator(MultipleLocator(_increment))

    time_diff_ax.set_xticks(
        index + bar_width * (len(time_diff_whitelist) - 1) / 2)
    time_diff_ax.set_xticklabels(benchmark_whitelist_labels)

    time_diff_ax.axhline(y=0, color='red', linestyle='--')

    _margin = 0
    _opacity = 0.08
    time_diff_ax.axhspan(0, time_diff_max + _margin,
                         facecolor='green', alpha=_opacity)
    time_diff_ax.axhspan(time_diff_min - _margin, 0,
                         facecolor='red', alpha=_opacity)

    time_diff_fig.set_size_inches(_width, int(_width * 1 / _ratio))

    time_diff_ax.set_ylabel('Time difference (ms)')
    time_diff_ax.set_title(f"Time difference for {bars_count} bars")
    time_diff_ax.legend()

    return mean_time_ax, time_diff_ax


if __name__ == "__main__":
    dir_path = "benchmarks/.out"

    items: list[BenchmarkJsonData] = []

    for file_name in listdir(dir_path):
        file_path = f"{dir_path}/{file_name}"
        data: BenchmarkJsonData = json.load(open(file_path, 'r'))
        items.append(data)

    bars_map: dict[int, list[BenchmarkJsonData]] = {}

    for item in items:
        _item_items: dict[int, list[BenchmarkJsonData]] = {}

        for benchmark in item["benchmarks"]:
            if benchmark["bars"] not in _item_items:
                _item_items[benchmark["bars"]] = []

            _item_items[benchmark["bars"]].append(benchmark)

        for bars, benchmarks in _item_items.items():
            if bars not in bars_map:
                bars_map[bars] = []

            bars_map[bars].append({
                "id": item["id"],
                "benchmarks": benchmarks
            })

    for bars, items in bars_map.items():
        (plt_normal, ratio_normal) = generate_plot(items, bars, False)

        (plt_log10, ratio_log10) = generate_plot(items, bars, True)

        plt_normal.figure.savefig(f"static/benchmarks/{bars}_mean_time.png")
        plt_log10.figure.savefig(
            f"static/benchmarks/{bars}_mean_time_log10.png")

        ratio_normal.figure.savefig(f"static/benchmarks/{bars}_time_diff.png")
