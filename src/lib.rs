use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    underlying: R,
    bytes_read: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            underlying: wrapped,
            bytes_read: 0_usize,
            reads: 0_usize,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.underlying
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self.underlying.read(buf)?;
        self.bytes_read += bytes_read;
        self.reads += 1;
        Ok(bytes_read)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        let mut internal: [u8; 8192] = [0; 8192];

        let initial_buf_len = buf.len();
        loop {
            match self.read(&mut internal) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    }
                    buf.extend(&internal[0..bytes_read]);
                }
                Err(e) => match e.kind() {
                    std::io::ErrorKind::Interrupted => continue,
                    _ => return Err(e),
                },
            }
        }
        Ok(buf.len() - initial_buf_len)
    }
}

pub struct WriteStats<W> {
    underlying: W,
    bytes_through: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            underlying: wrapped,
            bytes_through: 0_usize,
            writes: 0_usize,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.underlying
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.underlying.write(buf)?;
        self.bytes_through += bytes_written;
        self.writes += 1;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
