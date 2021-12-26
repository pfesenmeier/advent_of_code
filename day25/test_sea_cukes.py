from sea_cukes import SeaCucumberHerd

def test_get_value():
    # ..>
    # v..
    # ..>
    herd = SeaCucumberHerd("..>\nv..\n..>")

    assert herd.get_value(2,0) == '>'
    assert herd.get_value(0,1) == 'v'
    assert herd.get_value(2,2) == '>'

def test_oneline_input_move_eastward():
    herd = SeaCucumberHerd('...>>>>>...')

    herd.move()
    second_state = herd.to_string()
    herd.move()
    third_state = herd.to_string()

    assert second_state == '...>>>>.>..'
    assert third_state == '...>>>.>.>.'
