use futures_util::ready;
use futures_util::stream::*;
use std::pin::Pin;
use std::task::{Context, Poll};

pub use futures_util::stream::Filter;

pub struct AsyncTemplateLoop<St: Stream> {
    stream: Peekable<Enumerate<St>>,
}


impl<St: Stream> AsyncTemplateLoop<St> {
    pub fn new(stream: St) -> Self {
        AsyncTemplateLoop {
            stream: stream.enumerate().peekable(),
        }
    }
}

impl<St: Stream + Unpin> Stream for AsyncTemplateLoop<St> {
    type Item = (St::Item, LoopItem);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let is_terminated = ready!(Pin::new(&mut self.stream).poll_peek(cx)).is_none();

        self.stream.poll_next_unpin(cx).map(|opt| {
            opt.map(|(index, item)| {
                (
                    item,
                    LoopItem {
                        index,
                        first: index == 0,
                        last: is_terminated,
                    },
                )
            })
        })
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
