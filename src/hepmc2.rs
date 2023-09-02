use std::io::BufRead;

use avery::Event;

use crate::reader::Error;

pub(crate) struct Reader<T> (
    hepmc2::Reader<T>
);

impl<T: BufRead> Reader<T> {
    pub(crate) fn new(input: T) -> Result<Self, Error> {
        let reader = hepmc2::Reader::new(input);
        Ok(Self(reader))
    }
}

impl<T: BufRead> Iterator for Reader<T> {
    type Item = Result<Event, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|res| match res {
            Ok(ev) => Ok(ev.into()),
            Err(err) => Err(err.into()),
        })
    }
}
