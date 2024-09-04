import re, time, ctypes

NUMBER_OF_MINUTES_TO_COOLING_DOWN = 1

# Iterate over the input files
with open("../RAPL/main.c", 'r') as f:
    data = f.read()

lib = ctypes.CDLL('../RAPL/sensors.so')

# Define the return type of the function
lib.getTemperature.restype = ctypes.c_float

print("Cooling Down...")
time.sleep(60*NUMBER_OF_MINUTES_TO_COOLING_DOWN)

data = re.sub(r'#define TEMPERATURETHRESHOLD .*', f'#define TEMPERATURETHRESHOLD {lib.getTemperature()}', data)
# Write the updated file
with open("../RAPL/main.c", 'w') as f:
    f.write(data)
