from typing import Optional, Tuple, Union
from openpyxl import load_workbook
from openpyxl.workbook import Workbook
from openpyxl.worksheet.worksheet import Worksheet
import os
from pycel import ExcelCompiler
from api.src.worksheet_utils import format_coordinate, get_cell_coordinate_below, parse_coordinate
from nersent_pace_py import nersent_pace_py as pace


def format_config_column_id(column_id: str) -> str:
    return f"<nersent_pace::config::{column_id}>"


def format_data_column_id(column_id: str) -> str:
    return f"<nersent_pace::data::{column_id}>"


def format_input_column_id(column_id: str) -> str:
    return f"<nersent_pace::input::{column_id}>"


def format_output_column_id(column_id: str) -> str:
    return f"<nersent_pace::output::{column_id}>"


class ExcelBacktester():
    def __init__(self):
        self.worksheet_name: Optional[str] = None
        self.path: Optional[str] = None
        self.workbook: Optional[Workbook] = None
        self.worksheet: Optional[Worksheet] = None
        self.compiled_worksheet: Optional[ExcelCompiler] = None
        self.is_loaded: bool = False

        config_columns = [
            "on_bar_close",
            "initial_capital",
            "buy_with_equity",
            "risk_free_rate"
        ]

        data_columns = [
            "time",
            "open",
            "high",
            "low",
            "close",
            "volume"
        ]

        input_columns = [
            "strategy_signal"
        ]

        output_columns = [
            "time",
            "tick",
            "equity",
            "net_equity",
            "open_profit",
            "position_size",
            "returns",
            "direction",
            "logs",
            "pinescript"
        ]

        self.config_columns = list(
            map(format_config_column_id, config_columns))
        self.data_columns = list(map(format_data_column_id, data_columns))
        self.input_columns = list(
            map(format_input_column_id, input_columns))
        self.output_columns = list(
            map(format_output_column_id, output_columns))

        self.columns = [
            *self.config_columns,
            *self.data_columns,
            *self.input_columns,
            *self.output_columns
        ]

    def load(self, path: str, worksheet_name: str):
        self.is_loaded = False

        self.worksheet_name = worksheet_name
        self.path = path
        xlxs_path = os.path.abspath(path)
        self.workbook = load_workbook(filename=path)
        self.worksheet = self.workbook[worksheet_name]
        self.compiled_worksheet = ExcelCompiler(filename=xlxs_path)

        self.is_loaded = True

    def compute(self) -> dict[str, Union[str, int, float]]:
        if not self.is_loaded:
            raise Exception("ExcelBacktester is not loaded")

        column_coordinate_map: dict[str, Tuple[str, int]] = {}

        for row in self.worksheet.iter_rows():
            for cell in row:
                for pace_column_id in self.columns:
                    if cell is None or cell.value is None:
                        continue
                    value = str(cell.value).lower()
                    if pace_column_id in value:
                        coordinate = cell.coordinate
                        parsed_coordinate = parse_coordinate(coordinate)
                        column_coordinate_map[pace_column_id] = parsed_coordinate

        assert_columns = [*self.data_columns, *self.input_columns]

        for column_id in assert_columns:
            if column_id not in column_coordinate_map:
                raise Exception(
                    f"Could not find data column {column_id} in worksheet")

        column_to_length_map: dict[str, int] = {}

        for column_id in self.data_columns:
            (column, row) = column_coordinate_map[column_id]

            for i in range(row, self.worksheet.max_row + 1):
                cell = self.worksheet[column + str(i)]
                if cell is None or cell.value is None:
                    print(f"[{column_id}]: {i}")
                    break
                column_to_length_map[column_id] = i

        data_length: Optional[int] = None

        for column_id in self.data_columns:
            column_length = column_to_length_map[column_id]
            if data_length is None:
                data_length = column_length
                continue
            if column_length < data_length:
                data_length = column_length

        data_length = data_length - 1
        data: dict[str, list[float]] = {}

        for column_id in self.data_columns:
            (column, row) = column_coordinate_map[column_id]
            data[column_id] = []

            for i in range(row + 1, data_length + 2):
                cell = self.worksheet[column + str(i)]
                if cell is None or cell.value is None:
                    break
                data[column_id].append(int(cell.value))

        # print(column_coordinate_map)
        # print(data_length)

        for column_id in data:
            data[column_id] = data[column_id][:data_length]
            # if len(data[column_id]) != data_length:
            #     raise Exception(
            #         f"Data column {column_id} does not have the same length as other data columns: {len(data[column_id])} vs {data_length} | {data[column_id][0]} | {data[column_id][-1]}")

        (signal_column, signal_row) = column_coordinate_map[format_input_column_id(
            "strategy_signal")]
        signal_row_start = signal_row + 1
        signal_row_end = signal_row + data_length

        evaluation_target = f"{self.worksheet_name}!{signal_column}{signal_row_start}:{signal_column}{signal_row_end}"

        evaluated_signals = self.compiled_worksheet.evaluate(evaluation_target)

        if len(evaluated_signals) != data_length:
            raise Exception(
                f"Evaluated signals length does not match data length: {len(evaluated_signals)} vs {data_length}")

        signals: list[pace.StrategySignal] = []

        for i in range(len(evaluated_signals)):
            signal: str = evaluated_signals[i]
            pace_signal_id = "hold"

            if signal is not None:
                signal = signal.lower().strip()
                if signal == "long":
                    pace_signal_id = "long"
                elif signal == "long_entry" or signal == "long entry":
                    pace_signal_id = "long_entry"
                elif signal == "long_exit" or signal == "long exit":
                    pace_signal_id = "long_exit"
                elif signal == "short":
                    pace_signal_id = "short"
                elif signal == "short_entry" or signal == "short entry":
                    pace_signal_id = "short_entry"
                elif signal == "short_exit" or signal == "short exit":
                    pace_signal_id = "short_exit"

            signal = pace.StrategySignal({"id": pace_signal_id})
            signals.append(signal)

        assert len(signals) == data_length

        data_provider = pace.DataProvider({
            "time": data[format_data_column_id("time")],
            "open": data[format_data_column_id("open")],
            "close": data[format_data_column_id("close")],
            "high": data[format_data_column_id("high")],
            "low": data[format_data_column_id("low")],
            "volume": data[format_data_column_id("volume")],
        })

        on_bar_close_column = format_config_column_id("on_bar_close")
        initial_capital_column = format_config_column_id("initial_capital")
        buy_with_equity_column = format_config_column_id("buy_with_equity")
        risk_free_rate_column = format_config_column_id("risk_free_rate")

        backtest_config = {
            "on_bar_close": False,
            "initial_capital": 1000.0,
            "buy_with_equity": True,
            "risk_free_rate": 0.0,
        }

        if on_bar_close_column in column_coordinate_map:
            coordinate = format_coordinate(get_cell_coordinate_below(
                column_coordinate_map[on_bar_close_column]))
            backtest_config["on_bar_close"] = int(
                self.worksheet[coordinate].value) == 1

        if initial_capital_column in column_coordinate_map:
            coordinate = format_coordinate(get_cell_coordinate_below(
                column_coordinate_map[initial_capital_column]))
            backtest_config["initial_capital"] = float(
                self.worksheet[coordinate].value)

        if buy_with_equity_column in column_coordinate_map:
            coordinate = format_coordinate(get_cell_coordinate_below(
                column_coordinate_map[buy_with_equity_column]))
            backtest_config["buy_with_equity"] = int(
                self.worksheet[coordinate].value) == 1

        if risk_free_rate_column in column_coordinate_map:
            coordinate = format_coordinate(get_cell_coordinate_below(
                column_coordinate_map[risk_free_rate_column]))
            backtest_config["risk_free_rate"] = float(
                self.worksheet[coordinate].value)

        print(backtest_config)

        backtest_res = pace.run_backtest(
            data_provider, backtest_config, signals)

        column_update_map: dict[str, Union[str, int, float]] = {}

        for (i, bar) in enumerate(backtest_res.bars):
            for id in self.output_columns:
                if id not in column_coordinate_map:
                    continue

                (column, start_row) = column_coordinate_map[id]
                target_coordinate = f"{column}{start_row + i + 1}"

                if id == format_output_column_id("tick"):
                    column_update_map[target_coordinate] = bar.tick
                elif id == format_output_column_id("time"):
                    column_update_map[target_coordinate] = bar.time
                elif id == format_output_column_id("equity"):
                    column_update_map[target_coordinate] = bar.equity
                elif id == format_output_column_id("net_equity"):
                    column_update_map[target_coordinate] = bar.net_equity
                elif id == format_output_column_id("open_profit"):
                    column_update_map[target_coordinate] = bar.open_profit
                elif id == format_output_column_id("position_size"):
                    column_update_map[target_coordinate] = bar.position_size
                elif id == format_output_column_id("returns"):
                    column_update_map[target_coordinate] = bar.returns
                elif id == format_output_column_id("direction"):
                    column_update_map[target_coordinate] = bar.direction
                elif id == format_output_column_id("logs"):
                    column_update_map[target_coordinate] = bar.logs

        pinescript_column = format_output_column_id("pinescript")

        if pinescript_column in column_coordinate_map:
            target_coordinate = get_cell_coordinate_below(
                column_coordinate_map[pinescript_column])
            target_coordinate = format_coordinate(target_coordinate)
            column_update_map[target_coordinate] = backtest_res.pinescript

        return column_update_map
