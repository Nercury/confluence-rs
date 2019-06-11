/*!
<a href="https://github.com/Nercury/confluence-rs">
    <img style="position: absolute; top: 0; left: 0; border: 0;" src="https://s3.amazonaws.com/github/ribbons/forkme_left_green_007200.png" alt="Fork me on GitHub">
</a>
<style>.sidebar { margin-top: 53px }</style>
*/

/*!
Access and modify [Atlassian Confluence](https://www.atlassian.com/software/confluence/) pages from Rust.

## Working with this library

To start, create a new `Session` by calling a `login` on it
with your credentials.

Internally, the `Session` struct stores the auth `token`
and uses it when calling remote methods.

The token will be destroyed (automatic logout) when `Session` goes out of scope.
*/

#[macro_use]
extern crate log;
extern crate chrono;
extern crate reqwest;
extern crate xml;
extern crate xmltree;

pub mod http;
pub mod rpser;
pub mod wsdl;

mod page;
mod space;
mod transforms;

pub use page::{Page, PageSummary, PageUpdateOptions, UpdatePage};
pub use space::Space;
pub use transforms::FromElement;

use std::io::Error as IoError;
use std::result;

use self::http::HttpError;
use self::rpser::xml::BuildElement;
use self::rpser::{Method, RpcError};
use xmltree::Element;

const V2_API_RPC_PATH: &'static str = "/rpc/soap-axis/confluenceservice-v2?wsdl";

/// Client's session.
pub struct Session {
    wsdl: wsdl::Wsdl,
    token: String,
}

impl Drop for Session {
    fn drop(&mut self) {
        self.logout().unwrap();
    }
}

impl Session {
    /**
    Create new confluence session.

    ## Example

    ```no_run
    let session = confluence::Session::login(
        "https://confluence",
        "user",
        "pass"
    ).unwrap();
    ```
    */
    pub fn login(url: &str, user: &str, pass: &str) -> Result<Session> {
        debug!("logging in at url {:?} with user {:?}", url, user);

        let url = if url.ends_with("/") {
            &url[..url.len() - 1]
        } else {
            url
        };
        let wsdl_url = [url, V2_API_RPC_PATH].concat();

        debug!("getting wsdl from url {:?}", wsdl_url);

        let wsdl = try!(wsdl::fetch(&wsdl_url));
        let mut session = Session {
            wsdl: wsdl,
            token: String::new(),
        };

        let response = try!(session.call(
            Method::new("login")
                .with(Element::node("username").with_text(user))
                .with(Element::node("password").with_text(pass))
        ));

        let token = match try!(response.body.descend(&["loginReturn"])).text {
            Some(token) => token,
            _ => return Err(Error::ReceivedNoLoginToken),
        };

        session.token = token;

        Ok(session)
    }

    /// Explicitly log out out of confluence.
    ///
    /// This is done automatically at the end of Session's lifetime.
    pub fn logout(&self) -> Result<bool> {
        let response = try!(self.call(
            Method::new("logout").with(Element::node("token").with_text(self.token.clone()))
        ));

        Ok(match try!(response.body.descend(&["logoutReturn"])).text {
            Some(ref v) if v == "true" => {
                debug!("logged out successfully");
                true
            }
            _ => {
                debug!("log out failed (maybe expired token, maybe not loged in)");
                false
            }
        })
    }

    /**
    Returns a single Space.

    If the spaceKey does not exist: earlier versions of Confluence will throw an Exception. Later versions (3.0+) will return a null object.

    In this client the difference will be in error type.

    ## Example

    ```no_run
    # let session = confluence::Session::login("https://confluence", "user", "pass").unwrap();
    println!("Space: {:#?}",
        session.get_space(
            "SomeSpaceKey"
        )
    );
    ```
    */
    pub fn get_space(&self, space_key: &str) -> Result<Space> {
        let response = try!(self.call(
            Method::new("getSpace")
                .with(Element::node("token").with_text(self.token.clone()))
                .with(Element::node("spaceKey").with_text(space_key))
        ));

        let element = try!(response.body.descend(&["getSpaceReturn"]));

        Ok(try!(Space::from_element(element)))
    }

