import pandas as pd
from talipp.indicators import EMA, SMA, Stoch, RSI, ATR, MACD, Aroon, ADX
import numpy as np
from talipp.ohlcv import OHLCVFactory

from benchmarks.common import Benchmark, DataSize, get_df, now, save_benchmarks_to_json


# https://github.com/nardew/talipp
class TaLippBenchmarkRunner:
    @staticmethod
    def run(count: int, df: pd.DataFrame) -> list[Benchmark]:
        bars = len(df)

        print(f"\nRunning benchmarks for {bars} bars")

        benchmarks: list[Benchmark] = []

        _open = df["open"].to_numpy()
        _high = df["high"].to_numpy()
        _low = df["low"].to_numpy()
        _close = df["close"].to_numpy()
        _volume = df["volume"].to_numpy()

        _ohlcv = OHLCVFactory.from_dict({
            'high': _high,
            'low': _low,
            'close': _close,
            'open': _open,
            'volume': _volume,
        })

        def _sma_14():
            start_time = now()
            SMA(input_values=_close, period=14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("sma_14", count, _sma_14))
        benchmarks[-1].print()

        def _ema_14():
            start_time = now()
            EMA(input_values=_close, period=14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("ema_14", count, _ema_14))
        benchmarks[-1].print()

        def _rsi_14():
            start_time = now()
            RSI(input_values=_close, period=14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("rsi_14", count, _rsi_14))
        benchmarks[-1].print()

        def _stoch_14():
            start_time = now()
            Stoch(input_values=_ohlcv, period=14, smoothing_period=3)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("stoch_14", count, _stoch_14))
        benchmarks[-1].print()

        def _atr_14():
            start_time = now()
            ATR(input_values=_ohlcv, period=14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("atr_14", count, _atr_14))
        benchmarks[-1].print()

        def _macd_12_26():
            start_time = now()
            MACD(input_values=_close, fast_period=12,
                 slow_period=26, signal_period=9)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26", count, _macd_12_26))
        benchmarks[-1].print()

        def _macd_12_26_rsi_14():
            start_time = now()

            MACD(input_values=_close,  fast_period=12,
                 slow_period=26, signal_period=9)
            RSI(input_values=_close, period=14)

            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26_rsi_14", count, _macd_12_26_rsi_14))
        benchmarks[-1].print()

        def macd_12_26_rsi_14_aroon_14():
            start_time = now()

            MACD(input_values=_close,  fast_period=12,
                 slow_period=26, signal_period=9)
            RSI(input_values=_close, period=14)
            Aroon(input_values=_ohlcv, period=14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run(
            "macd_12_26_rsi_14_aroon_14", count, macd_12_26_rsi_14_aroon_14))
        benchmarks[-1].print()

        def _dmi_14():
            start_time = now()
            ADX(input_values=_ohlcv, period_di=14, period_adx=14)
            end_time = now()
            return start_time, end_time

        benchmarks.append(Benchmark.run("dmi_14", count, _dmi_14))
        benchmarks[-1].print()

        def _aroon_14():
            start_time = now()
            Aroon(input_values=_ohlcv, period=14)
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

    benchmarks_small = TaLippBenchmarkRunner.run(1, small_df)
    benchmarks_large = TaLippBenchmarkRunner.run(1, large_df)

    save_benchmarks_to_json("talipp", [
        *benchmarks_small,
        *benchmarks_large
    ], "talipp.json")

    print("Done")
