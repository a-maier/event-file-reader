use std::io::BufRead;

use avery::Event;

use crate::reader::Error;

pub(crate) struct Reader<T> (
    lhef::Reader<T>
);

impl<T: BufRead> Reader<T> {
    pub(crate) fn new(input: T) -> Result<Self, Error> {
        let reader = lhef::Reader::new(input)?;
        Ok(Self(reader))
    }
}

impl<T: BufRead> Iterator for Reader<T> {
    type Item = Result<Event, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let ev = self.0.hepeup();
        match ev {
            Ok(Some(ev)) => Some(Ok((self.0.heprup().to_owned(), ev).into())),
            Ok(None) => None,
            Err(err) => Some(Err(err.into())),
        }
    }
}
