import pandas as pd
import talib as ta
import numpy as np

from benchmarks.common import Benchmark, DataSize, get_df, now, save_benchmarks_to_json


# https://github.com/TA-Lib/ta-lib-python
class TaLibBenchmarkRunner:
    @staticmethod
    def run(count: int, df: pd.DataFrame) -> list[Benchmark]:
        bars = len(df)

        print(f"\nRunning benchmarks for {bars} bars")

        benchmarks: list[Benchmark] = []

        _open = df["open"].to_numpy()
        _high = df["high"].to_numpy()
        _low = df["low"].to_numpy()
        _close: np.ndarray = df["close"].to_numpy()
        _volume = df["volume"].to_numpy()

        def _sma_14():
            start_time = now()
            ta.SMA(_close, 14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("sma_14", count, _sma_14))
        benchmarks[-1].print()

        def _ema_14():
            start_time = now()
            ta.EMA(_close, 14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("ema_14", count, _ema_14))
        benchmarks[-1].print()

        def _rsi_14():
            start_time = now()
            ta.RSI(_close, 14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("rsi_14", count, _rsi_14))
        benchmarks[-1].print()

        def _stoch_14():
            start_time = now()
            ta.STOCH(_high, _low, _close, 14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("stoch_14", count, _stoch_14))
        benchmarks[-1].print()

        def _atr_14():
            start_time = now()
            ta.ATR(_high, _low, _close, 14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("atr_14", count, _atr_14))
        benchmarks[-1].print()

        def _macd_12_26():
            start_time = now()
            ta.MACD(_close, fastperiod=12, slowperiod=26, signalperiod=9)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26", count, _macd_12_26))
        benchmarks[-1].print()

        def _macd_12_26_rsi_14():
            start_time = now()
            ta.MACD(_close, fastperiod=12, slowperiod=26, signalperiod=9)
            ta.RSI(_close, 14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26_rsi_14", count, _macd_12_26_rsi_14))
        benchmarks[-1].print()

        def macd_12_26_rsi_14_aroon_14():
            start_time = now()
            ta.MACD(_close, fastperiod=12, slowperiod=26, signalperiod=9)
            ta.RSI(_close, 14)
            ta.AROON(_high, _low, 14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26_rsi_14_aroon_14", count, macd_12_26_rsi_14_aroon_14))
        benchmarks[-1].print()

        def _dmi_14():
            start_time = now()
            ta.DX(_high, _low, _close, 14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("dmi_14", count, _dmi_14))
        benchmarks[-1].print()

        def _aroon_14():
            start_time = now()
            ta.AROON(_high, _low, 14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("aroon_14", count, _aroon_14))
        benchmarks[-1].print()

        for benchmark in benchmarks:
            benchmark.bars = bars

        return benchmarks


if __name__ == "__main__":
    small_df = get_df(DataSize.SMALL)
    large_df = get_df(DataSize.LARGE)

    benchmarks_small = TaLibBenchmarkRunner.run(1000, small_df)
    benchmarks_large = TaLibBenchmarkRunner.run(1000, large_df)

    save_benchmarks_to_json("talib", [
        *benchmarks_small,
        *benchmarks_large
    ], "talib.json")

    print("Done")
