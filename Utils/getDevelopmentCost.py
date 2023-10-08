import re
import sys


def get_development_cost(filename):
    """Gets the development cost from file (the returned value type is String)"""
    reg_exp = r'\w+\s+\w+\s+\w+\s+\w+\s+\(\w+\)\s+\$(?P<dev_cost>\-?(\d+,?\d*|\d*,?\d+))'
    columns_pattern = re.compile(reg_exp)
    dev_cost = ""
    dev_cost_file = open(filename, "r")
    lines_dev_cost_file_fp = dev_cost_file.read().splitlines()
    for line in lines_dev_cost_file_fp:
        matches = columns_pattern.finditer(line)
        for match in matches:
            dev_cost = match.group("dev_cost")
    dev_cost_file.close()

    dev_cost = dev_cost.replace(',', '')

    return dev_cost

if __name__ == "__main__":
    print(get_development_cost(sys.argv[1]))
