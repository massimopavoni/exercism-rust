use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    data: R,
    bytes: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            data: wrapped,
            bytes: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.data.read(buf).inspect(|&br| {
            self.bytes += br;
            self.reads += 1;
        })
    }
}

pub struct WriteStats<W> {
    data: W,
    bytes: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            data: wrapped,
            bytes: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.data.write(buf).inspect(|&bw| {
            self.bytes += bw;
            self.writes += 1;
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
