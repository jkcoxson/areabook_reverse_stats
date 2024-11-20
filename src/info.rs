use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Calling {
    #[serde(rename = "cmisId")]
    pub cmis_id: u64,
    #[serde(rename = "positionId")]
    pub position_id: u64,
    #[serde(rename = "positionName")]
    pub position_name: String,
    #[serde(rename = "unitId")]
    pub unit_id: u64,
}

#[derive(Deserialize, Debug)]
pub struct MissionaryInfo {
    #[serde(rename = "missionaryId")]
    pub missionary_id: u64,
    #[serde(rename = "returningMemberMission")]
    pub returning_member_mission: bool,
    #[serde(rename = "ldsAccountId")]
    pub lds_account_id: u64,
    #[serde(rename = "ldsAccountIdOfProxyingUser")]
    pub lds_account_id_of_proxying_user: u64,
    #[serde(rename = "cmisId")]
    pub cmis_id: u64,
    pub roles: Vec<String>,
    pub first: String,
    pub last: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub gender: String,
    #[serde(rename = "churchArea")]
    pub church_area: String,
    pub callings: Option<Vec<Calling>>,
    #[serde(rename = "missionId")]
    pub mission_id: u64,
    #[serde(rename = "missionName")]
    pub mission_name: String,
    #[serde(rename = "missionOrgNumber")]
    pub mission_org_number: u64,
    #[serde(rename = "zoneId")]
    pub zone_id: u64,
    #[serde(rename = "zoneName")]
    pub zone_name: String,
    #[serde(rename = "districtId")]
    pub district_id: u64,
    #[serde(rename = "districtName")]
    pub district_name: String,
    #[serde(rename = "prosAreaId")]
    pub pros_area_id: u64,
    #[serde(rename = "prosAreaName")]
    pub pros_area_name: String,
}

impl MissionaryInfo {
    /// Reads the missionary information from a JSON file at the given path.
    pub fn from_cache<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content.try_into()?)
    }
}

impl TryFrom<String> for MissionaryInfo {
    type Error = std::io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(&value)?)
    }
}
