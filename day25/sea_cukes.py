class SeaCucumberHerd:
    def __init__(self, input):
        self.lines = input.split('\n')
        self.num_lines = len(self.lines)
        self.line_len = len(self.lines[0])

    def __str__(self):
        return '\n'.join(self.lines)
    
    def get_value(self, x, y):
        return self.lines[y][x]

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
        self.move_south()
    
    def move_east(self):
        lines = []

        for y in range(self.num_lines):
            line = ''
            for x in range(self.line_len):
                value = self.get_value(x, y)
                next_value = self.get_next_x_value(x, y)
                last_value = self.get_previous_x_value(x, y)

                if value == '>' and next_value == '.':
                    line += '.'
                elif value == '>' and next_value != '.':
                    line += '>'
                elif value == '.' and last_value == '>':
                    line += '>'
                else: 
                    line += value
            lines.append(line)
        self.lines = lines

    def get_next_y_value(self, x, y):
        next_index = y + 1 

        if next_index == self.num_lines:
            next_index = 0

        return self.get_value(x, next_index)
        
    def get_previous_y_value(self, x, y):
        previous_index = y - 1

        if previous_index == -1:
            previous_index = self.num_lines - 1

        return self.get_value(x, previous_index)


    def move_south(self):
        lines = []

        for y in range(self.num_lines):
            line = ''
            for x in range(self.line_len):
                value = self.get_value(x, y)
                next_value = self.get_next_y_value(x, y)
                last_value = self.get_previous_y_value(x, y)

                if value == 'v' and next_value == '.':
                    line += '.'
                elif value == 'v' and next_value != '.':
                    line += 'v'
                elif value == '.' and last_value == 'v':
                    line += 'v'
                else: 
                    line += value
            lines.append(line)
        self.lines = lines