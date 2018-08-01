pub trait Post {
    fn no(&self) -> u64;
}

pub trait Thread {
    type Chan: Chan;

    type Error: Error;
    type Post: Post;
    type Raw: Serialize;
    
    fn new<T: DeserializeOwned + Serialize>(T) -> Result<Self, Self::Chan::Error>;
    fn raw(self) -> Self::Raw;
    fn board(&self) -> &str;
}

pub trait Board {
    fn board(&self) -> &str;
    fn per_page(&self) -> u8;
    fn pages(&self) -> u8;
}

pub trait Chan {
    type Board: Board;
    type Error: Error;
    type Thread: Thread;

    fn site() -> &str;
    fn boards() -> Vec<Self::Board>;
    fn threads(&str) -> Vec<Self::Thread>;
    fn thread(&str, u64) -> Self::Thread;
}