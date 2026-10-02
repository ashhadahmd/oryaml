import oryaml


def test_loads_int_key():
    assert oryaml.loads("1: a\n2: b\n") == {1: "a", 2: "b"}


def test_loads_bool_key():
    assert oryaml.loads("true: a\nfalse: b\n") == {True: "a", False: "b"}


def test_loads_null_key():
    assert oryaml.loads("null: a\n") == {None: "a"}


def test_loads_float_key():
    assert oryaml.loads("1.5: a\n") == {1.5: "a"}


def test_dumps_int_key():
    assert oryaml.loads(oryaml.dumps({1: "a", 2: "b"})) == {1: "a", 2: "b"}


def test_dumps_bool_key():
    assert oryaml.loads(oryaml.dumps({True: "a", False: "b"})) == {True: "a", False: "b"}


def test_dumps_none_key():
    assert oryaml.loads(oryaml.dumps({None: "a"})) == {None: "a"}


def test_dumps_float_key():
    assert oryaml.loads(oryaml.dumps({1.5: "a"})) == {1.5: "a"}


def test_dumps_mixed_key_types():
    obj = {2: "int", "s": "str", True: "bool", None: "none"}
    assert oryaml.loads(oryaml.dumps(obj)) == obj
