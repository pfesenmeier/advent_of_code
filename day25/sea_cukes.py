class SeaCucumberHerd:
    def __init__(self, cucumbers):
        self.cucumbers = cucumbers

    def get(self, x, y):
        return self.cucumbers[y][x]
