import csv

def read_time(filename):
    time = []
    with open(filename) as csvfile:
        spamreader = csv.reader(csvfile)
        for row in spamreader:
            time.append(parse_data(row))
    return time

def write_time(time, filename):
    with open(filename, 'w') as csvfile:
        spamwriter = csv.writer(csvfile)
        for data in time:
            spamwriter.writerow(data)


def parse_data(row):
    parsed = []
    for data in row:
        try:
            parsed.append(int(data))
        except ValueError:
            parsed.append(data)
    return parsed

def transform_average(time):
    averaged = []
    tmp = []
    for data in time:
        tmp.append(data)
        if (len(tmp) > 9):
            averaged.append(calculate_average(tmp))
            tmp = []
    return averaged

def calculate_average(time):
    points = time[0][0]
    second = 0
    nano = 0
    for data in time:
        second = second + data[1]
        nano = nano + data[2]
    return [points, second / len(time), nano / len(time)]

def convert_miliseconds(time):
    converted = []
    for data in time:
        mili = data[1] * 10**3 + data[2] * 10**-6
        converted.append([data[0], mili])
    return converted

time = read_time('time.csv')
header = time.pop(0)
header = ['interest points', 'time (ms)']
# time = transform_average(time)
time = convert_miliseconds(time)
time.insert(0, header)
write_time(time, 'transformed_time.csv')
