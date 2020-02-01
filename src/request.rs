use std::{
    borrow::Cow,
    ops::Deref,
};
use reqwest::{
    Client,
    Method,
    header::HeaderMap,
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

    pub fn get_default_headers() -> HeaderMap {
        unimplemented!()
    }

    pub async fn make_web_request(
        &self,
        uri: Option<&str>,
        method: Option<Method>,
        params: impl Serialize,
        headers: impl Into<HeaderMap>
    ) -> WebResult<Response>
    {
        let client = Client::new();
        let request = client
            .request(method.unwrap_or(Method::POST), uri.unwrap_or(&self.uri))
            .headers(headers)
            .form(&params)
            .send()
            .await?;

        Ok(request)
    }

    pub async fn send() -> WebResult<String> {
        unimplemented!()
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