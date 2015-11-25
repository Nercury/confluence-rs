#[macro_use] extern crate hyper;
#[macro_use] extern crate log;
extern crate xml;
extern crate xmltree;

pub mod http;
pub mod wsdl;
pub mod rpser;

mod space;

pub use space::Space;

use std::result;
use std::io::{ Error as IoError };

use xmltree::Element;
use self::rpser::xml::BuildElement;
use self::rpser::{ RpcError, Method };
use self::http::HttpError;

const V2_API_RPC_PATH: &'static str = "/rpc/soap-axis/confluenceservice-v2?wsdl";

/// Client's session.
///
/// Creating this session performs log-in to confluence api.
///
/// User will be logged out automatically when session is no longer used. Manual
/// call to `logout` is not required.
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

        let url = if url.ends_with("/") { &url[..url.len() - 1] } else { url };
        let wsdl_url = [url, V2_API_RPC_PATH].concat();

        debug!("getting wsdl from url {:?}", wsdl_url);

        let wsdl = try!(wsdl::fetch(&wsdl_url));
        let mut session = Session { wsdl: wsdl, token: String::new() };

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

    /// Log out out of confluence.
    pub fn logout(&self) -> Result<bool> {

        let response = try!(self.call(
            Method::new("logout")
                .with(Element::node("token").with_text(self.token.clone()))
        ));

        Ok(match try!(response.body.descend(&["logoutReturn"])).text {
            Some(ref v) if v == "true" => {
                debug!("logged out successfully");
                true
            },
            _ => {
                debug!("log out failed (maybe expired token, maybe not loged in)");
                false
            },
        })
    }

    /**
    Get confluence's `Space`.

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

        let page_return = try!(response.body.descend(&["getSpaceReturn"]));

        trace!("page response {:#?}", page_return);

        Ok(Space {
            description: try!(page_return.get_at_path(&["description"])).text,
            home_page: try!(page_return.get_at_path(&["homePage"]).and_then(|e| e.as_long())),
            key: try!(page_return.get_at_path(&["key"]).and_then(|e| e.as_string())),
            name: try!(page_return.get_at_path(&["name"]).and_then(|e| e.as_string())),
            space_group: try!(page_return.get_at_path(&["name"])).text,
            space_type: try!(page_return.get_at_path(&["type"]).and_then(|e| e.as_string())),
            url: try!(page_return.get_at_path(&["url"]).and_then(|e| e.as_string())),
        })
    }

    /// Get confluence's `Page`.
    pub fn get_page_by_title(&self, space_key: &str, page_title: &str) -> Result<bool> {

        let response = try!(self.call(
            Method::new("getPage")
                .with(Element::node("token").with_text(self.token.clone()))
                .with(Element::node("spaceKey").with_text(space_key))
                .with(Element::node("pageTitle").with_text(page_title))
        ));

        let page_return = try!(response.body.descend(&["getPageReturn"]));

        debug!("page response {:#?}", page_return);

        Ok(true)
    }

    /// Call custom method on this session.
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
