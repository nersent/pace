import pandas as pd
import numpy as np

if __name__ == "__main__":
    count = 1_000_00

    df = pd.DataFrame(
        columns=['time', 'open', 'high', 'low', 'close', 'volume'])

    time_start = 1313625600
    time_increment = 1000

    open_start = 0.01
    open_increment = 0.01

    high_start = 0.01
    high_increment = 0.01

    low_start = 0.01
    low_increment = 0.01

    close_start = 0.01
    close_increment = 0.01

    volume_start = 0.01
    volume_increment = 0.01

    # use numpy instead
    df['time'] = np.arange(time_start, time_start +
                           count * time_increment, time_increment)
    df['open'] = np.arange(open_start, open_start +
                           count * open_increment, open_increment)
    df['high'] = np.arange(high_start, high_start +
                           count * high_increment, high_increment)
    df['low'] = np.arange(low_start, low_start + count *
                          low_increment, low_increment)
    df['close'] = np.arange(close_start, close_start +
                            count * close_increment, close_increment)
    df['volume'] = np.arange(
        volume_start, volume_start + count * volume_increment, volume_increment)

    # df.to_csv('data.csv', index=False)
    df.to_parquet('data.parquet', index=False)
