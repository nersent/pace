import json
import os
from time import time
import gspread
from gspread.utils import ExportFormat
from gspread import Cell

from api.src.excel_backtester import ExcelBacktester


class GoogleSheetBacktester():
    def __init__(self, client: gspread.Client, path: str):
        self.client = client
        self.path = path

    def update(self, url: str, worksheet_name: str):
        xlxs_path = os.path.abspath(
            os.path.join(self.path, f"generated.xlsx"))

        google_sheet = self.client.open_by_url(url)
        raw_exported_google_sheet = google_sheet.export(ExportFormat.EXCEL)

        with open(xlxs_path, "wb") as f:
            f.write(raw_exported_google_sheet)

        google_worksheet = google_sheet.worksheet(worksheet_name)

        backtester = ExcelBacktester()
        backtester.load(xlxs_path, worksheet_name)
        update_map = backtester.compute()

        # with open(os.path.abspath("local/fixtures/update_map.json"), "w") as f:
        #     f.write(json.dumps(update_map))

        google_worksheet_batch: list[dict] = []

        for coordinate, value in update_map.items():
            google_worksheet_batch.append({
                "range": coordinate,
                "values": [[value]]
            })

        google_worksheet.batch_update(google_worksheet_batch)
