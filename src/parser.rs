use std::io;
use std::io::{Bytes, Read};
use std::iter::Peekable;

struct Parser<R>
where
    R: Sized + Read,
{
    r: Peekable<Bytes<R>>,
}

enum Error {
    Io(io::Error),
    Illegal,
}

impl<R: Sized + Read> Parser<R> {
    fn new(r: R) -> Parser<R>
    where
        R: Read + Sized,
    {
        Parser { r: r.bytes().peekable() }
    }

    fn peek(&mut self) -> Option<&Result<u8, io::Error>> {
        self.r.peek()
    }

    fn next(&mut self) -> Option<Result<u8, io::Error>> {
        self.r.next()
    }

    fn read(&mut self) -> Result<Option<u8>, Error> {
        let mut f = |p: Parser| -> Result<Option<u8>, Error> Ok(None);
        match self.peek() {
            Some(r) => {
                match *r {
                    Ok(b' ') => f = |p| p.read(),
                    Ok(..) => return Err(Error::Illegal),
                    Err(..) => f = |p| p.next(),
                }
            }
            None => return Ok(None),
        }
        f(self)
    }
}
