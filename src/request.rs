use crate::WebResult;
use std::ops::Deref;
use reqwest::{
    Client,
    Method,
    header::HeaderMap,
    header::*,
    Response,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_HEADERS: HeaderMap<HeaderValue> = {
        let mut hm = HeaderMap::with_capacity(3);
        hm.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
        hm.insert(ACCEPT_ENCODING, "gzip".parse().unwrap());
        hm.insert(USER_AGENT, "ksoap2-android/2.6.0+".parse().unwrap());

        hm
    };
}

/// Struct which manages and sends web requests asynchronously
#[derive(Clone, Copy)]
pub struct WebHandle;

impl WebHandle {
    /// Asynchronously sends a HTTP Request requiring manual parameters which returns a [Response](https://docs.rs/reqwest/0.10.1/reqwest/struct.Response.html)
    ///
    /// ```
    /// use studentvue::request::WebHandle;
    /// use reqwest::{
    ///     header::HeaderMap,
    ///     Method,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let params: Vec<&str> = Vec::new();
    ///     let req = WebHandle::make_web_request("https://www.google.com", Method::POST, params, &HeaderMap::new())
    ///         .await?;
    ///
    ///     println!("{:?}", req.status());
    ///     Ok(())
    /// }
    /// ```
    ///
    #[inline]
    pub async fn make_web_request<R, M, S>(uri: R, method: M, params: S, headers: &HeaderMap) -> WebResult<Response>
    where
        R: AsRef<str>,
        M: Into<Method>,
        S: serde::Serialize
    {
        let client = Client::new();
        let request = client
            .request(method.into(), uri.as_ref())
            .headers(headers.clone())
            .form(&params)
            .send()
            .await?;

        Ok(request)
    }

    /// Asynchronously sends a POST request to the corresponding WebService endpoint or an optional url with specified parameters
    /// Optionally HTML escapes the response
    ///
    /// ```
    /// use studentvue::request::WebHandle;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let res = WebHandle::send("https://afsd.edupoint.com/Service/PXPCommunication.asmx/ProcessWebServiceRequest", &[("key", "value")])
    ///         .await?;
    ///
    ///     // ...
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn send(uri: impl AsRef<str>, params: impl serde::Serialize) -> WebResult<String> {
        let req = WebHandle::make_web_request(uri, Method::POST, params, DEFAULT_HEADERS.deref())
            .await?
            .text()
            .await?;

        Ok(
            String::from_utf8_lossy(req.as_bytes())
                .replace("&lt;", "<")
                .replace("&gt;", ">")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn status_check() {
        let status = WebHandle::make_web_request("http://www.google.com", Method::GET, <Vec<&str>>::new(), &HeaderMap::new())
            .await
            .unwrap()
            .status();

        assert_eq!(status.as_str(), "200");
    }
}
