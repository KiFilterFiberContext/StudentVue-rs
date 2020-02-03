#[derive(Debug)]
pub enum WebServiceHandle {
    PXPWebServices,
    HDInfoServices,
}

#[derive(Debug)]
pub enum Method {
    Attendance,
    StudentHealthInfo,
    GetStudentDocumentInitialData,
    GradeBook,
    StudentCalendar,
    TestWebServiceURL,
    ChildList,
    GetSupportedLanguages,
    GetContentOfAttachedDoc,
    StudentCalendarAssignmentDetails,
    SaveSoundFileData,
    UploadGBDocDataForStudentAssignment,
    StudentHWNotes,
    UpdateStudentHWNotes,
    StudentInfo,
    UpdatePXPMessage,
    StudentSchoolInfo,
    UpdateDeviceToken,
    StudentDisciplineInfo,
    GenerateAuthToken,
    StudentConference,
    GetMatchingDistrictList,
    StudentFee,
    PXPContentClassWebSiteGetFileXML,
    GetPXPMessages,
    GetContentUserDefinedModule,
    GetSoundFileData,
    GetAttachedDocToAssignment,
    GetClassWebSiteData,
    GetContentOfGBAttachedDoc,
    GetReportCardInitialData,
    GetReportCardDocumentData,
    GetSpecialEdData,
    StudentClassList,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Language {
    English = 0,
    Spanish = 1,
}

#[derive(Debug, PartialEq)]
pub enum ParamType<'a> {
    ChildIntID(u64),
    // u64
    HealthConditions(u64),
    // u64
    HealthVisits(bool),
    // bool
    HealthImmunizations(bool),
    // bool
    ReportPeriod(u64),
    ConcurrentSchOrgYearGU(&'a str),
    LoadAllTerms,
    RequestDate(&'a str),
    AssignmentID(&'a str),
    LanguageCode(u64),
    // u64
    ClassGU(&'a str),
    StudentClassList(&'a str),
    SoundFileListing(&'a str),
    GBDocumentData(&'a str),
    Key,
    MatchToDistrictZipCode(&'a str),
}

impl<'p> ToString for ParamType<'p> {
    fn to_string(&self) -> String {
        match self {
            ParamType::ChildIntID(id) => format!("<ChildIntID>{}</ChildIntID>", id),
            ParamType::HealthConditions(c) => format!("<HealthConditions>{}</HealthConditions>", c),
            ParamType::HealthVisits(v) => format!("<HealthVisits>{}</HealthVisits>", v),
            ParamType::HealthImmunizations(imm) => format!("<HealthImmunizations>{}</HealthImmunizations>", imm),
            ParamType::ConcurrentSchOrgYearGU(gu) => format!("<ConcurrentSchOrgYearGU>{}</ConcurrentSchOrgYearGU>", gu),
            ParamType::ReportPeriod(period) => format!("<ReportPeriod>{}</ReportPeriod>", period),
            ParamType::StudentClassList(list) => format!("<StudentClassList>{}</StudentClassList>", list),
            ParamType::RequestDate(date) => format!("<RequestDate>{}</RequestDate>", date),
            ParamType::LanguageCode(lang_id) => format!("<LanguageCode>{}</LanguageCode>", lang_id),
            ParamType::ClassGU(class_gu) => format!("<ClassGU>{}</ClassGU>", class_gu),
            ParamType::AssignmentID(id) => format!("<AssignmentID>{}</AssignmentID>", id),
            ParamType::SoundFileListing(listing) => format!("<SoundFileListing>{}</SoundFileListing>", listing),
            ParamType::GBDocumentData(data) => format!("<GBDocumentData>{}</GBDocumentData>", data),
            ParamType::Key => String::from("<Key>5E4B7859-B805-474B-A833-FDB15D205D40</Key>"),
            ParamType::MatchToDistrictZipCode(zip) => format!("<MatchToDistrictZipCode>{}</MatchToDistrictZipCode>", zip),
            ParamType::LoadAllTerms => String::from("<LoadAllTerms>true</LoadAllTerms>")
        }
    }
}

impl<'w> Into<&'w str> for WebServiceHandle {
    fn into(self) -> &'w str {
        match self {
            WebServiceHandle::PXPWebServices => "PXPWebServices",
            WebServiceHandle::HDInfoServices => "HDInfoServices",
        }
    }
}

impl<'m> Into<&'m str> for Method {
    fn into(self) -> &'m str {
        match self {
            Method::Attendance => "Attendance",
            Method::StudentHealthInfo => "StudentHealthInfo",
            Method::GetStudentDocumentInitialData => "GetStudentDocumentInitialData",
            Method::GradeBook => "Gradebook",
            Method::StudentCalendar => "StudentCalendar",
            Method::TestWebServiceURL => "TestWebServiceURL",
            Method::ChildList => "ChildList",
            Method::GetSupportedLanguages => "GetSupportedLanguages",
            Method::GetContentOfAttachedDoc => "GetContentOfAttachedDoc",
            Method::StudentCalendarAssignmentDetails => "StudentCalendarAssignmentDetails",
            Method::SaveSoundFileData => "SaveSoundFileData",
            Method::UploadGBDocDataForStudentAssignment => "UploadGBDocumentDataForStudentAssigment",
            Method::StudentHWNotes => "StudentHWNotes",
            Method::UpdateStudentHWNotes => "UpdateStudentHWNotes",
            Method::StudentInfo => "StudentInfo",
            Method::UpdatePXPMessage => "UpdatePXPMessage",
            Method::StudentSchoolInfo => "StudentSchoolInfo",
            Method::UpdateDeviceToken => "UpdateDeviceToken",
            Method::StudentDisciplineInfo => "StudentDisciplineInfo",
            Method::GenerateAuthToken => "GenerateAuthToken",
            Method::StudentConference => "StudentConference",
            Method::GetMatchingDistrictList => "GetMatchingDistrictList",
            Method::StudentFee => "StudentFee",
            Method::PXPContentClassWebSiteGetFileXML => "PXPContentCLassWebSiteGetFileXML",
            Method::GetPXPMessages => "GetPXPMessages",
            Method::GetContentUserDefinedModule => "GetContentUserDefinedModule",
            Method::GetSoundFileData => "GetSoundFileData",
            Method::GetAttachedDocToAssignment => "GetAttachedDocToAssignment",
            Method::GetClassWebSiteData => "GetClassWebSiteData",
            Method::GetContentOfGBAttachedDoc => "GetContentOfGBAttachedDoc",
            Method::GetReportCardInitialData => "GetReportCardInitialData",
            Method::GetReportCardDocumentData => "GetReportCardDocumentData",
            Method::GetSpecialEdData => "GetSpecialEdData",
            Method::StudentClassList => "StudentClassList",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_test() {
        let param = ParamType::ReportPeriod(7);
        assert_eq!(&param.to_string()[..], "<ReportPeriod>7</ReportPeriod>")
    }
}
