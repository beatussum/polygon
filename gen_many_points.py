#! /usr/bin/env python3


from random import uniform
from copy import deepcopy
from sys import argv

def many_inclusions(n):
    polys = [None for i in range(n)]
    bottom_left = (0, 0)
    top_right = (1, 1)
    for i in range(n):
        bottom_left = (bottom_left[0] - uniform(1, 5), bottom_left[1] - uniform(1, 5))
        top_right = (top_right[0] + uniform(1, 5), top_right[1] + uniform(1, 5))
        polys[i] = (
                bottom_left,
                ((top_right[0] + bottom_left[0]) / 2, bottom_left[1]),
                (top_right[0], bottom_left[1]),
                (top_right[0], (top_right[1] + bottom_left[1]) / 2),
                top_right,
                ((top_right[0] + bottom_left[0]) / 2, top_right[1]),
                (bottom_left[0], top_right[1]),
                (bottom_left[0], (top_right[1] + bottom_left[1]) / 2),
                )
    return polys

def no_inclusions(n):
    polys = [None for i in range(n)]
    bottom_left = (0, 0)
    top_right = (1, 1)
    polys[0] = (
            bottom_left,
            ((top_right[0] + bottom_left[0]) / 2, bottom_left[1]),
            (top_right[0], bottom_left[1]),
            (top_right[0], (top_right[1] + bottom_left[1]) / 2),
            top_right,
            ((top_right[0] + bottom_left[0]) / 2, top_right[1]),
            (bottom_left[0], top_right[1]),
            (bottom_left[0], (top_right[1] + bottom_left[1]) / 2),
            )
    mid = (n - 1) // 2 + 1
    for i in range(1, mid):
        bottom_left = (top_right[0] + 1, 0)
        top_right = (top_right[0] + 2, 1)
        polys[i] = (
                bottom_left,
                ((top_right[0] + bottom_left[0]) / 2, bottom_left[1]),
                (top_right[0], bottom_left[1]),
                (top_right[0], (top_right[1] + bottom_left[1]) / 2),
                top_right,
                ((top_right[0] + bottom_left[0]) / 2, top_right[1]),
                (bottom_left[0], top_right[1]),
                (bottom_left[0], (top_right[1] + bottom_left[1]) / 2),
                )
    bottom_left = (0, 0)
    top_right = (1, 1)
    for i in range(mid, n):
        bottom_left = (0, top_right[1] + 1)
        top_right = (1, top_right[1] + 2)
        polys[i] = (
                bottom_left,
                ((top_right[0] + bottom_left[0]) / 2, bottom_left[1]),
                (top_right[0], bottom_left[1]),
                (top_right[0], (top_right[1] + bottom_left[1]) / 2),
                top_right,
                ((top_right[0] + bottom_left[0]) / 2, top_right[1]),
                (bottom_left[0], top_right[1]),
                (bottom_left[0], (top_right[1] + bottom_left[1]) / 2),
                )
    return polys

def false_inclusions(n):
    polys = [None for i in range(n)]
    current = [[0, 0], [2, 0], [2, 1], [1, 1], [1, 2], [2, 2], [2, 3], [0, 3]]
    polys[0] = current
    for i in range(1, n):
        current = deepcopy(current)
        current[0][0] -= 2
        current[0][1] -= 2
        current[1][0] += 1
        current[1][1] -= 2
        current[2][0] += 1
        current[2][1] -= 2
        current[3][0] -= 2
        current[3][1] -= 2
        current[4][0] -= 2
        current[4][1] += 2
        current[5][0] += 1
        current[5][1] += 2
        current[6][0] += 1
        current[6][1] += 2
        current[7][0] -= 2
        current[7][1] += 2
        polys[i] = current
    return polys

def print_polys(polys):
    for i in range(len(polys)):
        for point in polys[i]:
            print(i, point[0], point[1])

def print_usage():
    print("Usage:")
    print(f"{argv[0]} <algorithm> <number_of_polygons_to_generate>")
    print("<algorithm> can be many_inclusions, no_inclusions, or false_inclusions")
    print("<number_of_polygons_to_generate> is a strictly positive integer")

if __name__ == "__main__":
    if len(argv) != 3:
        print_usage()
    else:
        try:
            n = int(argv[2])
        except(ValueError):
            print_usage()
            exit()
        if n < 1:
            print_usage()
            exit()
        match argv[1]:
            case "many_inclusions":
                polys = many_inclusions(n)
            case "no_inclusions":
                polys = no_inclusions(n)
            case "false_inclusions":
                polys = false_inclusions(n)
            case _:
                print_usage()
                exit()
        print_polys(polys)
