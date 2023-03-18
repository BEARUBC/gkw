from abc import abstractmethod
import json

from influxdb_client import InfluxDBClient, WriteOptions


class Module:
    @abstractmethod
    def run(self, input_json: dict) -> dict:
        pass

    def influx_write(self, measurement, time):
        with InfluxDBClient(url=self.url, token=self.token, org=self.org) as _client:
            # change write options params based on data batching
            # see https://github.com/influxdata/influxdb-client-python#writes
            with _client.write_api(write_options=WriteOptions(batch_size=500, flush_interval=10_000,
                                                              jitter_interval=2_000, retry_interval=5_000,
                                                              max_retries=5, max_retry_delay=30_000,
                                                              exponential_base=2)) as _write_client:
                pass
