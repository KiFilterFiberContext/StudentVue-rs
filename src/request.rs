use reqwest::{
    Client,
    Method,
    header::HeaderMap,
    header::{CONTENT_TYPE, HOST, ACCEPT_ENCODING, USER_AGENT},
    Response,
};
use super::WebResult;
use serde::Serialize;
use htmlescape::decode_html;

/// Struct which manages and sends web requests asynchronously
pub struct WebHandle;

impl WebHandle {
    /// Returns the default headers for **POST** requests from the StudentVue app
    ///
    /// # Headers:
    /// Content-Type, Accept-Encoding, User-Agent
    pub fn get_default_headers() -> HeaderMap {
        let mut headers = HeaderMap::with_capacity(3);
        headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
        headers.insert(ACCEPT_ENCODING, "gzip".parse().unwrap());
        headers.insert(USER_AGENT, "ksoap2-android/2.6.0+".parse().unwrap());

        headers
    }

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
    ///     let req = WebHandle::make_web_request("https://www.google.com", Method::POST, params, HeaderMap::new())
    ///         .await?;
    ///
    ///     println!("{:?}", req.status());
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn make_web_request<R, M, S>(uri: R, method: M, params: S, headers: HeaderMap) -> WebResult<Response>
    where
        R: AsRef<str>,
        M: Into<Method>,
        S: Serialize
    {
        let client = Client::new();
        let request = client
            .request(method.into(), uri.as_ref())
            .headers(headers)
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
    ///     let res = WebHandle::send("https://afsd.edupoint.com/Service/PXPCommunication.asmx/ProcessWebServiceRequest", &[("key", "value")], true)
    ///         .await?;
    ///
    ///     // ...
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn send(uri: impl AsRef<str>, params: impl Serialize, decode: bool) -> WebResult<String> {
        let req = WebHandle::make_web_request(uri, Method::POST, params, WebHandle::get_default_headers())
            .await?
            .text()
            .await?;

        let res = if decode {
            decode_html(req.as_str())
                .unwrap_or_default() // TODO: Use map_err
        }
        else {
            req
        };

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn status_check() {
        let status = WebHandle::make_web_request("http://www.google.com", Method::GET, <Vec<&str>>::new(), HeaderMap::new())
            .await
            .unwrap()
            .status();

        assert_eq!(status.as_str(), "200");
    }
}
