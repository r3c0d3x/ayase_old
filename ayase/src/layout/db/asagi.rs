//!Adapted from [Asagi's boards.sql](https://github.com/eksopl/asagi/blob/master/src/main/resources/net/easymodo/asagi/sql/Pgsql/boards.sql)

#[derive(Debug)]
pub struct board {
    pub doc_id: u32,
    pub media_id: u32,
    pub poster_ip: u64,
    pub num: u32,
    /// Used for ghost posts.
    pub subnum: u32,
    pub thread_num: u32,
    pub op: bool,
    pub timestamp: u32,
    pub timestamp_expired: u32,
    pub preview_orig: Option<String>,
    pub preview_w: u16,
    pub preview_h: u16,
    pub media_filename: Option<String>,
    pub media_w: u16,
    pub media_h: u16,
    pub media_size: u32,
    pub media_hash: Option<String>,
    pub media_orig: Option<String>,
    pub spoiler: bool,
    pub deleted: bool,
    pub capcode: char,
    pub email: Option<String>,
    pub name: Option<String>,
    pub trip: Option<String>,
    pub title: Option<String>,
    pub comment: Option<String>,
    pub delpass: Option<String>,
    pub sticky: bool,
    pub locked: bool,
    pub poster_hash: Option<String>,
    pub poster_country: Option<String>,
    pub exif: Option<String>,
}

#[derive(Debug)]
pub struct board_deleted {
    pub doc_id: u32,
    pub media_id: u32,
    pub poster_ip: u64,
    pub num: u32,
    pub subnum: u32,
    pub thread_num: u32,
    pub op: bool,
    pub timestamp: u32,
    pub timestamp_expired: u32,
    pub preview_orig: Option<String>,
    pub preview_w: u16,
    pub preview_h: u16,
    pub media_filename: Option<String>,
    pub media_w: u16,
    pub media_h: u16,
    pub media_size: u32,
    pub media_hash: Option<String>,
    pub media_orig: Option<String>,
    pub spoiler: bool,
    pub deleted: bool,
    pub capcode: char,
    pub email: Option<String>,
    pub name: Option<String>,
    pub trip: Option<String>,
    pub title: Option<String>,
    pub comment: Option<String>,
    pub delpass: Option<String>,
    pub sticky: bool,
    pub locked: bool,
    pub poster_hash: Option<String>,
    pub poster_country: Option<String>,
    pub exif: Option<String>,
}

#[derive(Debug)]
pub struct board_threads {
    pub thread_num: u32,
    pub time_op: u32,
    pub time_last: u32,
    pub time_bump: u32,
    pub time_ghost: u32,
    pub time_ghost_bump: u32,
    pub time_last_modified: u32,
    pub nreplies: u32,
    pub nimages: u32,
    pub sticky: bool,
    pub locked: bool,
}

#[derive(Debug)]
pub struct board_users {
    pub user_id: u32,
    pub name: String,
    pub trip: String,
    pub firstseen: i64,
    pub postcount: i64,
}

#[derive(Debug)]
pub struct board_images {
    pub media_id: u32,
    pub media_hash: String,
    pub media: Option<String>,
    pub preview_op: Option<String>,
    pub preview_reply: Option<String>,
    pub total: u64,
    pub banned: u16,
}

#[derive(Debug)]
pub struct board_daily {
    pub day: u64,
    pub posts: u64,
    pub images: u64,
    pub sage: u64,
    pub anons: u64,
    pub trips: u64,
    pub names: u64,
}