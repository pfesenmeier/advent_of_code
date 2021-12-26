from main import part_1


def test_part_1():
    with open("sample_input.txt") as input:
        input = input.read()
        assert part_1(input) == 58
