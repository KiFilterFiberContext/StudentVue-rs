use std::{
    borrow::Cow,
    ops::Deref,
};
use reqwest::{
    Client,
    header::HeaderMap
};
use super::WebResult;
use serde::Serialize;

// POST request endpoint (ProcessWebServiceRequest is redundant for SOAP requests)
// In this case since we are solely making regular POST requests it is required
pub const ENDPOINT: &str = "/Service/PXPCommunication.asmx/ProcessWebServiceRequest";

pub struct WebHandle<'a> {
    uri: Cow<'a, str>,
}

impl<'a> WebHandle<'a> {
    async fn new(district_uri: &str) -> WebHandle<'a> {
        WebHandle {
            uri: format!("{}{}", district_uri, ENDPOINT).into(),
        }
    }

    async fn send_post_req<S, H>(&self, uri: Option<&str>, params: S, headers: H) -> WebResult<String>
    where
        S: Serialize,
        H: Into<HeaderMap>
    {
        let client = Client::new();
        let request = client.post(uri.unwrap_or(&self.uri.deref()))
            .form(&params)
            .headers(headers.into())
            .send()
            .await?;

        Ok(
            request
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