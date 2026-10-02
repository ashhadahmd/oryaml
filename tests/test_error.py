import pytest

import oryaml


def test_loads_invalid_yaml_raises_valueerror():
    with pytest.raises(ValueError):
        oryaml.loads("a: [1, 2\n")


def test_loads_invalid_utf8_bytes_raises_unicodedecodeerror():
    with pytest.raises(UnicodeDecodeError):
        oryaml.loads(b"\xff\xfe")


def test_loads_rejects_non_str_bytes_input():
    with pytest.raises(TypeError):
        oryaml.loads(1)


def test_loads_rejects_none_input():
    with pytest.raises(TypeError):
        oryaml.loads(None)


def test_loads_rejects_list_input():
    with pytest.raises(TypeError):
        oryaml.loads([])


def test_dumps_unserializable_type_raises_typeerror():
    with pytest.raises(TypeError):
        oryaml.dumps(object())


def test_dumps_int_overflow_raises_overflowerror():
    with pytest.raises(OverflowError):
        oryaml.dumps(2**64)


def test_dumps_stream_rejects_invalid_type():
    with pytest.raises(TypeError):
        oryaml.dumps({"a": 1}, stream=123)