    /**
    Returns a single Page by space and title.

    ## Example

    ```no_run
    # let session = confluence::Session::login("https://confluence", "user", "pass").unwrap();
    println!("Page: {:#?}",
        session.get_page_by_title(
            "SomeSpaceKey", "Page Title"
        )
    );
    ```
    */
    pub fn get_page_by_title(&self, space_key: &str, page_title: &str) -> Result<Page> {
        let response = try!(self.call(
            Method::new("getPage")
                .with(Element::node("token").with_text(self.token.clone()))
                .with(Element::node("spaceKey").with_text(space_key))
                .with(Element::node("pageTitle").with_text(page_title))
        ));

        let element = try!(response.body.descend(&["getPageReturn"]));

        Ok(try!(Page::from_element(element)))
    }

    /**
    Returns a single Page by id.

    ## Example

    ```no_run
    # let session = confluence::Session::login("https://confluence", "user", "pass").unwrap();
    println!("Page: {:#?}",
        session.get_page_by_id(
            123456
        )
    );
    ```
    */
    pub fn get_page_by_id(&self, page_id: i64) -> Result<Page> {
        let response = try!(self.call(
            Method::new("getPage")
                .with(Element::node("token").with_text(self.token.clone()))
                .with(Element::node("pageId").with_text(page_id.to_string()))
        ));

        let element = try!(response.body.descend(&["getPageReturn"]));

        Ok(try!(Page::from_element(element)))
    }

    /**
    Adds or updates a page.

    # For adding

    The Page given as an argument should have:

    - (optional) parent_id
    - space
    - title
    - content

    fields at a minimum.

    Use helper `UpdatePage::with_create_fields` to create such page.

    ## Example

    ```no_run
    use confluence::UpdatePage;

    # let session = confluence::Session::login("https://confluence", "user", "pass").unwrap();
    session.store_page(
        UpdatePage::with_create_fields(
            None,
            "SpaceKey",
            "Page Title",
            "<b>Works</b>"
        )
    );
    ```

    # For updating

    The Page given should have:

    - (optional) parent_id
    - id
    - space
    - title
    - content
    - version

    fields at a minimum.

    Use method `into` on `Page` to convert it to `UpdatePage`.

    ## Example

    ```no_run
    use confluence::UpdatePage;

    # let session = confluence::Session::login("https://confluence", "user", "pass").unwrap();
    let mut page = session.get_page_by_title(
        "SomeSpaceKey", "Page Title"
    ).unwrap();

    page.title = "New Page Title".into();

    session.store_page(page.into());
    ```
    */
    pub fn store_page(&self, page: UpdatePage) -> Result<Page> {
        let mut element_items = vec![
            Element::node("space").with_text(page.space),
            Element::node("title").with_text(page.title),
            Element::node("content").with_text(page.content),
        ];

        if let Some(id) = page.id {
            element_items.push(Element::node("id").with_text(id.to_string()));
        }

        if let Some(version) = page.version {
            element_items.push(Element::node("version").with_text(version.to_string()));
        }

        if let Some(parent_id) = page.parent_id {
            element_items.push(Element::node("parentId").with_text(parent_id.to_string()));
        }

        let response = try!(self.call(
            Method::new("storePage")
                .with(Element::node("token").with_text(self.token.clone()))
                .with(Element::node("page").with_children(element_items))
        ));

        let element = try!(response.body.descend(&["storePageReturn"]));

        Ok(try!(Page::from_element(element)))
    }

