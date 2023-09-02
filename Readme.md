# event-file-reader

Generic event file reader.

This crate provides the [EventFileReader] struct for reading
scattering event files in different formats. See the [Features
section](#features) for a list of supported formats and the [avery
crate](https://docs.rs/avery/latest/avery/event/struct.Event.html)
for the format of the returned events.

## Example

```rust
use event_file_reader::EventFileReader as Reader;

let reader = Reader::new("events.lhe.gz")?;
for event in reader {
  let event = event?;
  // do something with the event
}
```

## Features

### Default Features

- `lhef`: Support for event files in the [Les Houches Event File format](https://crates.io/crates/lhef).
- `hepmc2`: Support for event files in the [HepMC 2 format](https://crates.io/crates/hepmc2).
- `flate2`: Support for DEFLATE compressed event files, e.g. gzip.
- `zstd`: Support for event files compressed with [zstd](https://en.wikipedia.org/wiki/Zstd).

### Non-default Features

- `all`: Enable all mutually compatible features. Use `--features all` instead of `--all-features`.
- `bzip2`: Support for [bzip2](https://en.wikipedia.org/wiki/Bzip2) compressed event files.
- `ntuple`: Support for [ntuple event files](https://crates.io/crates/ntuple).
- `lz4`: Support for [lz4](https://en.wikipedia.org/wiki/LZ4_(compression_algorithm)) compressed event files using the [lz4 crate](https://crates.io/crates/lz4). Incompatible with the `lz4_flex` feature.
- `lz4_flex`: Support for [lz4](https://en.wikipedia.org/wiki/LZ4_(compression_algorithm)) compressed event files using the [lz4_flex crate](https://crates.io/crates/lz4_flex). Incompatible with the `lz4` feature.


License: GPL-3.0-or-later
