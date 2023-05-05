import os
import traceback
from typing import Optional
from flask import Flask, request, jsonify
import gspread
from gspread import Client
from time import time
from pathlib import Path
import shutil
from flask_cors import CORS, cross_origin

from api.src.google_sheet_backtester import GoogleSheetBacktester

app = Flask(__name__)
cors = CORS(app)
app.config['CORS_HEADERS'] = 'Content-Type'


class GspreadClientProvider():
    def __init__(self):
        self._client: Optional[Client] = None

    def get(self) -> Optional[Client]:
        if self._client is None:
            self._client = gspread.service_account(
                filename=os.path.abspath("service_account.json")
            )
        return self._client


@app.route('/api/integration/google_sheets', methods=['GET'])
@cross_origin()
def google_sheets():
    url = request.args.get("url")
    worksheet_name = request.args.get("worksheet")

    print("\nGot new request")
    print(worksheet_name)
    print(url)

    if url is None:
        return jsonify({'message': 'Missing url'}), 400

    if worksheet_name is None:
        return jsonify({'message': 'Missing worksheet'}), 400

    google_client = GspreadClientProvider().get()

    timestamp = str(time())
    working_dir = os.path.abspath(f"local/working_dir/{timestamp}")
    Path(working_dir).mkdir(parents=True, exist_ok=True)

    def _clean():
        shutil.rmtree(working_dir)

    try:
        google_sheet_backtester = GoogleSheetBacktester(
            client=google_client, path=working_dir)

        google_sheet_backtester.update(
            url=url,
            worksheet_name=worksheet_name
        )

        _clean()

        return jsonify({"success": True, "message": "Success"}), 200
    except Exception as e:
        _clean()
        # raise e
        print(e)
        return jsonify({"success": False, "message": str(e)}), 500


@app.route('/api/version', methods=['GET'])
@cross_origin()
def version():
    return jsonify({"status": "ok"}), 200


if __name__ == "__main__":
    from waitress import serve

    host = os.getenv("HTTP_HOST", "0.0.0.0")
    # host = os.getenv("HTTP_HOST", "127.0.0.1")
    port = os.getenv("HTTP_PORT", 80)

    print(f"Listening on {host}:{port}")

    serve(app, host=host, port=port)
