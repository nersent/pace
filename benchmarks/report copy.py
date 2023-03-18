import json
from os import listdir
import sys
from typing import Optional, Tuple
from matplotlib import patches
import matplotlib.pyplot as plt
from matplotlib.ticker import FormatStrFormatter, MultipleLocator
import numpy as np

from benchmarks.common import BenchmarkJsonData


def generate_plot(items: list[BenchmarkJsonData], bars_count: int, log_scale: bool = True) -> Tuple[plt.Axes, plt.Axes]:
    lib_color_dict: dict[str, str] = {
        "pace": "red",
        "talib": "green",
        "pandas_ta": "blue",
        "pandas_ta_talib": "cyan",
        "ta": "yellow",
        "finta": "magenta",
        "talipp": "black",
        "spectre_cpu": "orange",
        "spectre_gpu": "purple"
    }

    lib_name_dict: dict[str, str] = {
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

    benchmark_name_dict: dict[str, str] = {
        "sma_14": "SMA(14)",
        "ema_14": "EMA(14)",
        "rsi_14": "RSI(14)",
        "stoch_14": "STOCH(14)",
        "macd_12_26": "MACD(12, 26)",
        "macd_12_26_rsi_14": "MACD(12, 26) + RSI(14)",
        # "macd_12_26_rsi_14_aroon_14": "MACD(12, 26) + RSI(14) + AROON(14)",
        # "dmi_14": "DMI(14, 14)",
        # "aroon_14": "AROON(14)",
        # "coppock_curve_length_10_long_14_short_11": "COPPOCK(10, 11, 14)",
    }

    lib_ids: list[str] = []

    for id in lib_color_dict:
        if id in [item["id"] for item in items]:
            if id not in lib_ids:
                lib_ids.append(id)

    benchmark_dict: dict[str, list[float]] = {}

    for item in items:
        id_index = lib_ids.index(item["id"])

        for benchmark in item["benchmarks"]:
            if benchmark["id"] not in benchmark_name_dict:
                continue

            if benchmark["id"] not in benchmark_dict:
                benchmark_dict[benchmark["id"]] = [0.0] * len(lib_ids)

            benchmark_dict[benchmark["id"]][id_index] = float(
                benchmark["mean"])

    benchmark_names = list(benchmark_dict.keys())
    benchmark_names = [benchmark_name_dict[benchmark]
                       for benchmark in benchmark_names]

    mean_times = list(benchmark_dict.values())
    mean_times_raw = mean_times.copy()

    if log_scale:
        mean_times = [[np.log10(time + 1) for time in times]
                      for times in mean_times]

    max_time_diff = max([max(times) for times in mean_times])

    # Create bar plot
    bar_width = 0.1
    index = np.arange(len(benchmark_names))
    fig, ax = plt.subplots()

    for i, (lib_id, times) in enumerate(zip(lib_ids, zip(*mean_times))):
        lib_name = lib_name_dict[lib_id]
        lib_color = lib_color_dict[lib_id]
        ax.bar(
            index + i * bar_width,
            times,
            bar_width,
            label=lib_name,
            color=lib_color,
        )

    ax.set_xticks(index + bar_width * (len(lib_ids) - 1) / 2)
    ax.set_xticklabels(benchmark_names)

    _increment = 0.25
    if max_time_diff > 1000:
        _increment = 250

    ax.yaxis.set_major_locator(MultipleLocator(_increment))

    _width = 30
    _ratio = 21 / 9
    fig.set_size_inches(_width, int(_width * 1 / _ratio))

    if log_scale:
        ax.set_ylabel('log10 Mean Time (ms)')
    else:
        ax.set_ylabel('Mean Time (ms)')

    ax.set_title(f"Mean time for {bars_count} bars")
    ax.legend()

    pace_id = lib_ids.index("pace")

    fig_ratio, ax_ratio = plt.subplots()

    included_lib_ids = ["talib", "pandas_ta_talib"]

    max_time_diff = 0

    for i, (lib_id, times) in enumerate(zip(lib_ids, zip(*mean_times_raw))):
        if lib_id not in included_lib_ids:
            continue
        lib_name = lib_name_dict[lib_id]
        lib_color = lib_color_dict[lib_id]

        pace_times = mean_times_raw[pace_id]

        ratios = [((time - pace_time)) if time !=
                  0 else 0 for pace_time, time in zip(pace_times, times)]

        # print(ratios)
        # ratios = [6, 9]
        # ratios: list[float] = []

        # for j, mean_time in enumerate(times):
        #     lib_id = lib_ids[j]
        #     if lib_id not in included_lib_ids:
        #         ratios.append(0)
        #         continue

        #     print("XD", lib_id, mean_time)

        #     mean_time_diff = mean_time - pace_times[j]
        #     ratios.append(mean_time_diff)
        max_time_diff = max(max_time_diff, max(ratios))

        ax_ratio.bar(
            index + i * bar_width,
            ratios,
            bar_width,
            label=lib_name,
            color=lib_color,
        )

    ax_ratio.set_xticks(index + bar_width * (len(included_lib_ids) + 1) / 2)
    ax_ratio.set_xticklabels(benchmark_names)

    _increment = 5
    if max_time_diff < 10:
        _increment = 0.1
    ax_ratio.yaxis.set_major_locator(MultipleLocator(_increment))

    ax_ratio.axhline(y=0, color='red', linestyle='--')

    fig_ratio.set_size_inches(_width, int(_width * 1 / _ratio))

    ax_ratio.set_ylabel('Time difference (ms)')
    ax_ratio.set_title(f"Time difference for {bars_count} bars")
    ax_ratio.legend()

    return ax, ax_ratio


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

        # ratio_log10.figure.savefig(f"static/benchmarks/{bars}_ratio_log10.png")
