// Iteration 2 inspired by fdumontmd's solution (flush underlying writer on flusn())
use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    bytes_read: usize,
    call_count: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> Self {
        Self {
            reader: wrapped,
            bytes_read: 0,
            call_count: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.call_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let res = self.reader.read(buf);
        if let Ok(n) = res {
            self.bytes_read += n;
        }
        self.call_count += 1;
        res
    }
}

pub struct WriteStats<W> {
    writer: W,
    bytes_written: usize,
    call_count: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> Self {
        Self {
            writer: wrapped,
            bytes_written: 0,
            call_count: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    pub fn writes(&self) -> usize {
        self.call_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let res = self.writer.write(buf);
        if let Ok(n) = res {
            self.bytes_written += n;
        }
        self.call_count += 1;
        res
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
