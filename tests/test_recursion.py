import pytest

import oryaml


def test_dumps_deeply_nested_list_raises():
    obj = []
    current = obj
    for _ in range(2000):
        new = []
        current.append(new)
        current = new

    with pytest.raises(RecursionError):
        oryaml.dumps(obj)


def test_dumps_deeply_nested_dict_raises():
    obj = {}
    current = obj
    for _ in range(2000):
        new = {}
        current["a"] = new
        current = new

    with pytest.raises(RecursionError):
        oryaml.dumps(obj)


def test_loads_deeply_nested_sequence_raises():
    doc = "a" * 0 + "[" * 2000 + "]" * 2000
    with pytest.raises((RecursionError, ValueError)):
        oryaml.loads(doc)


def test_dumps_moderately_nested_list_succeeds():
    obj = []
    current = obj
    for _ in range(50):
        new = []
        current.append(new)
        current = new

    assert oryaml.loads(oryaml.dumps(obj)) == obj
