from sea_cukes import SeaCucumberHerd
from time import *

input = """>.>.
>>..
..>.
...>"""
herd = SeaCucumberHerd(input)


steps = 0
print(herd)
while steps < 10:
    herd.move()
    print('')
    print(herd)
    sleep(1)
    steps += 1