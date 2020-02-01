use std::{
    borrow::Cow,
    ops::Deref,
};
use reqwest::{
    Client,
    Method,
    header::HeaderMap,
    header::{CONTENT_TYPE, HOST, ACCEPT_ENCODING, USER_AGENT},
    Response,
};
use super::WebResult;
use serde::Serialize;

// POST request endpoint (ProcessWebServiceRequest is redundant for SOAP requests)
// In this case since we are solely making regular POST requests it is required
pub const ENDPOINT: &str = "/Service/PXPCommunication.asmx/ProcessWebServiceRequest";

pub struct WebHandle<'a> {
    pub uri: Cow<'a, str>,
}

impl<'a> WebHandle<'a> {
    pub async fn new(district_uri: &str) -> WebHandle<'a> {
        WebHandle {
            uri: format!("{}{}", district_uri, ENDPOINT).into(),
        }
    }

    pub fn get_default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::with_capacity(4);
        headers.insert(HOST, self.uri.deref().parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
        headers.insert(ACCEPT_ENCODING, "gzip".parse().unwrap());
        headers.insert(USER_AGENT, "ksoap2-android/2.6.0+".parse().unwrap());

        headers
    }

    pub async fn make_web_request<S: Serialize>(&self, uri: &str, method: Method, params: S, headers: HeaderMap) -> WebResult<Response>
    {
        let client = Client::new();
        let request = client
            .request(method, uri)
            .headers(headers.into())
            .form(&params)
            .send()
            .await?;

        Ok(request)
    }

    pub async fn send(
        &self,
        uri: Option<&str>,
        method: Option<Method>,
        params: impl Serialize,
        headers: Option<HeaderMap>
    ) -> WebResult<String> {
        Ok(
            self.make_web_request(
                uri.unwrap_or(&self.uri), method.unwrap_or(Method::POST), params, headers.unwrap_or(self.get_default_headers())
            )
                .await?
                .text()
                .await?
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn response_check() {
        assert_eq!(2, 2);
    }
}