import os
import json
import csv
import io

"""
Load the json from given file name and then deserialize it into a python dict
"""
def load_json(filename):
    file  = open("responses/{filename}".format(filename=filename))
    data = json.load(file)
    file.close()
    return data

def load_csv_from_file(filename):
    file = open("responses/{filename}".format(filename=filename))
    data = parse_csv(file)
    file.close()
    return data


def load_csv_from_string(text):
    stream = io.StringIO(text)
    data = parse_csv(stream)
    stream.close()
    return data

def parse_csv(io_stream):
    objects = []
    input_file = csv.DictReader(io_stream)
    for row in input_file:
        objects.append(row)
    return objects

def sanitize_substrates(substrates):
    for substrate in substrates:
        substrate["sources"] = set(substrate["sources"].split(","))
        substrate["pmids"] = set(substrate["pmids"].split(","))
        substrate["enzymes"] = set(substrate["enzymes"].split(","))
