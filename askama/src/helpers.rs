use std::pin::Pin;
use std::task::{Context, Poll};
use futures_util::stream::*;
use futures_util::ready;

pin_project_lite::pin_project! {
    pub struct AsyncTemplateLoop<St: Sized> {
        #[pin]
        stream: Peekable<Enumerate<St>>
    }
}

impl<St: Stream + Sized> AsyncTemplateLoop<I> {
    pub fn new(stream: I) -> Self {
        AsyncTemplateLoop { stream: stream.enumerate().peekable() }
    }
}

impl<St: Stream + Sized> Stream for AsyncTemplateLoop<St> {
    type Item = (St::Item, LoopItem);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        let is_terminated = ready!(this.stream.as_mut().poll_peek(cx)).is_none();

        match ready!(this.stream.poll_next(cx)) {
            Some((index, item)) => Poll::Ready(Some((item, LoopItem {
                index,
                first: index == 0,
                last: is_terminated
            }))),
            None => Poll::Ready(None)
        }
    }
}

use std::iter::{Enumerate as StdEnumerate, Peekable as StdPeekable};

pub struct TemplateLoop<I>
where
    I: Iterator,
{
    iter: StdPeekable<StdEnumerate<I>>,
}

impl<I> TemplateLoop<I>
where
    I: Iterator,
{
    #[inline]
    pub fn new(iter: I) -> Self {
        TemplateLoop {
            iter: iter.enumerate().peekable(),
        }
    }
}

impl<I> Iterator for TemplateLoop<I>
where
    I: Iterator,
{
    type Item = (<I as Iterator>::Item, LoopItem);

    #[inline]
    fn next(&mut self) -> Option<(<I as Iterator>::Item, LoopItem)> {
        self.iter.next().map(|(index, item)| {
            (
                item,
                LoopItem {
                    index,
                    first: index == 0,
                    last: self.iter.peek().is_none(),
                },
            )
        })
    }
}

#[derive(Copy, Clone)]
pub struct LoopItem {
    pub index: usize,
    pub first: bool,
    pub last: bool,
}
