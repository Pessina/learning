use async_std::io::{Read, Write};

use futures::task::Poll;

use std::cmp::min;

use std::marker::Unpin;

pub struct MockTcpStream {
    pub read_data: Vec<u8>,
    pub write_data: Vec<u8>,
}

impl Read for MockTcpStream {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        let size: usize = min(self.read_data.len(), buf.len());
        buf[..size].copy_from_slice(&self.read_data[..size]);
        Poll::Ready(Ok(size))
    }
}

impl Write for MockTcpStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        self.write_data = Vec::from(buf);

        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

impl Unpin for MockTcpStream {}
