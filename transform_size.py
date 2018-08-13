import csv

def read_size(filename):
    size = []
    with open(filename) as csvfile:
        spamreader = csv.reader(csvfile)
        for row in spamreader:
            size.append(parse_data(row))
    return size

def write_size(size, filename):
    with open(filename, 'w') as csvfile:
        spamwriter = csv.writer(csvfile)
        for data in size:
            spamwriter.writerow(data)

def parse_data(row):
    parsed = []
    for data in row:
        try:
            parsed.append(int(data))
        except ValueError:
            parsed.append(data)
    return parsed

def sort_point(elem):
    return elem[0]

def transform_mb(size):
    transformed = []
    for data in size:
        transformed.append([data[0], data[1] * 10 ** -6])
    return transformed


size = read_size('size.csv')
header = size.pop(0)
size.sort(key=sort_point)
size = transform_mb(size)
size.insert(0, header)
write_size(size, 'transformed_size.csv')
