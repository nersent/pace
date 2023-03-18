from datetime import datetime
from enum import Enum
import json
import time
from typing import Any, Callable, Optional, Tuple
import numpy as np
import pandas as pd
from tqdm import tqdm


def now():
    return time.perf_counter()


class Benchmark:
    id: str
    time_list: list[float]
    bars: int
    options: dict[str, str]

    def __init__(self, id: str) -> None:
        self.id = id
        self.time_list = []
        self.bars = 0
        self.options = {}

    @staticmethod
    def run(id: str, n: int, cb: Callable[[], Tuple[float, float]]) -> 'Benchmark':
        instance = Benchmark(id)
        for _ in tqdm(range(int(n))):
            start_time, end_time = cb()
            time_s = end_time - start_time
            time_ms = time_s * 1000.0
            instance.time_list.append(time_ms)
        return instance

    def mean(self) -> float:
        return np.mean(self.time_list, axis=0)

    def stdev(self) -> float:
        return np.std(self.time_list, axis=0)

    def print(self) -> None:
        print(f"\n[{self.id}]: Mean={self.mean()}ms | Stdev={self.stdev()}ms\n")


class BenchmarkJsonData:
    id: str
    benchmarks: list['BenchmarkJsonEntryData']


class BenchmarkJsonEntryData:
    id: str
    runs: int
    bars: int
    mean: float
    stdev: float
    options: Optional[dict[str, Any]]


def save_benchmarks_to_json(id: str, benchmarks: list[Benchmark], filename: str) -> None:
    data = {
        "id": id,
        "benchmarks": [
            {
                "id": benchmark.id,
                "runs": len(benchmark.time_list),
                "bars": benchmark.bars,
                "mean": benchmark.mean(),
                "stdev": benchmark.stdev(),
                "options": benchmark.options,
            } for benchmark in benchmarks
        ]
    }

    path = f"benchmarks/.out/{filename}"

    with open(path, 'w') as f:
        json.dump(data, f, indent=2)


class DataSize(Enum):
    SMALL = 1
    LARGE = 2


def get_df(size: DataSize) -> pd.DataFrame:
    file_name: str = ""

    if size == DataSize.SMALL:
        file_name = "small.parquet"
    elif size == DataSize.LARGE:
        file_name = "large.parquet"
    else:
        raise ValueError("Invalid DataSize")

    base_path = "benchmarks/.data"
    df = pd.read_parquet(f"{base_path}/{file_name}")

    return df
