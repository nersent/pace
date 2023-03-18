import sys
import pandas as pd
from ta.trend import SMAIndicator, EMAIndicator
from ta.momentum import RSIIndicator, StochasticOscillator
from ta.trend import MACD, AroonIndicator, ADXIndicator
from ta.volatility import AverageTrueRange
import numpy as np

from benchmarks.common import Benchmark, DataSize, get_df, now, save_benchmarks_to_json


# https://github.com/bukosabino/ta
class TaBenchmarkRunner:
    @staticmethod
    def run(count: int, df: pd.DataFrame) -> list[Benchmark]:
        bars = len(df)

        print(f"\nRunning benchmarks for {bars} bars")

        benchmarks: list[Benchmark] = []

        def _sma_14():
            start_time = now()
            SMAIndicator(df["close"], 14).sma_indicator()
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("sma_14", count, _sma_14))
        benchmarks[-1].print()

        def _ema_14():
            start_time = now()
            EMAIndicator(df["close"], 14).ema_indicator()
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("ema_14", count, _ema_14))
        benchmarks[-1].print()

        def _rsi_14():
            start_time = now()
            RSIIndicator(df["close"], 14).rsi()
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("rsi_14", count, _rsi_14))
        benchmarks[-1].print()

        def _stoch_14():
            start_time = now()
            StochasticOscillator(df["high"], df["low"],
                                 df["close"], 14).stoch()
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("stoch_14", count, _stoch_14))
        benchmarks[-1].print()

        def _atr_14():
            start_time = now()
            AverageTrueRange(df["high"], df["low"],
                             df["close"], 14).average_true_range()
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("atr_14", count / 100, _atr_14))
        benchmarks[-1].print()

        def _macd_12_26():
            start_time = now()

            macd = MACD(df["close"], window_slow=12,
                        window_fast=26, window_sign=9)
            macd.macd()
            macd.macd_signal()

            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26", count / 10, _macd_12_26))
        benchmarks[-1].print()

        def _macd_12_26_rsi_14():
            start_time = now()

            macd = MACD(df["close"], window_slow=12,
                        window_fast=26, window_sign=9)
            macd.macd()
            macd.macd_signal()

            RSIIndicator(df["close"], 14).rsi()

            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26_rsi_14", count / 10, _macd_12_26_rsi_14))
        benchmarks[-1].print()

        def macd_12_26_rsi_14_aroon_14():
            start_time = now()

            macd = MACD(df["close"], window_slow=12,
                        window_fast=26, window_sign=9)
            macd.macd()
            macd.macd_signal()

            RSIIndicator(df["close"], 14).rsi()

            aroon = AroonIndicator(df["close"], 14)
            aroon.aroon_up()
            aroon.aroon_down()
            aroon.aroon_indicator()

            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26_rsi_14_aroon_14", count / 100, macd_12_26_rsi_14_aroon_14))
        benchmarks[-1].print()

        def _dmi_14():
            start_time = now()

            dmi = ADXIndicator(df["high"], df["low"], df["close"], 14)
            dmi.adx()
            dmi.adx_neg()
            dmi.adx_pos()
            end_time = now()

            return start_time, end_time

        benchmarks.append(Benchmark.run("dmi_14", count / 100, _dmi_14))
        benchmarks[-1].print()

        def _aroon_14():
            start_time = now()

            aroon = AroonIndicator(df["close"], 14)
            aroon.aroon_up()
            aroon.aroon_down()
            aroon.aroon_indicator()

            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("aroon_14", count / 100, _aroon_14))
        benchmarks[-1].print()

        for benchmark in benchmarks:
            benchmark.bars = bars

        return benchmarks


if __name__ == "__main__":
    small_df = get_df(DataSize.SMALL)
    large_df = get_df(DataSize.LARGE)

    benchmarks_small = TaBenchmarkRunner.run(1000, small_df)
    benchmarks_large = TaBenchmarkRunner.run(1000, large_df)

    save_benchmarks_to_json("ta", [
        *benchmarks_small,
        *benchmarks_large
    ], "ta.json")

    print("Done")
