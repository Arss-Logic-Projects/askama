use futures_util::{TryStream, TryStreamExt};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub use {async_stream::try_stream, bytestring::ByteString, futures_util::stream::BoxStream};

pub fn assert_is_template(_: &impl crate::Template) {}

pub fn try_next<St: TryStream + Unpin>(stream: &mut St) -> TryNext<'_, St> {
    TryNext { stream }
}

pub struct TryNext<'a, St> {
    stream: &'a mut St,
}

impl<'a, St: TryStream + Unpin> Future for TryNext<'a, St> {
    type Output = Result<Option<St::Ok>, St::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.stream
            .try_poll_next_unpin(cx)
            .map(Option::transpose)
    }
}
