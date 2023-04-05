import pickle
import sys
import re
import time

# use data_to_stdout_txt.py instead of this
# TEST emg_integration data using battery_simulation_opened.txt 
sys.path.append(r"C:\\Users\\Ray Ho\\Documents\\UBC BIONICS\\grasp-py\\src\\grasp_analytics\\data_generation\\battery")

with open("C:/Users/Ray Ho/Documents/UBC BIONICS/gkw/application/emg_integration/python/battery_simulation_export.pkl", "rb") as f:
    data = pickle.load(f)   
    pickled = str(pickle.dumps(data,0))
    
    arr = re.search(r"(\\naF-?[0-9]+\.*[0-9]+)+", pickled).group(0).split("\\naF")
    #print(arr)
    # print("------------------------------------")
    # print(pickled)
    # print("------------------------------------")
    for s in arr:
        print(s)
    
    #time.sleep(1)    
        #sys.stdout.flush()
    #sys.stdout.write(pickled)
    
    #sys.stdout.flush() 