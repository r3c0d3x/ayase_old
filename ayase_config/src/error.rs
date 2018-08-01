error_chain! {
    foreign_links {
        Io(::std::io::Error) #[doc = "For IO-related errors."];
        Json(::serde_json::Error) #[doc = "For errors from parsing JSON."];
    }
}