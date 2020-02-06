use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct StudentClassSchedule {
    #[serde(rename = "StudentClassSchedule")]
    pub schedule: Schedule,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Schedule {
    #[serde(rename = "TermIndex")]
    pub term: u8,
    #[serde(rename = "ClassLists")]
    pub class_list: ClassList,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ClassList {
    #[serde(rename = "ClassListing")]
    pub class: Vec<Class>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Class {
    pub period: u8,
    pub course_title: String,
    pub room_name: String, // damn CAFE
    pub teacher: String,
    pub teacher_email: String,
}