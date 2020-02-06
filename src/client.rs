//! High level connection client
//!
//! This module provides `ParamBuilder` as an high level XML formatter to create parameter strings and
//! `Client` which can seamlessly interface with any school's studentvue API

use crate::{
    request::WebHandle,
    enums::*,
    model::*,
    error::VueError,
};
use std::{
    borrow::Cow,
    fmt,
    fmt::Write,
};
use quick_xml::de;
use serde::Deserialize;

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

/// StudentVUE parameter builder
#[derive(Debug, Clone)]
pub struct ParamBuilder {
    pub param_str: String,
}

impl<'c> Client<'c> {
    /// Instantiates a new `Client` with the username, password, and corresponding StudentVUE district url
    pub fn create(district_url: &'c str, username: &'c str, password: &'c str) -> Self {
        Client {
            uri: [district_url, ENDPOINT].concat().into(),
            user: username,
            pwd: password,
        }
    }

    /// Calls a method from a specified `WebServiceHandle` with the specified parameters
    ///
    /// # Example
    ///
    /// ```
    /// use studentvue::{
    ///     client::Client,
    ///     enums::{Method, WebServiceHandle},
    ///     client::ParamBuilder
    /// };
    /// use std::env;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (user, pwd) = (env::args().next().unwrap(), env::args().next().unwrap());
    ///
    ///     let client = Client::create("https://studentvue.phoenixunion.org", user.as_str(), pwd.as_str());
    ///     let xml = client.call_service(WebServiceHandle::PXPWebServices, Method::StudentSchoolInfo, ParamBuilder::create())
    ///         .await
    ///         .expect("Could not call service!");
    ///
    ///     println!("{}", xml);
    /// }
    /// ```
    pub async fn call_service(
        &self,
        web_service_handle: WebServiceHandle,
        method_name: Method,
        param_str: ParamBuilder,
    ) -> Result<String, VueError> {
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

    /// Retrieves grades from a student; can be current or from a specified reporting period
    #[inline]
    pub async fn get_grades(&self, report_period: Option<u64>) -> Result<grade::GbData, VueError> {
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

    /// Gets the absences from the student
    #[inline]
    pub async fn get_attendance(&self) -> Result<attendance::AttData, VueError> {
        let xml_data = self.call_service(WebServiceHandle::PXPWebServices, Method::Attendance, ParamBuilder::create())
            .await?;

        Ok(de::from_str(xml_data.as_str())?)
    }

    /// Retrieves student information such as their name, address, and grade
    #[inline]
    pub async fn get_student_info(&self) -> Result<student::Student, VueError> {
        let xml_data = self.call_service(WebServiceHandle::PXPWebServices, Method::StudentInfo, ParamBuilder::create())
            .await?;

        Ok(de::from_str(xml_data.as_str())?)
    }

    /// Retrieves the student's current school schedule
    #[inline]
    pub async fn get_schedule(&self) -> Result<schedule::StudentClassSchedule, VueError> {
        let xml_data = self.call_service(WebServiceHandle::PXPWebServices, Method::StudentClassList, ParamBuilder::create())
            .await?;

        Ok(de::from_str(xml_data.as_str())?)
    }

    /// Grabs information about the currently attended school
    #[inline]
    pub async fn get_school_info(&self) -> Result<school::StudentSchoolInfoListing, VueError> {
        let xml_data = self.call_service(WebServiceHandle::PXPWebServices, Method::StudentSchoolInfo, ParamBuilder::create())
            .await?;

        Ok(de::from_str(xml_data.as_str())?)
    }
}

impl ParamBuilder {
    /// Creates a new `ParamBuilder` instance
    pub fn create() -> Self {
        ParamBuilder {
            param_str: String::new()
        }
    }

    /// Adds several elements to the `Parms` node
    #[inline]
    pub fn add_elements(&mut self, params: &[ParamType]) -> Result<Self, std::fmt::Error> {
        for p in params.iter() {
            write!(&mut self.param_str, "{}\n", p.to_string())?;
        }

        Ok(self.clone())
    }

    /// Creates a xml string based on the attained attribute strings
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
