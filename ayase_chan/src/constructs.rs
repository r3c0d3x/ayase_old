use std::marker::PhantomData;

use traits::*;

pub struct Thread<L: Layout>(pub L::Format);

impl<L: Layout> Identifiable for Thread<L> {
    fn id() -> &'static str {
        "thread"
    }
}

impl<L: Layout> Construct<L> for Thread<L> {
    fn new(val: L::Format) -> Self {
        Thread(val)
    }

    fn into_format(self) -> L::Format {
        self.0
    }
}

pub struct Post<L: Layout>(pub L::Format);

impl<L: Layout> Identifiable for Post<L> {
    fn id() -> &'static str {
        "post"
    }
}

impl<L: Layout> Construct<L> for Post<L> {
    fn new(val: L::Format) -> Self {
        Post(val)
    }

    fn into_format(self) -> L::Format {
        self.0
    }
}

pub struct File<L: Layout>(pub L::Format);

impl<L: Layout> Identifiable for File<L> {
    fn id() -> &'static str {
        "file"
    }
}

impl<L: Layout> Construct<L> for File<L> {
    fn new(val: L::Format) -> Self {
        File(val)
    }

    fn into_format(self) -> L::Format {
        self.0
    }
}