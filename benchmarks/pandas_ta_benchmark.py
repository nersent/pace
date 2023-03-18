import sys
import pandas as pd
import pandas_ta as pta

from benchmarks.common import Benchmark, DataSize, get_df, now, save_benchmarks_to_json


# https://github.com/twopirllc/pandas-ta
class PandasTABenchmarkRunner:
    @staticmethod
    def run(count: int, df: pd.DataFrame, use_talib=False) -> list[Benchmark]:
        bars = len(df)

        print(f"\nRunning benchmarks for {bars} bars | talib: {use_talib}")

        benchmarks: list[Benchmark] = []

        def _sma_14():
            start_time = now()
            pta.sma(df["close"], length=14, talib=use_talib)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("sma_14", count, _sma_14))
        benchmarks[-1].print()

        def _ema_14():
            start_time = now()
            pta.ema(df["close"], length=14, talib=use_talib)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("ema_14", count, _ema_14))
        benchmarks[-1].print()

        def _rsi_14():
            start_time = now()
            pta.rsi(df["close"], length=14, talib=use_talib)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("rsi_14", count, _rsi_14))
        benchmarks[-1].print()

        def _stoch_14():
            start_time = now()
            pta.stoch(df["high"], df["low"], df["close"],
                      length=14, talib=use_talib)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("stoch_14", count, _stoch_14))
        benchmarks[-1].print()

        def _atr_14():
            start_time = now()
            pta.atr(df["high"], df["low"], df["close"],
                    length=14, talib=use_talib)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("atr_14", count, _atr_14))
        benchmarks[-1].print()

        def _macd_12_26():
            start_time = now()
            pta.macd(df["close"], fast=12, slow=26, talib=use_talib)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26", count, _macd_12_26))
        benchmarks[-1].print()

        def _macd_12_26_rsi_14():
            start_time = now()
            pta.macd(df["close"], fast=12, slow=26, talib=use_talib)
            pta.rsi(df["close"], length=14, talib=use_talib)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26_rsi_14", count / 10, _macd_12_26_rsi_14))
        benchmarks[-1].print()

        def macd_12_26_rsi_14_aroon_14():
            start_time = now()
            pta.macd(df["close"], fast=12, slow=26, talib=use_talib)
            pta.rsi(df["close"], length=14, talib=use_talib)
            pta.aroon(df["high"], df["low"], length=14, talib=use_talib)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26_rsi_14_aroon_14", count / 10, macd_12_26_rsi_14_aroon_14))
        benchmarks[-1].print()

        def _dmi_14():
            start_time = now()
            pta.dm(df["high"], df["low"], length=14, talib=use_talib)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("dmi_14", count / 10, _dmi_14))
        benchmarks[-1].print()

        def _aroon_14():
            start_time = now()
            pta.aroon(df["high"], df["low"], length=14, talib=use_talib)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("aroon_14", count / 10, _aroon_14))
        benchmarks[-1].print()

        def _coppock_curve_length_10_long_14_short_11():
            start_time = now()
            pta.coppock(df["close"], length=10, fast=11,
                        slow=10, talib=use_talib)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("coppock_curve_length_10_long_14_short_11",
                          count / 10, _coppock_curve_length_10_long_14_short_11))
        benchmarks[-1].print()

        for benchmark in benchmarks:
            benchmark.bars = bars

        return benchmarks


if __name__ == "__main__":
    small_df = get_df(DataSize.SMALL)
    large_df = get_df(DataSize.LARGE)

    benchmarks_small = PandasTABenchmarkRunner.run(100, small_df, False)
    benchmarks_large = PandasTABenchmarkRunner.run(100, large_df, False)

    benchmarks_small_with_talib = PandasTABenchmarkRunner.run(
        100, small_df, True)
    benchmarks_large_with_talib = PandasTABenchmarkRunner.run(
        100, large_df, True)

    save_benchmarks_to_json(
        "pandas_ta", [
            *benchmarks_small,
            *benchmarks_large,
        ], "pandas_ta.json")

    save_benchmarks_to_json(
        "pandas_ta_talib", [
            *benchmarks_small_with_talib,
            *benchmarks_large_with_talib,
        ], "pandas_ta_talib.json")

    print("Done")
