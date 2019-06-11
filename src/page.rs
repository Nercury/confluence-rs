use chrono::offset::Utc;
use chrono::DateTime;

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
    pub created: DateTime<Utc>,
    /// Username of the creator
    pub creator: String,
    /// Timestamp page was modified
    pub modified: DateTime<Utc>,
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

/// Page Object for creating a Page.
#[derive(Debug)]
pub struct UpdatePage {
    /// The id of the page
    pub id: Option<i64>,
    /// The key of the space that this page belongs to
    pub space: String,
    /// The title of the page
    pub title: String,
    /// The page content
    pub content: String,
    /// The version number of this page
    pub version: Option<i32>,
    /// The id of the parent page
    pub parent_id: Option<i64>,
}

/// Options for updating the page.
#[derive(Debug)]
pub struct PageUpdateOptions {
    // Edit comment for the updated page
    pub version_comment: Option<String>,
    // Is this update a 'minor edit'? (default value: false)
    pub minor_edit: bool,
}

impl PageUpdateOptions {
    pub fn new_minor() -> PageUpdateOptions {
        PageUpdateOptions {
            version_comment: None,
            minor_edit: true,
        }
    }

    pub fn new_minor_with_comment<S: Into<String>>(comment: S) -> PageUpdateOptions {
        PageUpdateOptions {
            version_comment: Some(comment.into()),
            minor_edit: true,
        }
    }
}

impl UpdatePage {
    pub fn with_create_fields<S: Into<String>>(
        parent_id: Option<i64>,
        space: &str,
        title: &str,
        content: S,
    ) -> UpdatePage {
        UpdatePage {
            id: None,
            space: space.into(),
            title: title.into(),
            content: content.into(),
            version: None,
            parent_id: parent_id,
        }
    }
}

impl From<Page> for UpdatePage {
    fn from(other: Page) -> UpdatePage {
        UpdatePage {
            id: Some(other.id),
            space: other.space,
            title: other.title,
            content: other.content,
            version: Some(other.version),
            parent_id: if other.parent_id == 0 {
                None
            } else {
                Some(other.parent_id)
            },
        }
    }
}
