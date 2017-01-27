#!/bin/python3

import sys


time = input().strip()

def time_conversion(time):
    am_or_pm = time[-2:].lower()
    time = time[:-2]
    hour = time[:2]
    if am_or_pm == "am":
        if hour == "12":
            return "00" + time[2:]
        return time
    if am_or_pm == "pm":
        if hour == "12":
            return time
        pm_hour = str(int(hour) + 12)
        return pm_hour + time[2:]
    else:
        raise RuntimeError("Invalid time passed", am_or_pm)

print(time_conversion(time))
