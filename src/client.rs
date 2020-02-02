use std::{
    borrow::Cow,
    fmt,
};
use crate::{
    request::WebHandle,
    WebResult,
    enums::*,
    XMLElement,
};

// POST request endpoint (ProcessWebServiceRequest is redundant for SOAP requests)
// In this case since we are solely making regular POST requests it is required
pub const ENDPOINT: &str = "/Service/PXPCommunication.asmx/ProcessWebServiceRequest";

/// Struct which connects to the StudentVUE service
#[derive(Debug, Clone, PartialEq)]
pub struct Client<'c> {
    pub uri: Cow<'c, str>,
    pub user: &'c str,
    pub pwd: &'c str,
}

#[derive(Debug, Clone)]
pub struct ParamBuilder {
    pub children: Vec<String>,
}

impl<'c> Client<'c> {
    pub fn create(district_url: &'c str, username: &'c str, password: &'c str) -> Self {
        Client {
            uri: format!("{}{}", district_url, ENDPOINT).into(),
            user: username,
            pwd: password,
        }
    }
    pub async fn call_service(&self, web_service_handle: WebServiceHandle, method_name: Method, param_str: ParamBuilder) -> WebResult<String> {
        let body = [
            ("userID", self.user),
            ("password", self.pwd),
            ("skipLoginLog", "true"),
            ("parent", "false"),
            ("webServiceHandleName", web_service_handle.into()),
            ("methodName", method_name.into()),
            ("paramStr", &param_str.to_string())
        ];

        Ok(
            WebHandle::send(&self.uri, body, true)
                .await?
        )
    }
}

impl<'p> ParamBuilder {
    pub fn create(len: usize) -> Self {
        ParamBuilder {
            children: Vec::with_capacity(len),
        }
    }

    pub fn add_element(&mut self, param: ParamType) -> Self {
        self.children.push(param.to_string());
        self.clone()
    }
}

impl<'p> ToString for ParamBuilder {
    fn to_string(&self) -> String {
        format!("<Parms>{}</Parms>", self.children.join(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn request1() {

    }
}

