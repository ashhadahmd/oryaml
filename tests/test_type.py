import pytest

import oryaml


class TestNone:
    def test_dumps(self):
        assert oryaml.dumps(None) == b"null\n"

    def test_loads(self):
        assert oryaml.loads("null") is None
        assert oryaml.loads("~") is None
        assert oryaml.loads("") is None


class TestBool:
    def test_dumps_true(self):
        assert oryaml.loads(oryaml.dumps(True)) is True

    def test_dumps_false(self):
        assert oryaml.loads(oryaml.dumps(False)) is False

    def test_loads_true(self):
        assert oryaml.loads("true") is True

    def test_loads_false(self):
        assert oryaml.loads("false") is False


class TestInt:
    def test_dumps_zero(self):
        assert oryaml.loads(oryaml.dumps(0)) == 0

    def test_dumps_negative(self):
        assert oryaml.loads(oryaml.dumps(-1)) == -1

    def test_dumps_int64_max(self):
        val = 9223372036854775807
        assert oryaml.loads(oryaml.dumps(val)) == val

    def test_dumps_uint64_max(self):
        val = 18446744073709551615
        assert oryaml.loads(oryaml.dumps(val)) == val

    def test_dumps_overflow(self):
        with pytest.raises(OverflowError):
            oryaml.dumps(18446744073709551616)

    def test_loads_large_unsigned(self):
        assert oryaml.loads("18446744073709551615") == 18446744073709551615

    def test_loads_128bit(self):
        assert oryaml.loads("12345678901234567890123") == 12345678901234567890123
        assert oryaml.loads("-12345678901234567890123") == -12345678901234567890123
        assert oryaml.loads(str(2**128 - 1)) == 2**128 - 1


class TestFloat:
    def test_dumps(self):
        assert oryaml.loads(oryaml.dumps(1.5)) == 1.5

    def test_loads(self):
        assert oryaml.loads("1.5") == 1.5


class TestStr:
    def test_dumps(self):
        assert oryaml.loads(oryaml.dumps("hello")) == "hello"

    def test_dumps_unicode(self):
        assert oryaml.loads(oryaml.dumps("☃")) == "☃"

    def test_loads_quoted(self):
        assert oryaml.loads('"hello"') == "hello"


class TestList:
    def test_dumps_empty(self):
        assert oryaml.loads(oryaml.dumps([])) == []

    def test_dumps(self):
        assert oryaml.loads(oryaml.dumps([1, 2, 3])) == [1, 2, 3]

    def test_dumps_tuple(self):
        assert oryaml.loads(oryaml.dumps((1, 2, 3))) == [1, 2, 3]

    def test_dumps_nested(self):
        obj = [[1, 2], [3, [4, 5]]]
        assert oryaml.loads(oryaml.dumps(obj)) == obj


class TestDict:
    def test_dumps_empty(self):
        assert oryaml.loads(oryaml.dumps({})) == {}

    def test_dumps(self):
        obj = {"a": 1, "b": 2}
        assert oryaml.loads(oryaml.dumps(obj)) == obj


class TestTag:
    def test_local_tag_raises(self):
        with pytest.raises(ValueError, match="unsupported YAML tag"):
            oryaml.loads("a: !Ref foo\n")

    def test_core_tags_resolve(self):
        assert oryaml.loads('!!str 123') == "123"
        assert oryaml.loads('!!int "7"') == 7


class TestType:
    def test_dumps_unsupported_type(self):
        with pytest.raises(TypeError):
            oryaml.dumps(object())

    def test_dumps_set_unsupported(self):
        with pytest.raises(TypeError):
            oryaml.dumps({1, 2, 3})
