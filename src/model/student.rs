use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StudentInfo {
    #[serde(rename = "FormattedName")]
    pub name: String,
    #[serde(rename = "PermID")]
    pub id: u32,
    pub gender: String,
    pub grade: u8,
    pub address: String,
    pub nickname: Option<String>,
    pub birth_date: String,
    #[serde(rename = "EMail")]
    pub email: String,
    pub phone: String,
    pub current_school: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Student {
    pub student_info: StudentInfo,
}