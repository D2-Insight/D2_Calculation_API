
import numpy as np
import matplotlib.pyplot as plt
import math
import json


with open('data.json', 'r') as f:
    jData = json.load(f)

valid_formuals = set()

x_lst = []
y_lst = []
y_less_lst = []
for i in jData["data"]:
    x = int(i)
    y = int(jData["data"][i])
    x_lst.append(x)
    y_lst.append(y)
    y_less_lst.append(y-1)


def does_linear_formula_fit(_formula: np.ndarray, _x_lst: list, _y_lst: list, _y_lst2: list) -> bool:
    """Checks if the linear formula fits the data"""
    formula_line = list(np.polyval(_formula, _x_lst))
    for i in range(len(_x_lst)):
        if _y_lst[i] < formula_line[i] or _y_lst2[i] > formula_line[i]:
            return False
    return True


high_line = plt.plot(x_lst, y_lst)
low_line = plt.plot(x_lst, y_less_lst)
if jData["fomrula_degree"] == 1:
    intersect_lines = []
    for i in range(len(low_line[0].get_xydata())):
        x1 = low_line[0].get_xydata()[i][0]
        y1 = low_line[0].get_xydata()[i][1]
        for j in range(len(high_line[0].get_xydata())):
            x2 = high_line[0].get_xydata()[j][0]
            y2 = high_line[0].get_xydata()[j][1]
            line = ([x1, y1], [x2, y2])
            intersect_lines.append(line)
    # if line in intersect_lines stays inbetween the high and low line then it is a valid line
    for i in intersect_lines:
        x1 = i[0][0]
        y1 = i[0][1]
        x2 = i[1][0]
        y2 = i[1][1]
        for j in range(len(high_line[0].get_xydata())):
            x3 = high_line[0].get_xydata()[j][0]
            y3 = high_line[0].get_xydata()[j][1]
            for k in range(len(low_line[0].get_xydata())):
                x4 = low_line[0].get_xydata()[k][0]
                y4 = low_line[0].get_xydata()[k][1]
                if x1 <= x3 and x2 >= x4:
                    try:
                        linear = np.polyfit([x1, x2], [y1, y2], 1)
                    except:
                        continue
                    if does_linear_formula_fit(linear, x_lst, y_lst, y_less_lst):
                        valid_formuals.add((linear[0], linear[1]))
                        plt.plot(x_lst, np.polyval(linear, x_lst), 'r-')

if not valid_formuals:
    jData["fomrula_degree"] = 2
    print("no linear could be found, trying quadratic")

linear_avg = (np.polyfit(x_lst, y_lst, jData["fomrula_degree"]) + np.polyfit(
    x_lst, y_less_lst, jData["fomrula_degree"]))/2
valid_formuals.add(tuple(linear_avg))
plt.plot(x_lst, np.polyval(linear_avg, x_lst), 'r-')

print(valid_formuals)
if jData["show_gui"]:
    plt.show()
