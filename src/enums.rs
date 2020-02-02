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

#[derive(Debug, PartialEq)]
pub enum ParamType<'a> {
    ChildIntID(&'a str), // u64
    HealthConditions(&'a str), // u64
    HealthVisits(&'a str), // bool
    HealthImmunizations(&'a str), // bool
    ReportPeriod(&'a str),
    ConcurrentSchOrgYearGU(&'a str),
    LoadAllTerms,
    RequestDate(&'a str),
    AssignmentID(&'a str),
    LanguageCode(&'a str), // u64
    ClassGU(&'a str),
    StudentClassList(&'a str),
    SoundFileListing(&'a str),
    GBDocumentData(&'a str),
    Key,
    MatchToDistrictZipCode(&'a str),
}

impl<'p> ToString for ParamType<'p> {
    fn to_string(&self) -> String {
        let (name, value) = match self {
            ParamType::ChildIntID(id) => ("ChildIntID", id),
            ParamType::HealthConditions(c) => ("HealthConditions", c),
            ParamType::HealthVisits(v) => ("HealthVisits", v),
            ParamType::HealthImmunizations(imm) => ("HealthImmunizations", imm),
            ParamType::ConcurrentSchOrgYearGU(gu) => ("ConcurrentSchOrgYearGU", gu),
            ParamType::ReportPeriod(period) => ("ReportPeriod", period),
            ParamType::StudentClassList(list) => ("StudentClassList", list),
            ParamType::RequestDate(date) => ("RequestDate", date),
            ParamType::LanguageCode(lang_id) => ("LanguageCode", lang_id),
            ParamType::ClassGU(class_gu) => ("ClassGU", class_gu),
            ParamType::AssignmentID(id) => ("AssignmentID", id),
            ParamType::SoundFileListing(listing) => ("SoundFileListing", listing),
            ParamType::GBDocumentData(data) => ("GBDocumentData", data),
            ParamType::Key => ("Key", &"5E4B7859-B805-474B-A833-FDB15D205D40"),
            ParamType::MatchToDistrictZipCode(zip) => ("MatchToDistrictZipCode", zip),
            ParamType::LoadAllTerms => ("LoadAllTerms", &"true")
        };

        // lol
        format!("\n<{}>{}</{}>", name, value, name)
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
