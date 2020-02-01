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
    ChildIntID(u64),
    HealthConditions(bool),
    HealthVisits(bool),
    HealthImmunizations(bool),
    ReportPeriod(&'a str),
    ConcurrentSchOrgYearGU(&'a str),
    LoadAllTerms(bool),
    RequestDate(&'a str),
    LanguageCode(u64),
    ClassGU(&'a str),
    SoundFileListing(&'a str),
    GBDocumentData(&'a str),
    Key(&'a str),
    MatchToDistrictZipCode(&'a str),
    Empty,
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
