import pickle
import sys

sys.path.append('application/emg_integration/python/data_to_stdout.py')

with open('battery_simulation_export1.pkl', 'rb') as f:
    data = pickle.load(f)
    sys.stdout.write(data)
    sys.stdout.flush()