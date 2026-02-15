import gggp


def test_version_is_string():
    v = gggp.version()
    assert isinstance(v, str)
    assert v.count(".") >= 1


def test_one_point_crossover():
    a = [1, 2, 3, 4]
    b = [10, 20, 30, 40]
    c1, c2 = gggp.one_point_crossover(a, b, split=2)
    assert c1 == [1, 2, 30, 40]
    assert c2 == [10, 20, 3, 4]


def test_mutate_gene():
    result = gggp.mutate_gene([1, 2, 3], index=1, value=99)
    assert result == [1, 99, 3]

