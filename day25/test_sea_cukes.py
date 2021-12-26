from sea_cukes import SeaCucumberHerd


def test_get_value():
    herd = SeaCucumberHerd(
        """..>
v..
..>"""
    )

    assert herd.get_value(2, 0) == ">"
    assert herd.get_value(0, 1) == "v"
    assert herd.get_value(2, 2) == ">"


def test_to_string():
    input = """>..
.>.
..>"""
    herd = SeaCucumberHerd(input)

    assert input == herd.to_string()


def test_oneline_input_move_eastward():
    herd = SeaCucumberHerd("...>>>>>...")

    herd.move()
    second_state = herd.to_string()
    herd.move()
    third_state = herd.to_string()

    assert second_state == "...>>>>.>.."
    assert third_state == "...>>>.>.>."


def test_multiline_input_move_eastward():
    herd = SeaCucumberHerd(
        """>..
.>.
..>"""
    )

    herd.move()

    assert (
        herd.to_string()
        == """.>.
..>
>.."""
    )


def test_multiline_multidirection_simple():
    herd = SeaCucumberHerd(
        """..........
.>v....v..
.......>..
.........."""
    )

    herd.move()

    assert (
        herd.to_string()
        == """..........
.>........
..v....v>.
.........."""
    )


def test_sample_input():
    initial_state = """v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"""
    first_result = """....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v"""
    second_result = """>.v.v>>..v
v.v.>>vv..
>v>.>.>.v.
>>v>v.>v>.
.>..v....v
.>v>>.v.v.
v....v>v>.
.vv..>>v..
v>.....vv."""
    third_result = """v>v.v>.>v.
v...>>.v.v
>vv>.>v>..
>>v>v.>.v>
..>....v..
.>.>v>v..v
..v..v>vv>
v.v..>>v..
.v>....v.."""

    with open("sample_input.txt") as input:
        input = input.read()

        herd = SeaCucumberHerd(input)

        assert herd.to_string() == initial_state
        herd.move()
        assert herd.to_string() == first_result
        herd.move()
        assert herd.to_string() == second_result
        herd.move()
        assert herd.to_string() == third_result
