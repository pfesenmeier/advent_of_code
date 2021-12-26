class SeaCucumberHerd:
    # input is a list of rows
    # input is '..>\n..v\n<<<'
    def __init__(self, input):
        self.rows = input.split('\n')
        self.num_lines = len(self.rows)
        self.line_len = len(self.rows[0])

    def to_string(self):
        return '\n'.join(self.rows)
    
    def get_value(self, x, y):
        return self.rows[y][x]

    def get_next_x_value(self, x, y):
        next_index = x + 1

        if next_index == self.line_len:
            next_index = 0

        return self.get_value(next_index, y)
        
    def get_previous_x_value(self, x, y):
        previous_index = x - 1

        if previous_index == -1:
            previous_index = self.line_len - 1

        return self.get_value(previous_index, y)

    def move(self):
        self.move_east()
        # self.move_south()
    
    def move_east(self):
        result = []

        for y in range(self.num_lines):
            row = ''
            for x in range(self.line_len):

                value = self.get_value(x, y)

                next_value = self.get_next_x_value(x, y)
                last_value = self.get_previous_x_value(x, y)

                if value == '>' and next_value == '.':
                    row += '.'
                elif value == '>' and next_value == '>':
                    row += '>'
                elif value == '.' and last_value == '>':
                    row += '>'
                else: 
                    row += value

            result.append(row)

        self.rows = result

####################

input = '...>>>>>...'
expected_second_state = '...>>>>.>..'
expected_third_state = '...>>>.>.>.'

herd = SeaCucumberHerd(input)

herd.move()
second_state = herd.to_string()
herd.move()
third_state = herd.to_string()

print('expected 2nd:', expected_second_state)
print('actual 2nd:', second_state)
print('expected 3rd:', expected_third_state)
print('actual 3rd:', third_state)
assert second_state == expected_second_state
assert third_state == expected_third_state