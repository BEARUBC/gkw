import math
from influxdb_client import InfluxDBClient, WriteOptions
import pandas as pd

url = "http://localhost:8086"
token = "Rbg3aKBu-nU_wY9wXkxCVLzT9WhH725mZ6LwEQgQjrppmeLYZ1J9xrjqXlZz6-oLfDJQhJWE169pyaN9rpmDzg=="
org = "0ed254cf3dca2b2b"
bucket = "GRASPDB"


class ContinuousEMGModel:
    def __init__(self, sensitivity=10):
        self.theta = [x / sensitivity for x in range(sensitivity, 0, -1)]
        self.cache_size = len(self.theta)
        self.cache = []
        self.results = []

    def add_to_cache(self, val):
        self.cache.append(val)
        if len(self.cache) >= self.cache_size:
            self.cache.pop(0)

    def apply_model_to_df(self, data_df):
        y_data = list(data_df)
        y = [0] * len(y_data)

        for j in range(len(self.theta), len(y)):
            for i in range(len(self.theta)):
                y[j] += self.theta[i] * y_data[j - i]

        pd_df = pd.DataFrame([(i, y[i]) for i in range(len(y))], columns=["x", "y"])
        pd_df["y"] = pd_df["y"].map(lambda x: ((1 / (1 + math.exp(-x))) - 0.5) * 2)
        return pd_df

    def next_value(self, val):
        self.add_to_cache(val)
        y = 0
        for i in range(len(self.cache)):
            y += self.theta[i] * self.cache[len(self.cache) - i - 1]

        y /= sum(self.theta)

        self.results.append(y)
        return y

    def influx_write(self, measure_name):

        with InfluxDBClient(url=url, token=token, org=org) as _client:

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
                    bucket,
                    org,
                    record=pd.DataFrame(self.cache),
                    data_frame_measurement_name=measure_name,
                    data_frame_tag_columns=["electrode"],
                )
