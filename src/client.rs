use crate::{
    request::WebHandle,
    WebResult,
    enums::*,
};
use std::{
    borrow::Cow,
    fmt,
};

// POST request endpoint (ProcessWebServiceRequest is redundant for SOAP requests)
// In this case since we are solely making regular POST requests it is required
static ENDPOINT: &str = "/Service/PXPCommunication.asmx/ProcessWebServiceRequest";

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

    pub async fn call_service(
        &self,
        web_service_handle: WebServiceHandle,
        method_name: Method,
        param_str: ParamBuilder,
    ) -> WebResult<String> {
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

    #[inline]
    pub async fn get_grades(&self, report_period: Option<u64>) -> WebResult<String> {
        let parms = if report_period.is_none() {
            ParamBuilder::create()
        } else {
            ParamBuilder::create_with_size(1)
                .add_element(ParamType::ReportPeriod(report_period.unwrap()))
        };

        Ok(
            self.call_service(WebServiceHandle::PXPWebServices, Method::GradeBook, parms)
                .await?
        )
    }
}

impl ParamBuilder {
    pub fn create() -> Self {
        ParamBuilder {
            children: Vec::new(),
        }
    }

    pub fn create_with_size(size: usize) -> Self {
        ParamBuilder {
            children: Vec::with_capacity(size),
        }
    }

    pub fn add_element(&mut self, param: ParamType) -> Self {
        self.children.push(format!("\n{}", param.to_string()));
        self.clone()
    }
}

impl<'p> ToString for ParamBuilder {
    #[inline]
    fn to_string(&self) -> String {
        format!("<Parms>{}\n</Parms>", self.children.join(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_building() {
        let mut params = ParamBuilder::create_with_size(5)
            .add_element(ParamType::AssignmentID("e2qekn"))
            .add_element(ParamType::ChildIntID(1))
            .add_element(ParamType::LanguageCode(0))
            .add_element(ParamType::RequestDate("1/23/19"))
            .add_element(ParamType::HealthImmunizations(true));

        let res = "<Parms>\n<AssignmentID>e2qekn</AssignmentID>\n<ChildIntID>1</ChildIntID>\n<LanguageCode>0</LanguageCode>\n<RequestDate>1/23/19</RequestDate>\n<HealthImmunizations>true</HealthImmunizations>\n</Parms>";
        assert_eq!(&params.to_string(), res);
    }
}
