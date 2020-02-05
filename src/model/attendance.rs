use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Absence {
    pub absence_date: String,
    pub reason: String,
    pub note: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Absences {
    #[serde(rename = "Absence")]
    pub absence: Vec<Absence>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Attendance {
    pub absences: Absences,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct AttData {
    pub attendance: Attendance
}
