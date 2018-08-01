//! The Ayase archival architecture has 4 key components:
//!
//! - [Construct](traits/trait.Construct.html)
//!   - A construct is the ideal version of something.
//!   - It's a generic trait, representing a kind of thing (with any layout) that we would want to store in a Store.
//! - [Format](traits/trait.Format.html)
//!   - A format is the type that stores things that are arranged.
//!   - It's a trait for the type that a Layout uses stores its data.
//!   - Two different layouts or stores can use the same format, but in incompatible ways.
//! - [Layout](traits/trait.Layout.html)
//!   - A layout is a way of arranging something.
//!   - It's a trait that arranges things in a certain Format.
//! - [Source](traits/trait.Source.html)
//!   - A source is a place where the data enters the system.
//! - [Store](traits/trait.Store.html)
//!   - Represents a [data store](https://en.wikipedia.org/wiki/Data_store).
//!   - If a Layout can convert an instance of its Format to the Format used by a Store, that Layout is said to be
//!     [Storable](../shamrock/traits/trait.Storable.html). If a Layout is Storable, then we can store things organized in that Layout in the Store.

#![warn(missing_docs)]

extern crate either;
extern crate futures;
extern crate serde;
extern crate typemap;

#[cfg(test)] extern crate serde_value;

#[cfg(test)] mod test;

mod traits;

pub use traits::*;