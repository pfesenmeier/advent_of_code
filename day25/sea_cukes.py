class SeaCucumberHerd:
    def __init__(self, input):
        self.input = input

    def get_cukes(self):
        return self.input

    def get_next_x_index(self, i):
        next_index = i + 1
        if next_index == len(self.input):
            next_index = 0

        return next_index

    def get_previous_x_index(self, i):
        last_index = i - 1
        if last_index == -1:
            last_index = len(self.input) - 1
        
        return last_index
    
    def move(self):
        result = ''

        for i in range(len(input)):
        # while i < max_x:
            value = self.input[i]
            next_index = self.get_next_x_index(i)
            last_index = self.get_previous_x_index(i)

            next_value = self.input[next_index]
            last_value = self.input[last_index]

            if value == '>' and next_value == '.':
                result += '.'
            elif value == '>' and next_value == '>':
                result += '>'
            elif value == '.' and last_value == '>':
                result += '>'
            else: 
                result += value

        self.input = result

####################

input = '...>>>>>...'
expected_second_state = '...>>>>.>..'
expected_third_state = '...>>>.>.>.'

herd = SeaCucumberHerd(input)

herd.move()
second_state = herd.get_cukes()
herd.move()
third_state = herd.get_cukes()
print('expected 2nd:', expected_second_state)
print('actual 2nd:', second_state)
print('expected 3rd:', expected_third_state)
print('actual 3rd:', third_state)
assert second_state == expected_second_state
assert third_state == expected_third_state