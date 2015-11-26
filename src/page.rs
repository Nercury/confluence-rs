use chrono::{ DateTime, UTC };

/// Page.
#[derive(Debug)]
pub struct Page {
    /// The id of the page
    pub id: i64,
    /// The key of the space that this page belongs to
    pub space: String,
	/// The id of the parent page
	pub parent_id: i64,
    /// The title of the page
	pub title: String,
	/// The url to view this page online
    pub url: String,
    /// The version number of this page
    pub version: i32,
	/// The page content
    pub content: String,
    /// Timestamp page was created
    pub created: DateTime<UTC>,
    /// Username of the creator
    pub creator: String,
    /// Timestamp page was modified
    pub modified: DateTime<UTC>,
    /// Username of the page's last modifier
    pub modifier: String,
    /// Whether or not this page is the space's homepage
    pub home_page: bool,
    /// Status of the page (eg. current or deleted)
    pub content_status: String,
    /// Whether the page is current and not deleted
    pub current: bool,
}

/// Page Summary.
#[derive(Debug)]
pub struct PageSummary {
    /// The id of the page
    pub id: i64,
    /// The key of the space that this page belongs to
    pub space: String,
	/// The id of the parent page
	pub parent_id: i64,
    /// The title of the page
	pub title: String,
	/// The url to view this page online
    pub url: String,
}
