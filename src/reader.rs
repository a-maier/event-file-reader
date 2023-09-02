use std::{io::BufReader, fs::File, path::Path};

use audec::auto_decompress;
use avery::Event;
use thiserror::Error;

pub struct EventFileReader (
    Box<dyn Iterator<Item = Result<Event, Error>>>
);

impl EventFileReader {
    pub fn new<P: AsRef<Path>>(infile: P) -> Result<Self, Error> {
        let file = File::open(infile.as_ref())?;
        let mut input = auto_decompress(BufReader::new(file));
        let buf = input.fill_buf()?;
        #[cfg(feature = "ntuple")]
        if buf.starts_with(b"root") {
            return Ok(Self(Box::new(crate::ntuple::Reader::new(infile)?)));
        }
        // TODO: trim_ascii_start as soon as it's stable
        let buf = trim_ascii_start(buf);
        #[cfg(feature = "lhef")]
        if buf.starts_with(b"<LesHouchesEvents") {
            return Ok(Self(Box::new(crate::lhef::Reader::new(input)?)));
        }
        #[cfg(feature = "hepmc2")]
        if buf.starts_with(b"HepMC") {
            return Ok(Self(Box::new(crate::hepmc2::Reader::new(input)?)));
        }
        Err(Error::UnknownFormat)
    }
}

impl Iterator for EventFileReader {
    type Item = Result<Event, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

fn trim_ascii_start(buf: &[u8]) -> &[u8] {
    if let Some(pos) = buf.iter().position(|b| !b.is_ascii_whitespace()) {
        &buf[pos..]
    } else {
        &[]
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to determine event file format")]
    UnknownFormat,
    #[error("I/O error")]
    IOError(#[from] std::io::Error),
    #[cfg(feature = "lhef")]
    #[error("LHEF error")]
    LHEFError(#[from] lhef::reader::ReadError),
    #[cfg(feature = "hepmc2")]
    #[error("HepMC2 error")]
    HepMC2Error(#[from] hepmc2::reader::LineParseError),
    #[cfg(feature = "ntuple")]
    #[error("ntuple error")]
    NTupleError(#[from] ntuple::reader::ReadError),
    #[cfg(feature = "ntuple")]
    #[error("Failed to read from ROOT file")]
    NTupleConstructError,
}
