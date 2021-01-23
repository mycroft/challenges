use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    bytes: usize,
    ops: usize,
    wrapped: R,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        Self { bytes: 0, ops: 0, wrapped: _wrapped }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.ops
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.ops += 1;

        let _ret = self.wrapped.read(buf);

        if let Ok(bt) = _ret {
            self.bytes += bt;
        }

        _ret
    }
}

pub struct WriteStats<W> {
    bytes: usize,
    ops: usize,
    wrapped: W
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        Self { bytes: 0, ops: 0, wrapped: _wrapped }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.ops
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.ops += 1;

        let _ret = self.wrapped.write(buf);

        if let Ok(bt) = _ret {
            self.bytes += bt;
        }

        _ret
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
