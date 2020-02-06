use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct StudentSchoolInfoListing {
    #[serde(rename = "StudentSchoolInfoListing")]
    pub school_info: SchoolInfo,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SchoolInfo {
    #[serde(rename = "School")]
    pub school_name: String,
    pub principal: String,
    pub school_address: String,
    pub school_city: String,
    pub school_state: String,
    pub school_zip: u32,
    pub phone: String,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "StaffLists")]
    pub staff_list: StaffList,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct StaffList {
    #[serde(rename = "StaffList")]
    pub staff: Vec<Staff>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Staff {
    pub name: String,
    #[serde(rename = "EMail")]
    pub email: String,
    pub title: String,
    pub phone: String,
}