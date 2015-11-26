use xmltree::Element;
use rpser::xml::BuildElement;

use {
    Result,
    Space,
    Page,
    PageSummary,
};

pub trait FromElement {
    fn from_element(element: Element) -> Result<Self> where Self: Sized;
}

impl FromElement for Space {
    fn from_element(element: Element) -> Result<Space> {
        Ok(Space {
            description: try!(element.get_at_path(&["description"])).text,
            home_page: try!(element.get_at_path(&["homePage"]).and_then(|e| e.as_long())),
            key: try!(element.get_at_path(&["key"]).and_then(|e| e.as_string())),
            name: try!(element.get_at_path(&["name"]).and_then(|e| e.as_string())),
            space_group: try!(element.get_at_path(&["name"])).text,
            space_type: try!(element.get_at_path(&["type"]).and_then(|e| e.as_string())),
            url: try!(element.get_at_path(&["url"]).and_then(|e| e.as_string())),
        })
    }
}

impl FromElement for Page {
    fn from_element(element: Element) -> Result<Page> {
        Ok(Page {
            id: try!(element.get_at_path(&["id"]).and_then(|e| e.as_long())),
            space: try!(element.get_at_path(&["space"]).and_then(|e| e.as_string())),
            parent_id: try!(element.get_at_path(&["parentId"]).and_then(|e| e.as_long())),
            title: try!(element.get_at_path(&["title"]).and_then(|e| e.as_string())),
            url: try!(element.get_at_path(&["url"]).and_then(|e| e.as_string())),
            version: try!(element.get_at_path(&["version"]).and_then(|e| e.as_int())),
            content: try!(element.get_at_path(&["content"]).and_then(|e| e.as_string())),
            created: try!(element.get_at_path(&["created"]).and_then(|e| e.as_datetime())),
            creator: try!(element.get_at_path(&["creator"]).and_then(|e| e.as_string())),
            modified: try!(element.get_at_path(&["modified"]).and_then(|e| e.as_datetime())),
            modifier: try!(element.get_at_path(&["modifier"]).and_then(|e| e.as_string())),
            home_page: try!(element.get_at_path(&["homePage"]).and_then(|e| e.as_boolean())),
            content_status: try!(element.get_at_path(&["contentStatus"]).and_then(|e| e.as_string())),
            current: try!(element.get_at_path(&["current"]).and_then(|e| e.as_boolean())),
        })
    }
}

impl FromElement for PageSummary {
    fn from_element(element: Element) -> Result<PageSummary> {
        Ok(PageSummary {
            id: try!(element.get_at_path(&["id"]).and_then(|e| e.as_long())),
            space: try!(element.get_at_path(&["space"]).and_then(|e| e.as_string())),
            parent_id: try!(element.get_at_path(&["parentId"]).and_then(|e| e.as_long())),
            title: try!(element.get_at_path(&["title"]).and_then(|e| e.as_string())),
            url: try!(element.get_at_path(&["url"]).and_then(|e| e.as_string())),
        })
    }
}
