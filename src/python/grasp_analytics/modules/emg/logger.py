from influxdb_client import InfluxDBClient, WriteOptions
import pandas as pd
from src.grasp_analytics.definitions import SETTINGS


class Logger:
    def __init__(self, measurement_name, df, tags=[]):
        self.measure_name = (
            measurement_name  # name of measurement for writing Pandas DataFrame
        )
        self.tag_columns = tags  # list of DataFrame columns which are tags, rest columns will be fields
        self.df = df
        self.settings = SETTINGS["emg"]["logging"]
        self.url = self.settings["url"]
        self.token = self.settings["token"]
        self.org = self.settings["org"]
        self.bucket = self.settings["bucket"]

    def influx_write(self):
        with InfluxDBClient(url=self.url, token=self.token, org=self.org) as _client:
            # change write options params based on data batching
            # see https://github.com/influxdata/influxdb-client-python#writes
            with _client.write_api(
                write_options=WriteOptions(
                    batch_size=500,
                    flush_interval=10_000,
                    jitter_interval=2_000,
                    retry_interval=5_000,
                    max_retries=5,
                    max_retry_delay=30_000,
                    exponential_base=2,
                )
            ) as _write_client:
                _write_client.write(
                    self.bucket,
                    self.org,
                    record=pd.DataFrame(self.df),
                    data_frame_measurement_name=self.measure_name,
                    data_frame_tag_columns=self.tag_columns,
                )
