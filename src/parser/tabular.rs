use std::io;
use std::io::{BufRead, Read};

pub struct LineReader<R: Read> {
    inner: R,
    buf: Vec<u8>,
}

impl<R: BufRead> LineReader<R> {
    pub(crate) fn new(inner: R) -> Self {
        Self {
            inner,
            buf: Vec::new(),
        }
    }

    pub(crate) fn read_line_self(&mut self) -> io::Result<Option<String>> {
        let mut line = Vec::new();
        let bytes_read = self.inner.read_until(b'\n', &mut line)?;

        if bytes_read == 0 {
            if !self.buf.is_empty() {
                let cloned_buf = self.buf.clone();
                self.buf.clear();
                let cloned_string = String::from_utf8_lossy(&cloned_buf);
                return Ok(Some(cloned_string.into_owned()));
            }
            return Ok(None);
        }

        if line.last() == Some(&b'\r') {
            line.pop();
        }

        self.buf.extend(line.iter());

        let cloned_buf = self.buf.clone();
        self.buf.clear();
        let cloned_string = String::from_utf8_lossy(&cloned_buf);
        Ok(Some(cloned_string.into_owned()))
    }
}

impl<R: Read> Read for LineReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: BufRead> BufRead for LineReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Ok(&self.buf)
    }

    fn consume(&mut self, amt: usize) {
        self.buf.drain(..amt);
    }

    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        buf.clear();
        match self.read_line_self() {
            Ok(Some(line)) => {
                buf.push_str(&line);
                Ok(line.len())
            }
            Ok(None) => Ok(0),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_line_self() {
        let input = "Hello, world!\nWelcome to Rust.\n";
        let mut reader = LineReader::new(input.as_bytes());
        assert_eq!(
            reader.read_line_self().unwrap(),
            Some("Hello, world!\n".to_owned())
        );
        assert_eq!(
            reader.read_line_self().unwrap(),
            Some("Welcome to Rust.\n".to_owned())
        );
        assert_eq!(reader.read_line_self().unwrap(), None);
    }

    #[test]
    fn test_read_line() {
        let input = "Hello, world!\nWelcome to Rust.\n";
        let mut reader = LineReader::new(input.as_bytes());
        let mut buffer = String::new();
        assert_eq!(reader.read_line(&mut buffer).unwrap(), 14);
        assert_eq!(buffer, "Hello, world!\n");
        buffer.clear();
        assert_eq!(reader.read_line(&mut buffer).unwrap(), 17);
        assert_eq!(buffer, "Welcome to Rust.\n");
        buffer.clear();
        assert_eq!(reader.read_line(&mut buffer).unwrap(), 0);
    }
}
