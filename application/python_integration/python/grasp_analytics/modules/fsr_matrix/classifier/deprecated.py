import tensorflow as tf
import numpy as np
import pandas as pd
import os
from ..map_raw_output import open_serial_connection, collect_reading

input_size = 160
output_size = 4  # nothing, sphere, cyl, cube
data_path = "shuffled_train.csv"

# stupid hacky way to get path to model
sep = os.path.sep
script_dir = os.path.dirname(os.path.realpath(__file__))
model_path = script_dir + sep + "models" + sep + "model.hdf5"
print(model_path)

output_map = ["Nothing", "Foam Sphere", "Wood Cube", "Wood cylinder"]


def get_output(inference):
    return output_map[inference.argmax()]


def normalize(data):
    for y in range(len(data)):
        for x in range(len(data[y])):
            data[y][x] = data[y][x] / 1024
    return data


def load_model(path):
    return tf.keras.models.load_model(path)


class ObjectClassifier:
    def __init__(self, mode="new", save_model=True):
        if mode == "new":
            self.model = self.create_model()
            self.train_model()
            if save_model:
                self.save_model(model_path)
        elif mode == "load":
            self.model = load_model(model_path)

    def classify_object(self, reading):
        return get_output(self.model.predict(np.asarray([reading])))

    def run_test(self):
        ser = open_serial_connection()
        collect_reading(ser)
        while True:
            reading = collect_reading(ser)
            if not reading:
                print("Not getting any readings from the sensor. Try restarting.")
                continue
            inference = self.predict(reading)
            print(str(inference).ljust(20), get_output(inference))

    def load_data(self):
        data = pd.read_csv(data_path)
        for col in data.columns:
            data[col] = data[col].astype(float)
        mask = np.random.rand(len(data)) < 0.8
        train = data[mask]
        test = data[~mask]

        print("formatting train data")
        train_y = train.label.to_numpy()
        train_x = normalize(train.drop("label", axis=1).to_numpy())

        print("formatting test data")
        test_y = test.label.to_numpy()
        test_x = normalize(test.drop("label", axis=1).to_numpy())

        return train_x, train_y, test_x, test_y

    def create_layers(self):
        return [
            tf.keras.layers.Dense(input_size, activation=tf.nn.relu),
            tf.keras.layers.Dense(160),
            tf.keras.layers.Dropout(0.2),
            tf.keras.layers.Dense(100),
            tf.keras.layers.Dense(4, activation=tf.nn.softmax),
        ]

    def create_model(self):
        m = tf.keras.models.Sequential()
        for layer in self.create_layers():
            m.add(layer)
        m.compile(
            optimizer="adam",
            loss="sparse_categorical_crossentropy",
            metrics=["accuracy"],
        )
        return m

    def train_model(self):
        train_x, train_y, test_x, test_y = self.load_data()
        # train model, update tensorboard
        # log_dir = "logs" + datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
        # tensorboard_callback = tf.keras.callbacks.TensorBoard(log_dir=log_dir, histogram_freq=1)
        print(train_x.shape, train_y.shape)
        self.model.fit(train_x, train_y, epochs=50)
        self.model.evaluate(test_x, test_y)

    def predict(self, reading):
        return self.model.predict(reading)

    def save_model(self, path):
        self.model.save(path)


if __name__ == "__main__":
    classifier = ObjectClassifier(mode="load")
    classifier.run_test()