    /**
    Updates the page.

    Same as `store_page`, but with additional update options parameter.
    */
    pub fn update_page(&self, page: UpdatePage, options: PageUpdateOptions) -> Result<Page> {
        let mut element_items = vec![
            Element::node("space").with_text(page.space),
            Element::node("title").with_text(page.title),
            Element::node("content").with_text(page.content),
        ];

        if let Some(id) = page.id {
            element_items.push(Element::node("id").with_text(id.to_string()));
        }

        if let Some(version) = page.version {
            element_items.push(Element::node("version").with_text(version.to_string()));
        }

        if let Some(parent_id) = page.parent_id {
            element_items.push(Element::node("parentId").with_text(parent_id.to_string()));
        }

        let mut update_options = vec![];

        if let Some(comment) = options.version_comment {
            update_options.push(Element::node("versionComment").with_text(comment));
        }

        update_options.push(Element::node("minorEdit").with_text(if options.minor_edit {
            "true"
        } else {
            "false"
        }));

        let response = try!(self.call(
            Method::new("updatePage")
                .with(Element::node("token").with_text(self.token.clone()))
                .with(Element::node("page").with_children(element_items))
                .with(Element::node("pageUpdateOptions").with_children(update_options))
        ));

        let element = try!(response.body.descend(&["updatePageReturn"]));

        Ok(try!(Page::from_element(element)))
    }

    /**
    Returns all the direct children of this page.

    ## Example

    ```no_run
    # let session = confluence::Session::login("https://confluence", "user", "pass").unwrap();
    println!("Page Summaries: {:#?}",
        session.get_children(
            123456
        )
    );
    ```
    */
    pub fn get_children(&self, page_id: i64) -> Result<Vec<PageSummary>> {
        let response = try!(self.call(
            Method::new("getChildren")
                .with(Element::node("token").with_text(self.token.clone()))
                .with(Element::node("pageId").with_text(page_id.to_string()))
        ));

        let element = try!(response.body.descend(&["getChildrenReturn"]));

        let mut summaries = vec![];

        for element in element.children {
            summaries.push(try!(PageSummary::from_element(element)));
        }

        Ok(summaries)
    }

    /// Call a custom method on this session.
    ///
    /// ## Usage
    ///
    /// The elements in `Method` struct here will be converted directly
    /// into SOAP envelope's Body.
    ///
    /// The returned `Response`.`body` will contain the parsed Body element.
    ///
    /// ## Discussion
    ///
    /// So far only few methods have convenience wrappers here, so if you need to call [something
    /// else](https://developer.atlassian.com/confdev/confluence-rest-api/confluence-xml-rpc-and-soap-apis/remote-confluence-methods),
    /// it's not so convenient, but possible.
    ///
    /// If you need an example, look at how these convenience methods are implemented.
    ///
    /// Pull requests are welcome!
    pub fn call(&self, method: rpser::Method) -> Result<rpser::Response> {
        let url = match self.wsdl.operations.get(&method.name) {
            None => return Err(Error::MethodNotFoundInWsdl(method.name)),
            Some(ref op) => &op.url,
        };

        // do now show password in logs
        if method.name == "login" {
            debug!("[call] login ******");
        } else {
            debug!("[call] {}", method);
        }

        let envelope = method.as_xml(url);

        // do now show password in logs
        if method.name != "login" {
            trace!("[method xml] {}", envelope);
        }

        let http_response = try!(http::soap_action(url, &method.name, &envelope));

        trace!("[response xml] {}", http_response.body);

        Ok(try!(rpser::Response::from_xml(&http_response.body)))
    }
}

/// Confluence library error.
#[derive(Debug)]
pub enum Error {
    MethodNotFoundInWsdl(String),
    ReceivedNoLoginToken,
    Io(IoError),
    Http(HttpError),
    Rpc(RpcError),
}

impl From<HttpError> for Error {
    fn from(other: HttpError) -> Error {
        Error::Http(other)
    }
}

impl From<RpcError> for Error {
    fn from(other: RpcError) -> Error {
        Error::Rpc(other)
    }
}

impl From<rpser::xml::Error> for Error {
    fn from(other: rpser::xml::Error) -> Error {
        RpcError::from(other).into()
    }
}

impl From<IoError> for Error {
    fn from(other: IoError) -> Error {
        Error::Io(other)
    }
}

pub type Result<T> = result::Result<T, Error>;
