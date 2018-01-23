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
        enum X {
            Read,
            Next,
        }
        let x: X;
        match self.peek() {
            Some(r) => {
                match *r {
                    Ok(b' ') => x = Read,
                    Ok(..) => return Err(Error::Illegal),
                    Err(..) => x = Next,
                }
            }
            None => return Ok(None),
        }
        match x {
        }
    }
}
