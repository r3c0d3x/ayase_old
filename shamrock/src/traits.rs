use either::Either;
use futures::{Future, Stream};
use futures::stream::BoxStream;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::any::Any;
use std::error::Error;
use std::marker::PhantomData;
use std::sync::Mutex;
use typemap::{Key, SendMap};

use constructs::*;

/// If this trait is implemented, the layout can be used for the Store because the Layout's format can converted to the
/// one used by the Store.
///
/// We chose to have the trait work with Stores and Layouts directly because, while multiple Layouts can use the same
/// format, they might be best represented in the Store in different ways. For example, Asagi and Ayase (the layout)
/// are both based on JSON, but are represented completely differently in Stores.
///
/// XXX: The first sentence in the second paragaph sounds weird.
pub trait Storable<S: Store>: Layout {
    /// Convert a value in our own format to one compatible with a Store's.
    fn convert(Self::Format) -> S::Format;
}

pub trait Identifiable {
    /// Returns a unique, human-friendly, static identifier for the type. It should not be based on generics.
    fn id() -> &'static str
        where Self: Sized;
}

/// A place to store data of a certain Format.
pub trait Store: Identifiable {
    type Config: DeserializeOwned;
    /// The type of error that can occur when trying to store data.
    type Error: Error;
    /// The core data type that the Store uses. This isn't too important as long as Layouts implement Storable for this Store.
    type Format: Format;

    /// Setup the Store.
    ///
    /// The deserializable value recieved is the config that it's given. 
    ///
    /// XXX: Should this be asynchronous (using futures)?
    fn new(Self::Config) -> Box<Future<Item = Self, Error = Self::Error>>;

    fn store<L: Storable<Self>, C: Construct<L>>(&self, _: C) -> Box<Future<Item = (), Error = Self::Error>>
        where Self: Sized;
}

/// The way that the raw memory is encoded in the associated Raw type.
///
/// # Implementing
///
/// This is meant to be implemented by a newtype struct.
///
/// # Examples
///
/// - JSON
///
/// XXX: Fix this explanation. Talk about how Format : Layout :: Layout : Store.
pub trait Format {
    //type Error: Error;
    type Value;

    /// Converts the type into it's raw type.
    fn as_value(self) -> Self::Value;
}

/// A way of organizing the data contained in a Store.
///
/// # Implementing
///
/// This is meant to be implemented by a unit struct.
///
/// # Examples
///
/// - A DB schema
pub trait Layout: Identifiable {
    /// The type of error that can occur from not being able to correctly conform data to the layout.
    type Error: Error;

    /// The Format that data is stored in when using this Layout.
    type Format: Format;
}

/// A kind of thing that we would store data for.
///
/// # Implementing
///
/// This is meant to be implemented by any struct able to contain the Format specified by the Layout.
///
/// # Examples
///
/// - Post
/// - Thread
/// - File
pub trait Construct<L: Layout>: Identifiable {
    fn new(L::Format) -> Self
        where Self: Sized;
    fn into_format(self) -> L::Format;
}

pub trait Source: Identifiable {
    type Config: DeserializeOwned;
    type Error: Error;
    //type Item: Construct<Self::Layout>;0
    type Layout: Layout;
    //type Stream: Stream<Item = Self::Item, Error = Self::Error>;

    //fn new(Self::Config) -> Box<Future<Item = Self, Error = Self::Error>>;
    fn stream<C: Any + Construct<Self::Layout>>(&mut self) -> Option<BoxStream<C, Self::Error>> where Self: Sized;
}

//pub struct StreamMap();
//
//pub struct SourceStreamKey<S: Source, C: Any + Construct<S::Layout>>(C, PhantomData<S>);
//
//impl<S: Source, C: Any + Construct<S::Layout>> From<C> for SourceStreamKey<S, C> {
//    fn from(val: C) -> SourceStreamKey<S, C> {
//        SourceStreamKey(val, PhantomData)
//    }
//}
//
//impl<S: 'static + Source, C: Any + Construct<S::Layout>> Key for SourceStreamKey<S, C>
//    where S::Error: 'static
//{
//    type Value = BoxStream<C, BoxStream<C, S::Error>>;
//}

//impl<E: Error, L: Layout, C: Any + Construct<L>> Key for Box<C> {
//    type Value = BoxStream<Self, E>;
//}