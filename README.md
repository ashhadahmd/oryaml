# oryaml

oryaml is a fast YAML library for Python, written in Rust. On a 100 MiB document it
[parses](#deserialize) about 12x as fast as PyYAML's libyaml-backed `CSafeLoader`, and
returns identical Python objects. It is a typed replacement for `yaml.safe_load()` and
`yaml.safe_dump()`, and it does not construct arbitrary Python objects from tags.

[oryaml.loads()](#deserialize) parses one YAML document from `str` or `bytes` into
`dict`, `list`, `str`, `int`, `float`, `bool` and `None` objects. It resolves the
YAML 1.2 core schema and raises on duplicate keys rather than silently overwriting them.

[oryaml.dumps()](#serialize) serializes Python objects to UTF-8 YAML `bytes`. It can also
write directly to a file path or a binary file-like object.

oryaml supports CPython 3.9, 3.10, 3.11, 3.12, 3.13 and 3.14.

It distributes x86_64, i686, aarch64, armv7, s390x and ppc64le wheels for Linux (manylinux,
plus musllinux for x86_64, i686, aarch64 and armv7), x86_64 and arm64 wheels for macOS,
and x64 and x86 wheels for Windows. Other platforms build from the source distribution.

oryaml is in beta. The API is small and covered by tests, but behavior may change before
1.0. Review [Types](#types) and [Questions](#questions) for differences from PyYAML before
migrating. There is a [CHANGELOG](https://github.com/ashhadahmd/oryaml/blob/main/CHANGELOG.md)
available in the repository.

1. [Usage](#usage)
    1. [Install](#install)
    2. [Quickstart](#quickstart)
    3. [Migrating](#migrating)
    4. [Serialize](#serialize)
    5. [Deserialize](#deserialize)
2. [Types](#types)
3. [Architecture](#architecture)
4. [Testing](#testing)
5. [Performance](#performance)
6. [Questions](#questions)
7. [Packaging](#packaging)
8. [License](#license)

## Usage

### Install

To install a wheel from PyPI, install the `oryaml` package:

```sh
pip install oryaml
```

In `requirements.txt` format, specify:

```txt
oryaml >= 0.1, < 0.2
```

To build from source, see [Packaging](#packaging).

### Quickstart

This is an example of deserializing and serializing:

```python
>>> import oryaml
>>> oryaml.loads("name: oryaml\ntags: [fast, rust]\n")
{'name': 'oryaml', 'tags': ['fast', 'rust']}
>>> oryaml.dumps(_)
b'name: oryaml\ntags:\n- fast\n- rust\n'
```

Writing to a path or to a binary file-like object:

```python
>>> oryaml.dumps({"a": 1}, "out.yaml")
>>> with open("out.yaml", "wb") as f:
...     oryaml.dumps({"a": 1}, f)
```

### Migrating

To migrate from PyYAML, the largest difference is that `oryaml.dumps()` returns `bytes`
and `yaml.safe_dump()` returns a `str`. File objects passed to `dumps()` must be opened in
binary mode.

| PyYAML | oryaml |
| :--- | :--- |
| `yaml.safe_load(text)` | `oryaml.loads(text)` |
| `yaml.safe_dump(obj)` | `oryaml.dumps(obj).decode()` |
| `yaml.safe_dump(obj, f)` | `oryaml.dumps(obj, f)` |

`yaml.safe_dump()` sorts keys by default. `oryaml.dumps()` preserves `dict` insertion
order, and sorting is not configurable.

PyYAML resolves the YAML 1.1 schema, so `yes`, `no`, `on`, `off`, `1_000` and
`2020-01-01` become `bool`, `int` and `datetime.date`. oryaml resolves the YAML 1.2 core
schema, in which these are strings.

PyYAML keeps the last value of a duplicated key. oryaml raises `ValueError`.

`yaml.safe_load_all()` has no equivalent. A document containing more than one `---`
section raises `ValueError`.

### Serialize

```python
def dumps(obj: Any, stream: str | SupportsWrite[bytes] | None = None) -> bytes | None: ...
```

`dumps()` serializes Python objects to YAML.

It natively serializes `str`, `dict`, `list`, `tuple`, `int`, `float`, `bool` and `None`
instances. Subclasses of these types, such as `collections.OrderedDict` and
`enum.IntEnum`, are serialized as their base type. Other types, including `set`,
`bytes`, `datetime` and `enum.Enum`, are not supported.

The output is a `bytes` object containing UTF-8. Mappings and sequences are written in
block style.

If `stream` is a `str`, it is treated as a file path, and the file is created or
truncated. Otherwise `stream` must have a `write()` method that accepts `bytes`. The
document is serialized completely before `write()` is called once, followed by `flush()`
if the stream has one. `dumps()` returns `None` when `stream` is given.

The global interpreter lock (GIL) is held for the duration of the call.

It raises `TypeError` on an unsupported type, with the message
`Object of type '...' is not serializable`.

It raises `OverflowError` on an integer outside the range of a signed 64-bit minimum
to an unsigned 64-bit maximum.

It raises `RecursionError` if the object nests more than 254 levels deep.

It raises `OSError`, or a subclass such as `FileNotFoundError`, if writing to a file
path fails.

Exceptions raised by the stream's own `write()` or `flush()` methods propagate unchanged.

### Deserialize

```python
def loads(data: str | bytes, /) -> Any: ...
```

`loads()` deserializes one YAML document to Python objects. It deserializes to `dict`,
`list`, `tuple` (for sequence keys only), `int`, `float`, `str`, `bool` and `None`
objects.

`bytes` and `str` input are accepted. `bytes` must be valid UTF-8. `bytearray` and
`memoryview` are not accepted.

Anchors and aliases are supported. Each alias produces an independent copy of the
anchored value. Merge keys (`<<`) are not applied and are kept as ordinary keys.

Repeated mapping keys within one call share a single interned `str` object, which
reduces memory use on documents with many records of the same shape.

The global interpreter lock (GIL) is held for the duration of the call.

It raises `ValueError` on invalid YAML, on a document containing more than one
document, on a duplicated mapping key, on an unsupported local tag such as `!Ref`, and
on input nested more than 128 levels deep. The message begins with `YAML Parse Error:`
and includes the line and column.

It raises `UnicodeDecodeError` if `bytes` input is not valid UTF-8.

It raises `TypeError` if the input is not `str` or `bytes`, or if a mapping is used as a
mapping key.

## Types

### str

Strings are UTF-8 in both directions. `dumps()` quotes a string only when the YAML 1.2
core schema would otherwise read it as another type, so `"123"` is written as `'123'`.
Strings such as `"yes"` are written unquoted because they are strings under YAML 1.2.
Parsers that use YAML 1.1, including PyYAML, read them back as `bool`. Multiline strings
are written as literal block scalars (`|-`).

### int

`dumps()` serializes integers from a signed 64-bit minimum (-9223372036854775808) to an
unsigned 64-bit maximum (18446744073709551615) and raises `OverflowError` outside it.

`loads()` deserializes integers up to 128 bits exactly, including hexadecimal (`0x10`)
and octal (`0o17`) forms. Integers beyond 128 bits are deserialized as `float` and lose
precision.

### float

`.nan`, `.inf` and `-.inf` are supported in both directions. Exponent forms such as `1e3`
deserialize as `float`.

### dict keys

Keys may be `str`, `int`, `float`, `bool`, `None` or `tuple` in both directions.
`dumps()` writes a `tuple` key as a YAML sequence, and `loads()` reads a sequence key
back as a `tuple`, so these keys round-trip. A mapping used as a key cannot be
represented in Python and raises `TypeError`.

### tags

The standard scalar tags `!!str`, `!!int`, `!!float`, `!!bool` and `!!null` are applied.
Local tags such as `!Ref` or `!include` raise `ValueError`. The tags `!!binary`, `!!set`,
`!!timestamp` and `!!omap` are currently ignored, and the untagged value is returned.

## Architecture

![oryaml architecture: loads and dumps data flow through the Rust extension](https://raw.githubusercontent.com/ashhadahmd/oryaml/main/docs/architecture.svg)

`loads()` does not build an intermediate tree. `serde_yaml` drives the libyaml parser
(through the pure-Rust `unsafe-libyaml` port), and a visitor in `src/deserialize.rs`
creates Python objects directly from parser events.

`dumps()` converts Python objects to a `serde_yaml::Value` tree, emits it into one buffer,
and only then returns or writes it. An unsupported object therefore fails before
anything reaches the file or stream.

| File | Responsibility |
| :--- | :--- |
| `src/lib.rs` | Python module, `loads()` and `dumps()` entry points, argument checks |
| `src/deserialize.rs` | YAML events to Python objects: key interning, tuple keys, tag checks |
| `src/serialize.rs` | Python objects to YAML bytes, path and stream output |
| `src/cache.rs` | Interned key cache used during one `loads()` call |
| `src/error.rs` | Mapping of internal errors to Python exceptions |
| `oryaml.pyi` | Type stubs, shipped in the wheel with `py.typed` |

## Testing

The test suite covers the public API, every supported type in both directions,
non-`str` keys, error types and messages, recursion limits, and stream and file output,
including exceptions raised by `write()` and `flush()`. It runs in CI on Linux, Windows
and macOS against CPython 3.9 and 3.14 before any release is published.

The benchmark corpus was also deserialized by oryaml and PyYAML, and both produced
identical output for all 302,222 records.

The tests require only `pytest` and are included in the source distribution:

```sh
pytest -q tests
```

## Performance

![Parse throughput: oryaml 24.4 MiB/s, PyYAML CSafeLoader 2.1 MiB/s, ruamel.yaml 0.24 MiB/s](https://raw.githubusercontent.com/ashhadahmd/oryaml/main/docs/benchmark.svg)

Deserialization of a 100 MiB document containing 302,222 flat records of integers,
floats, booleans, strings and short lists:

| Library | Time (s) | Throughput (MiB/s) | vs. oryaml |
| :--- | :--- | :--- | :--- |
| oryaml 0.1.0 | 4.1 | 24.4 | 1.0 |
| PyYAML 6.0.3 (`CSafeLoader`) | 48 | 2.1 | 11.7x slower |
| ruamel.yaml 0.19.1 (`typ="safe"`, pure Python) | 415 | 0.24 | 101x slower |

### Reproducing

The above was measured using CPython 3.13 in a Linux container under Docker Desktop with
12 vCPUs. The oryaml figure is the median of three runs. The corpus is generated by
`tests/generate_yaml.py`, and the oryaml timings can be reproduced from the repository
root with:

```sh
python tests/benchmark.py
```

## Questions

### Will it deserialize to dataclasses, datetimes or custom classes?

No. oryaml is a safe loader and never constructs arbitrary objects from tags. Converting
to richer types belongs in a validation library one level above the parser.

### Will it serialize to `str`?

No. `bytes` is the correct type for a serialized document. Call `.decode()` if a `str`
is required.

### Will it support multiple documents in one stream?

Not yet. Each call accepts exactly one document.

### Will it support YAML 1.1 resolution?

No. oryaml follows the YAML 1.2 core schema.

### Will it preserve comments or formatting for round-trip editing?

No. Use `ruamel.yaml` in round-trip mode for that.

## Packaging

Building oryaml from source requires [Rust](https://www.rust-lang.org/) 1.85 or later and
the [maturin](https://github.com/PyO3/maturin) build tool. A C compiler is not required.
The recommended build command is:

```sh
maturin build --release
```

For local development:

```sh
git clone https://github.com/ashhadahmd/oryaml.git
cd oryaml
pip install maturin pytest
maturin develop --release
pytest -q tests
```

The library does not require any other host-level or Python package to be installed.

## License

oryaml was written by Ashhad Ahmed, copyright 2025 - 2026, and is available under your
choice of the [Apache 2.0](https://github.com/ashhadahmd/oryaml/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/ashhadahmd/oryaml/blob/main/LICENSE-MIT) license.
