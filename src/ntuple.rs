use std::path::Path;

use avery::Event;

use crate::reader::Error;

pub(crate) struct Reader(ntuple::Reader);

impl Reader {
    pub(crate) fn new<P: AsRef<Path>>(input: P) -> Result<Self, Error> {
        ntuple::Reader::new(input)
            .map(Reader)
            .ok_or_else(|| Error::ConstructNTuple)
    }
}

impl Iterator for Reader {
    type Item = Result<Event, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|res| match res {
            Ok(ev) => Ok(ev.into()),
            Err(err) => Err(err.into()),
        })
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).map(|res| match res {
            Ok(ev) => Ok(ev.into()),
            Err(err) => Err(err.into()),
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    fn last(self) -> Option<Self::Item> {
        self.0.last().map(|res| match res {
            Ok(ev) => Ok(ev.into()),
            Err(err) => Err(err.into()),
        })
    }
}
