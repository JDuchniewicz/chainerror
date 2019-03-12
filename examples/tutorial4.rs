use chainerror::*;
use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<Error + Send + Sync>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func2() -> Result<(), Box<Error + Send + Sync>> {
    let filename = "foo.txt";
    do_some_io().map_err(mstrerr!("Error reading '{}'", filename))?;
    Ok(())
}

fn func1() -> Result<(), Box<Error + Send + Sync>> {
    func2().map_err(mstrerr!("func1 error"))?;
    Ok(())
}

fn main() -> Result<(), Box<Error + Send + Sync>> {
    if let Err(e) = func1() {
        eprintln!("{:?}", e);
    }
    Ok(())
}
