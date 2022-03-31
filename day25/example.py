from sea_cukes import SeaCucumberHerd
from time import *


with open('sample_input.txt') as input:
    input = input.read()

    herd = SeaCucumberHerd(input)

    while True:
        herd.move()
        print('')
        print(herd)
        sleep(1)