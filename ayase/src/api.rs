use error::Error;
use futures::{Future, Stream};
use hyper::{Chunk, Client};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct Board {
    pub board: String,
    pub per_page: u8,
    pub pages: u8,
}

#[allow(missing_docs)]
pub mod types {
    #[derive(Debug, Deserialize)]
    pub struct Board {
        pub board: String,
        pub per_page: u8,
        pub pages: u8,
    }
    
    #[derive(Debug, Deserialize)]
    pub struct Boards {
        pub boards: Vec<Board>,
    }
    
    #[derive(Debug, Deserialize)]
    pub struct Thread {
        pub no: u64,
        pub last_modified: u64,
    }
    
    #[derive(Debug, Deserialize)]
    pub struct Page {
        pub page: u8,
        pub threads: Vec<Thread>,
    }

    /*

    pub fn diff_threads(old: &Threads, new: &Threads) -> Vec<u64> {

    }

    */
    
    pub type Threads = Vec<Page>;
}

use self::types::*;

fn get<T: 'static + DeserializeOwned>(url: &str, client: &Client<HttpsConnector<HttpConnector>>) -> Box<Future<Item = T, Error = Error>> {
    Box::new(
        client
            .get(url.parse().unwrap())
            .map_err(Error::from)
            .and_then(|res| {
                res.body().concat2().map_err(Error::from).and_then(move |body: Chunk| {
                    serde_json::from_slice(&body).map_err(Error::from)
                })
            })
    )
}

/// Get boards
pub fn boards(client: &Client<HttpsConnector<HttpConnector>>) -> Box<Future<Item = types::Boards, Error = Error>> {
    get("https://a.4cdn.org/boards.json", client)
}

/// Get threads from a board
pub fn threads(board: &str, client: &Client<HttpsConnector<HttpConnector>>) -> Box<Future<Item = types::Threads, Error = Error>> {
    get(format!("https://a.4cdn.org/{}/threads.json", board).as_ref(), client)
}

/// Get thread from a board
pub fn thread(board: &str, no: u64, client: &Client<HttpsConnector<HttpConnector>>) -> Box<Future<Item = types::Thread, Error = Error>> {
    get(format!("https://a.4cdn.org/{}/thread/{}.json", board, no).as_ref(), client)
}