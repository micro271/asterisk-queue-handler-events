use std::alloc::{Layout, alloc};

use super::bytes::BytesHandler;
use tokio::io::AsyncWrite;

pub struct BufWriter<T> {
    inner: T,
    buffer: BytesHandler,
}

impl<T: AsyncWrite> BufWriter<T> {
    fn new(inner: T) -> Self {
        Self {
            inner,
            buffer: BytesHandler::new(),
        }
    }
}

impl<T: AsyncWrite> AsyncWrite for BufWriter<T> {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        todo!()
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        todo!()
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        todo!()
    }
}
