use rpser::xml::BuildElement;
use xmltree::Element;

use {Page, PageSummary, Result, Space};

pub trait FromElement {
    fn from_element(element: Element) -> Result<Self>
    where
        Self: Sized;
}

impl FromElement for Space {
    fn from_element(element: Element) -> Result<Space> {
        Ok(Space {
            description: element.get_at_path(&["description"])?.text,
            home_page: element.get_at_path(&["homePage"]).and_then(|e| e.as_long())?,
            key: element.get_at_path(&["key"]).and_then(|e| e.as_string())?,
            name: element.get_at_path(&["name"]).and_then(|e| e.as_string())?,
            space_group:element.get_at_path(&["name"])?.text,
            space_type: element.get_at_path(&["type"]).and_then(|e| e.as_string())?,
            url: element.get_at_path(&["url"]).and_then(|e| e.as_string())?,
        })
    }
}

impl FromElement for Page {
    fn from_element(element: Element) -> Result<Page> {
        Ok(Page {
            id: element.get_at_path(&["id"]).and_then(|e| e.as_long())?,
            space: element.get_at_path(&["space"]).and_then(|e| e.as_string())?,
            parent_id: element.get_at_path(&["parentId"]).and_then(|e| e.as_long())?,
            title: element.get_at_path(&["title"]).and_then(|e| e.as_string())?,
            url: element.get_at_path(&["url"]).and_then(|e| e.as_string())?,
            version: element.get_at_path(&["version"]).and_then(|e| e.as_int())?,
            content: element
                .get_at_path(&["content"])
                .and_then(|e| e.as_string())?,
            created: element
                .get_at_path(&["created"])
                .and_then(|e| e.as_datetime())?,
            creator: element
                .get_at_path(&["creator"])
                .and_then(|e| e.as_string())?,
            modified: element
                .get_at_path(&["modified"])
                .and_then(|e| e.as_datetime())?,
            modifier: element
                .get_at_path(&["modifier"])
                .and_then(|e| e.as_string())?,
            home_page: element
                .get_at_path(&["homePage"])
                .and_then(|e| e.as_boolean())?,
            content_status: element
                .get_at_path(&["contentStatus"])
                .and_then(|e| e.as_string())?,
            current: element
                .get_at_path(&["current"])
                .and_then(|e| e.as_boolean())?
        })
    }
}

impl FromElement for PageSummary {
    fn from_element(element: Element) -> Result<PageSummary> {
        Ok(PageSummary {
            id: element.get_at_path(&["id"]).and_then(|e| e.as_long())?,
            space: element.get_at_path(&["space"]).and_then(|e| e.as_string())?,
            parent_id: element.get_at_path(&["parentId"]).and_then(|e| e.as_long())?,
            title: element.get_at_path(&["title"]).and_then(|e| e.as_string())?,
            url: element.get_at_path(&["url"]).and_then(|e| e.as_string())?,
        })
    }
}
