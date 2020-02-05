use crate::{
    request::WebHandle,
    WebResult,
    enums::*,
    xml::*,
};
use std::{
    borrow::Cow,
    fmt,
    fmt::Write,
};
use quick_xml::de;
use serde::Deserialize;
use std::ops::Deref;

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
    pub param_str: String,
}

impl<'c> Client<'c> {
    pub fn create(district_url: &'c str, username: &'c str, password: &'c str) -> Self {
        Client {
            uri: [district_url, ENDPOINT].concat().into(),
            user: username,
            pwd: password,
        }
    }

    #[inline]
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
            ("paramStr", &param_str.build_string())
        ];

        Ok(
            WebHandle::send(&self.uri, body)
                .await?
        )
    }

    #[inline]
    pub async fn get_grades(&self, report_period: Option<u64>) -> WebResult<GbData> {
        let parms = if report_period.is_none() {
            ParamBuilder::create()
        } else {
            ParamBuilder::create()
                .add_elements(&[ParamType::ReportPeriod(report_period.unwrap())])?
        };

        let xml_data = self.call_service(WebServiceHandle::PXPWebServices, Method::GradeBook, parms)
                .await?;

        Ok(de::from_str(xml_data.as_str())?)
    }

    #[inline]
    pub async fn get_absences(&self) -> WebResult<AttData> {
        let xml_data = self.call_service(WebServiceHandle::PXPWebServices, Method::Attendance, ParamBuilder::create())
            .await?;

        Ok(de::from_str(xml_data.as_str())?)
    }
}

impl ParamBuilder {
    pub fn create() -> Self {
        ParamBuilder {
            param_str: String::new()
        }
    }

    #[inline]
    pub fn add_elements(&mut self, params: &[ParamType]) -> Result<Self, std::fmt::Error> {
        for p in params.iter() {
            write!(&mut self.param_str, "{}\n", p.to_string())?;
        }

        Ok(self.clone())
    }

    #[inline]
    pub fn build_string(&self) -> String {
        ["<Parms>\n", self.param_str.as_ref(), "</Parms>"].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_building() {
        let mut params = ParamBuilder::create()
            .add_elements(&[ParamType::AssignmentID("e2qekn"),
                ParamType::ChildIntID(1),
                ParamType::LanguageCode(0),
                ParamType::RequestDate("1/23/19"),
                ParamType::HealthImmunizations(true)
            ]).unwrap();

        let res = "<Parms>\n<AssignmentID>e2qekn</AssignmentID>\n<ChildIntID>1</ChildIntID>\n<LanguageCode>0</LanguageCode>\n<RequestDate>1/23/19</RequestDate>\n<HealthImmunizations>true</HealthImmunizations>\n</Parms>";
        assert_eq!(&params.build_string(), res);
    }
}
