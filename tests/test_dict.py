import pytest

import oryaml


def test_dict_key_order_preserved():
    obj = {"z": 1, "a": 2, "m": 3}
    assert list(oryaml.loads(oryaml.dumps(obj)).keys()) == ["z", "a", "m"]


def test_dict_nested():
    obj = {"a": {"b": {"c": 1}}}
    assert oryaml.loads(oryaml.dumps(obj)) == obj


def test_dict_mixed_value_types():
    obj = {"a": 1, "b": "two", "c": 3.0, "d": None, "e": True, "f": [1, 2]}
    assert oryaml.loads(oryaml.dumps(obj)) == obj


def test_dict_large():
    obj = {f"key_{i}": i for i in range(1000)}
    assert oryaml.loads(oryaml.dumps(obj)) == obj


def test_dict_duplicate_keys_raises():
    # Unlike PyYAML (which silently keeps the last value), serde_yaml
    # rejects duplicate mapping keys as invalid YAML.
    doc = "a: 1\na: 2\n"
    with pytest.raises(ValueError):
        oryaml.loads(doc)


def test_dict_tuple_key_roundtrip():
    obj = {(1, (2, "x")): "a"}
    assert oryaml.loads(oryaml.dumps(obj)) == obj


def test_dict_sequence_key_loads_as_tuple():
    assert oryaml.loads("? [1, 2]\n: a\n") == {(1, 2): "a"}


def test_dict_mapping_key_loads_raises():
    with pytest.raises(TypeError):
        oryaml.loads("? {a: 1}\n: b\n")


def test_dict_unsupported_key_type():
    with pytest.raises(TypeError):
        oryaml.dumps({object(): "a"})
