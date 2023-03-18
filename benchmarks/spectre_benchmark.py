import sys
from typing import Any, Callable
import pandas as pd
from spectre import factors
from spectre.data import ArrowLoader, CsvDirLoader
import numpy as np
from spectre.data import YahooDownloader

from benchmarks.common import Benchmark, DataSize, get_df, now, save_benchmarks_to_json


# https://github.com/Heerozh/spectre
class SpectreBenchmarkRunner:
    @staticmethod
    def run(count: int, df: pd.DataFrame, use_gpu: bool) -> list[Benchmark]:
        bars = len(df)

        print(f"\nRunning benchmarks for {bars} bars")

        benchmarks: list[Benchmark] = []

        df["Dividends"] = 0
        df["Open"] = df["open"].astype(np.float32)
        df["High"] = df["high"].astype(np.float32)
        df["Low"] = df["low"].astype(np.float32)
        df["Close"] = df["close"].astype(np.float32)
        df["Volume"] = df["volume"].astype(np.float64)
        df['Date'] = pd.to_datetime(
            df['time'], unit='s')
        _first_date = df['Date'].iloc[0]
        _last_date = df['Date'].iloc[-1]
        df.to_csv("benchmarks/.data/xd/xd.csv")

        loader = CsvDirLoader(
            "benchmarks/.data/xd",
            ohlcv=('Open', 'High', 'Low', 'Close', 'Volume'),
            prices_index='Date',
            parse_dates=True,
            dtype={'Open': np.float32, 'High': np.float32, 'Low': np.float32,
                   'Close': np.float32,
                   'Volume': np.float64, 'Dividends': np.float64})

        def _run(cb):
            start = _first_date
            end = _last_date

            engine = factors.FactorEngine(loader)

            if use_gpu:
                engine.to_cuda()
            else:
                engine.to_cpu()

            engine.remove_all_factors()
            cb(engine)

            engine.run(start, end)

            start_time = now()
            res = engine.run(start, end)
            end_time = now()

            return start_time, end_time

        def _sma_14():
            return _run(lambda engine: engine.add(factors.SMA(14), 'sma_14'))

        benchmarks.append(Benchmark.run("sma_14", count, _sma_14))
        benchmarks[-1].print()

        def _ema_14():
            return _run(lambda engine: engine.add(factors.EMA(14), 'ema_14'))

        benchmarks.append(Benchmark.run("ema_14", count, _ema_14))
        benchmarks[-1].print()

        def _rsi_14():
            return _run(lambda engine: engine.add(factors.RSI(14), 'rsi_14'))

        benchmarks.append(Benchmark.run("rsi_14", count, _rsi_14))
        benchmarks[-1].print()

        # def _stoch_14():
        #     start_time = now()
        #     Stoch(input_values=_ohlcv, period=14, smoothing_period=3)
        #     end_time = now()
        #     return start_time, end_time

        # benchmarks.append(Benchmark.run("stoch_14", count, _stoch_14))
        # benchmarks[-1].print()

        def _atr_14():
            return _run(lambda engine: engine.add(factors.MA(14, inputs=(factors.TrueRange(),)), 'atr_14'))

        benchmarks.append(Benchmark.run("atr_14", count, _atr_14))
        benchmarks[-1].print()

        def _macd_12_26():
            return _run(lambda engine: engine.add(factors.MACD(12, 26, 9), 'macd_12_26_9'))

        benchmarks.append(Benchmark.run(
            "macd_12_26", count, _macd_12_26))
        benchmarks[-1].print()

        def _macd_12_26_rsi_14():
            def _init(engine):
                engine.add(factors.MACD(12, 26, 9), 'macd_12_26_9')
                engine.add(factors.RSI(14), 'rsi_14')
            return _run(_init)

        benchmarks.append(Benchmark.run(
            "macd_12_26_rsi_14", count, _macd_12_26_rsi_14))
        benchmarks[-1].print()

        # def macd_12_26_rsi_14_aroon_14():
        #     start_time = now()

        #     MACD(input_values=_close,  fast_period=12,
        #          slow_period=26, signal_period=9)
        #     RSI(input_values=_close, period=14)
        #     Aroon(input_values=_ohlcv, period=14)
        #     end_time = now()
        #     return start_time, end_time

        # benchmarks.append(Benchmark.run(
        #     "macd_12_26_rsi_14_aroon_14", count, macd_12_26_rsi_14_aroon_14))
        # benchmarks[-1].print()

        # def _dmi_14():
        #     start_time = now()
        #     ADX(input_values=_ohlcv, period_di=14, period_adx=14)
        #     end_time = now()
        #     return start_time, end_time

        # benchmarks.append(Benchmark.run("dmi_14", count, _dmi_14))
        # benchmarks[-1].print()

        # def _aroon_14():
        #     start_time = now()
        #     Aroon(input_values=_ohlcv, period=14)
        #     end_time = now()
        #     return start_time, end_time

        # benchmarks.append(Benchmark.run("aroon_14", count, _aroon_14))
        # benchmarks[-1].print()

        for benchmark in benchmarks:
            benchmark.bars = bars

        return benchmarks


if __name__ == "__main__":
    small_df = get_df(DataSize.SMALL)
    large_df = get_df(DataSize.LARGE)

    benchmarks_small_cpu = SpectreBenchmarkRunner.run(100, small_df, False)
    benchmarks_large_cpu = SpectreBenchmarkRunner.run(2, large_df, False)

    benchmarks_small_gpu = SpectreBenchmarkRunner.run(100, small_df, True)
    benchmarks_large_gpu = SpectreBenchmarkRunner.run(2, large_df, True)

    save_benchmarks_to_json("spectre_cpu", [
        *benchmarks_small_cpu,
        *benchmarks_large_cpu
    ], "spectre_cpu.json")

    save_benchmarks_to_json("spectre_gpu", [
        *benchmarks_small_gpu,
        *benchmarks_large_gpu
    ], "spectre_gpu.json")

    print("Done")
