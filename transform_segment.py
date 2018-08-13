import csv

def read_segment(filename):
    segment = []
    with open(filename) as csvfile:
        spamreader = csv.reader(csvfile)
        for row in spamreader:
            segment.append(parse_data(row))
    return segment

def write_segment(segment, filename):
    with open(filename, 'w') as csvfile:
        spamwriter = csv.writer(csvfile)
        for data in segment:
            spamwriter.writerow(data)


def parse_data(row):
    parsed = []
    for data in row:
        try:
            parsed.append(int(data))
        except ValueError:
            parsed.append(data)
    return parsed

def transform_average(segment):
    averaged = []
    tmp = []
    for data in segment:
        tmp.append(data)
        if (len(tmp) > 9):
            averaged.append(calculate_average(tmp))
            tmp = []
    return averaged

def calculate_average(segment):
    points = segment[0][0]
    summed = 0
    for data in segment:
        summed = summed + data[1]
    return [points, summed / len(segment)]


segment = read_segment('segment.csv')
header = segment.pop(0)
# segment = transform_average(segment)
segment.insert(0, header)
write_segment(segment, 'transformed_segment.csv')
