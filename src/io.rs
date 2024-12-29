use crate::_cunistd;

pub use no_std_io2::io::*;

pub struct File {
    fd: i32,
}

impl File {
    pub fn open(path: &str, flags: i32) -> Result<Self> {
        let path = path.as_bytes();
        match unsafe { _cunistd::open(path.as_ptr(), flags) } {
            -1 => Err(Error::new(ErrorKind::Other, "open failed")),
            fd => Ok(Self { fd }),
        }
    }

    pub fn new(fd: i32) -> Self {
        Self { fd }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { _cunistd::close(self.fd) };
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match unsafe { _cunistd::read(self.fd, buf.as_mut_ptr(), buf.len() as i32) } {
            -1 => Err(Error::new(ErrorKind::Other, "read failed")),
            n => Ok(n as usize),
        }
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match unsafe { _cunistd::write(self.fd, buf.as_ptr(), buf.len() as i32) } {
            -1 => Err(Error::new(ErrorKind::Other, "write failed")),
            n => Ok(n as usize),
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
