class SeaCucumberHerd:
    # input is a list of rows
    # input is '..>\n..v\n<<<'
    def __init__(self, input):
        self.rows = input.split('\n')
        self.num_rows = len(self.rows)
        self.len_row = len(self.rows[0])

    def get_cukes(self):
        return '\n'.join(self.rows)
    
    def get_value(self, x, y):
        return self.rows[y][x]

    def get_next_x_value(self, x, y):
        next_x_index = self.get_next_x_index(x)
        return self.get_value(next_x_index, y)
        
    def get_previous_x_value(self, x, y):
        previous_x_index = self.get_previous_x_index(x)
        return self.get_value(previous_x_index, y)

    def get_next_x_index(self, i):
        next_index = i + 1
        if next_index == self.len_row:
            next_index = 0

        return next_index

    def get_previous_x_index(self, i):
        last_index = i - 1
        if last_index == -1:
            last_index = self.len_row - 1
        
        return last_index
    def move(self):
        self.move_east()
        # self.move_south()
    
    def move_east(self):
        result = []

        for y in range(self.num_rows):
            for x in range(self.len_row):
                row = ''

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
second_state = herd.get_cukes()
herd.move()
third_state = herd.get_cukes()
print('expected 2nd:', expected_second_state)
print('actual 2nd:', second_state)
print('expected 3rd:', expected_third_state)
print('actual 3rd:', third_state)
assert second_state == expected_second_state
assert third_state == expected_third_state