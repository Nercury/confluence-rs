/// Space.
#[derive(Debug)]
pub struct Space {
    /// The space key
    pub key: String,
    /// The name of the space
    pub name: String,
    /// Space group
    pub space_group: Option<String>,
    /// Space type
    pub space_type: String,
    /// The url to view this space online
    pub url: String,
    /// The id of the space homepage
    pub home_page: i64,
    /// The HTML rendered space description
    pub description: Option<String>,
}
