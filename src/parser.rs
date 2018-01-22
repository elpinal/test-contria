use std::io;
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

    fn peek(&mut self) -> Option<&Result<u8, io::Error>> {
        self.r.peek()
    }

    fn next(&mut self) -> Option<Result<u8, io::Error>> {
        self.r.next()
    }

    fn read(&mut self) -> Result<Option<u8>, io::Error> {
        match self.peek() {
            Some(r) => {
                match *r {
                    Ok(b' ') => self.read(),
                    Ok(..) => (),
                    Err(..) => self.next(),
                }
            }
            None => Ok(None),
        }
    }
}
