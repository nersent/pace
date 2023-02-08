from pace import pace

from packages.optimizer.src.pace_glue.timeframe import Timeframe


class AssetDataProvider():
    def __init__(self, path: str, asset_name: str, timeframe: Timeframe):
        self._asset_data_provider = pace.AssetDataProvider(
            path, asset_name, timeframe.value)

    def get_asset_name(self) -> str:
        return self._asset_data_provider.get_asset_name()

    def get_timeframe(self) -> Timeframe:
        return self._asset_data_provider.get_timeframe()
