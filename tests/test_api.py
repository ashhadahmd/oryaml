import io

import pytest

import oryaml


def test_loads_exists():
    assert callable(oryaml.loads)


def test_dumps_exists():
    assert callable(oryaml.dumps)


def test_loads_signature():
    with pytest.raises(TypeError):
        oryaml.loads()


def test_dumps_signature():
    with pytest.raises(TypeError):
        oryaml.dumps()


def test_dumps_returns_bytes():
    assert isinstance(oryaml.dumps({"a": 1}), bytes)


def test_loads_accepts_str():
    assert oryaml.loads("a: 1") == {"a": 1}


def test_loads_accepts_bytes():
    assert oryaml.loads(b"a: 1") == {"a": 1}


def test_roundtrip():
    obj = {"a": 1, "b": [1, 2, 3], "c": {"d": None}}
    assert oryaml.loads(oryaml.dumps(obj)) == obj


def test_dumps_to_binary_stream():
    buf = io.BytesIO()
    assert oryaml.dumps({"a": 1}, buf) is None
    assert oryaml.loads(buf.getvalue()) == {"a": 1}


def test_dumps_to_path(tmp_path):
    path = tmp_path / "out.yaml"
    oryaml.dumps({"a": 1}, str(path))
    assert oryaml.loads(path.read_bytes()) == {"a": 1}


def test_dumps_to_missing_dir_raises_filenotfounderror(tmp_path):
    with pytest.raises(FileNotFoundError):
        oryaml.dumps({"a": 1}, str(tmp_path / "missing" / "out.yaml"))


def test_dumps_text_stream_error_propagates(tmp_path):
    with open(tmp_path / "out.yaml", "w") as f, pytest.raises(TypeError):
        oryaml.dumps({"a": 1}, f)


def test_dumps_stream_write_error_propagates():
    class Broken:
        def write(self, data):
            raise BrokenPipeError("gone")

    with pytest.raises(BrokenPipeError):
        oryaml.dumps({"a": 1}, Broken())


def test_dumps_stream_flush_error_propagates():
    class FailingFlush(io.BytesIO):
        def flush(self):
            raise OSError("flush failed")

    with pytest.raises(OSError, match="flush failed"):
        oryaml.dumps({"a": 1}, FailingFlush())
