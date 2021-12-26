from sea_cukes import SeaCucumberHerd
# 1. Simulate "move" for one line of cucumbers of east-moving ('>') species 
# 2. Reorganize this logic into the class 'SeaCukes'
# 3. Do multiple lines of cucumbers
# 4. Do the south-moving ('v') cucumbers
# 5. Implement logic to determine if field has moved


def move(input):
    result = ''

    for i in range(len(input)):
    # while i < max_x:
        value = input[i]

        next_index = i + 1
        if next_index == len(input):
            next_index = 0
        
        last_index = i - 1
        if last_index == -1:
            last_index = len(input) - 1

        next_value = input[next_index]
        last_value = input[last_index]

        if value == '>' and next_value == '.':
            result += '.'
        elif value == '>' and next_value == '>':
            result += '>'
        elif value == '.' and last_value == '>':
            result += '>'
        else: 
            result += value

    return result

# testing...
input = '...>>>>>...'
expected_second_state = '...>>>>.>..'
expected_third_state = '...>>>.>.>.'

second_state = move(input)
third_state = move(second_state)
print('expected 2nd:', expected_second_state)
print('actual 2nd:', second_state)
print('expected 3rd:', expected_third_state)
print('actual 3rd:', third_state)
assert second_state == expected_second_state
assert third_state == expected_third_state

        

    
    






















# with open("sample_input.txt") as input:
#     input = input.read()
#     print('input: ')
#     print(input)
#     lines = input.split("\n")

#     print('lines: ')
#     print(lines)

#     # for height in range(len(lines)):
#     #     for width in range(len(lines[0])):
#     #         # print(lines[height][width])