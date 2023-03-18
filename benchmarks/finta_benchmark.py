import sys
import pandas as pd
from finta import TA as fta
import numpy as np

from benchmarks.common import Benchmark, DataSize, get_df, now, save_benchmarks_to_json


# https://github.com/peerchemist/finta
class FinTaBenchmarkRunner:
    @staticmethod
    def run(count: int, df: pd.DataFrame) -> list[Benchmark]:
        bars = len(df)

        print(f"\nRunning benchmarks for {bars} bars")

        benchmarks: list[Benchmark] = []

        def _sma_14():
            start_time = now()
            fta.SMA(df, 14, "close")
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("sma_14", count, _sma_14))
        benchmarks[-1].print()

        def _ema_14():
            start_time = now()
            fta.EMA(df, 14, "close")
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("ema_14", count, _ema_14))
        benchmarks[-1].print()

        def _rsi_14():
            start_time = now()
            fta.RSI(df, 14, "close")
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("rsi_14", count, _rsi_14))
        benchmarks[-1].print()

        def _stoch_14():
            start_time = now()
            fta.STOCH(df, 14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("stoch_14", count, _stoch_14))
        benchmarks[-1].print()

        def _atr_14():
            start_time = now()
            fta.ATR(df, 14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("atr_14", count / 100, _atr_14))
        benchmarks[-1].print()

        def _macd_12_26():
            start_time = now()

            fta.MACD(df, period_fast=12, period_slow=26,
                     signal=9, column="close")

            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26", count, _macd_12_26))
        benchmarks[-1].print()

        def _macd_12_26_rsi_14():
            start_time = now()

            fta.MACD(df, period_fast=12, period_slow=26,
                     signal=9, column="close")
            fta.RSI(df, 14, "close")

            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26_rsi_14", count / 100, _macd_12_26_rsi_14))
        benchmarks[-1].print()

        # def macd_12_26_rsi_14_aroon_14():
        #     start_time = now()

        #     fta.MACD(df, period_fast=12, period_slow=26, signal=9, column="close")
        #     fta.RSI(df, 14, "close")

        #     aroon = AroonIndicator(df["close"], 14)
        #     aroon.aroon_up()
        #     aroon.aroon_down()
        #     aroon.aroon_indicator()

        #     end_time = now()
        #     return start_time, end_time

        # benchmarks.append(Benchmark.run(
        #     "macd_12_26_rsi_14_aroon_14", count, macd_12_26_rsi_14_aroon_14))
        # benchmarks[-1].print()

        def _dmi_14():
            start_time = now()

            fta.DMI(df, 14)
            end_time = now()

            return start_time, end_time

        benchmarks.append(Benchmark.run("dmi_14", count / 100, _dmi_14))
        benchmarks[-1].print()

        # def _aroon_14():
        #     start_time = now()

        #     aroon = AroonIndicator(df["close"], 14)
        #     aroon.aroon_up()
        #     aroon.aroon_down()
        #     aroon.aroon_indicator()

        #     end_time = now()
        #     return start_time, end_time

        # benchmarks.append(Benchmark.run("aroon_14", count, _aroon_14))
        # benchmarks[-1].print()

        for benchmark in benchmarks:
            benchmark.bars = bars

        return benchmarks


def normalize_df(df: pd.DataFrame) -> pd.DataFrame:
    # df["Low"] = df["low"]
    # df["High"] = df["high"]
    # df["Open"] = df["open"]
    # df["Close"] = df["close"]
    # df["Volume"] = df["volume"]
    return df


if __name__ == "__main__":
    small_df = normalize_df(get_df(DataSize.SMALL))
    large_df = normalize_df(get_df(DataSize.LARGE))

    benchmarks_small = FinTaBenchmarkRunner.run(1000, small_df)
    benchmarks_large = FinTaBenchmarkRunner.run(1000, large_df)

    save_benchmarks_to_json("finta", [
        *benchmarks_small,
        *benchmarks_large
    ], "finta.json")

    print("Done")
