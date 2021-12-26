from sea_cukes import SeaCucumberHerd

def test_get():
    # ..>
    # v..
    # ..>
    input = "..>\nv..\n..>"
    herd = SeaCucumberHerd(input.split('\n'))

    assert herd.get(2,0) == '>'
    assert herd.get(0,1) == 'v'
    assert herd.get(2,2) == '>'
