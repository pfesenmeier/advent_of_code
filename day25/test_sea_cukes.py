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

    assert input == str(herd)


def test_oneline_input_move_eastward():
    herd = SeaCucumberHerd("...>>>>>...")

    herd.move()
    second_state = str(herd)
    herd.move()
    third_state = str(herd)

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
        str(herd)
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
        str(herd)
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

        assert str(herd) == initial_state
        herd.move()
        assert str(herd) == first_result
        herd.move()
        assert str(herd) == second_result
        herd.move()
        assert str(herd) == third_result
