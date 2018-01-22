use std::io::{Bytes, Read};
use std::iter::Peekable;

struct Parser<R>
where
    R: Sized + Read,
{
    r: Peekable<Bytes<R>>,
}

impl<R: Sized + Read> Parser<R> {
    fn new(r: R) -> Parser<R>
    where
        R: Read + Sized,
    {
        Parser { r: r.bytes().peekable() }
    }
}
