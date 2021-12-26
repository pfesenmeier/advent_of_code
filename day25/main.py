from sea_cukes import SeaCucumberHerd

# [x] Simulate "move" for one line of cucumbers of east-moving ('>') species
# [x] Reorganize this logic into the class 'SeaCukes'
# [x] Do multiple lines of cucumbers
# [ ] Do the south-moving ('v') cucumbers
# [ ] Implement logic to determine if field has moved


def part_1(input):
    herd = SeaCucumberHerd(input)

    steps = 0
    last_herd = str(herd)
    herd.move()
    steps += 1

    while str(herd) != last_herd:
        last_herd = str(herd) 
        herd.move()
        steps += 1
    
    return steps
   

if __name__ == "__main__":
    with open("input.txt") as input:
        input = input.read()

        print(part_1(input))
