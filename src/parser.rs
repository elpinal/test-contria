use std::io::Read;

struct Parser<R>
where
    R: Sized + Read,
{
    r: R,
}

impl<R: Sized + Read> Parser<R> {
    fn new(r: R) -> Parser<R>
    where
        R: Read + Sized,
    {
        Parser { r }
    }
}
