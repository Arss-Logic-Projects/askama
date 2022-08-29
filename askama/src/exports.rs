use futures_util::{ready, TryStream};
use pin_project_lite::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub use {async_stream::try_stream, bytestring::ByteString, futures_util::stream::LocalBoxStream};

pub fn assert_is_template(_: &impl crate::Template) {}

pub fn try_next<St: TryStream>(stream: &mut St) -> TryNext<'_, St> {
    TryNext { stream }
}

pin_project! {
    pub struct TryNext<'a, St: TryStream> {
        #[pin]
        stream: &'a mut St
    }
}

impl<'a, St: TryStream> Future for TryNext<'a, St> {
    type Output = Result<Option<St::Ok>, St::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        // SAFETY: stream is guarantied to be pinned because self is
        let stream = unsafe { this.stream.map_unchecked_mut(|s| *s) };

        Poll::Ready(ready!(stream.try_poll_next(cx)).transpose())
    }
}
