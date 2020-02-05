use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct GbData {
    pub gradebook: Gradebook,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Gradebook {
    pub courses: Courses,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Courses {
    pub course: Vec<Course>
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Course {
    pub period: u8,
    #[serde(rename = "Title")]
    pub class_name: String,
    pub room: u32,
    pub staff: String,
    #[serde(rename = "StaffEMail")]
    pub staff_email: String,
    #[serde(rename = "Marks")]
    pub marks: Vec<Marks>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Marks {
    pub mark: Vec<Mark>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Mark {
    #[serde(rename = "CalculatedScoreString")]
    pub grade: char,
    #[serde(rename = "CalculatedScoreRaw")]
    pub percent: f32,
}
