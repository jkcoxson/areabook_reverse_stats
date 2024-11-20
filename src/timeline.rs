// timeline.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: String, // UUID as a String
    #[serde(rename = "modBy")]
    pub modified_by: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<u64>, // Unix timestamp in milliseconds
    #[serde(rename = "endTime")]
    pub end_time: Option<u64>, // Unix timestamp in milliseconds
    #[serde(rename = "eventType")]
    pub event_type: u32,
    pub subject: Option<String>,
    #[serde(rename = "lessonPlan")]
    pub report: Option<String>,
    #[serde(rename = "contactTypeId")]
    pub contact_type_id: Option<u32>,
    #[serde(rename = "creationDate")]
    pub creation_date: Option<u64>, // Unix timestamp in milliseconds
    #[serde(rename = "updatedDate")]
    pub updated_date: Option<u64>, // Unix timestamp in milliseconds
    #[serde(rename = "lessonYN")]
    pub lesson_yn: u8, // Boolean as 0/1
    #[serde(rename = "ownerYN")]
    pub owner_yn: u8, // Boolean as 0/1
    #[serde(rename = "memberPresentYN")]
    pub member_present_yn: u8, // Boolean as 0/1
}

#[derive(Deserialize, Debug, Serialize)]
pub struct PersonEvent {
    pub id: String,
    #[serde(rename = "personId")]
    pub person_id: String,
    #[serde(rename = "eventId")]
    pub event_id: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Drop {
    pub id: String,
    #[serde(rename = "personId")]
    pub person_id: String,
    #[serde(rename = "dropDate")]
    pub drop_date: u64,
    pub note: Option<String>,
    pub created_by: Option<String>,
    pub status: u8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Reset {
    pub id: String,
    #[serde(rename = "personId")]
    pub person_id: String,
    #[serde(rename = "resetDate")]
    pub reset_date: u64,
    pub created_by: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SacramentAttendance {
    pub id: String,
    #[serde(rename = "personId")]
    pub person_id: String,
    pub date: String, // ISO 8601 format
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Contact {
    #[serde(rename = "id")]
    pub id: String, // UUID as a String

    #[serde(rename = "householdId")]
    pub household_id: String, // UUID as a String

    #[serde(rename = "status")]
    pub status: u32,

    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,

    #[serde(rename = "first")]
    pub first_name: Option<String>,

    #[serde(rename = "last")]
    pub last_name: Option<String>,

    #[serde(rename = "gender")]
    pub gender: Option<String>, // Assuming single-character string ("M"/"F")

    #[serde(rename = "note")]
    pub note: Option<String>,

    #[serde(rename = "phoneMobile")]
    pub phone_mobile: Option<String>, // Stored as a String to accommodate different phone formats

    #[serde(rename = "phoneWork")]
    pub phone_work: Option<String>,

    #[serde(rename = "phoneMobileTextable")]
    pub phone_mobile_textable: Option<bool>, // Boolean field

    #[serde(rename = "ageCategory")]
    pub age_category: Option<u32>,

    #[serde(rename = "affirmedInterestExpirationDate")]
    pub affirmed_interest_expiration_date: u64, // Unix timestamp in milliseconds

    #[serde(rename = "lastEventDate")]
    pub last_event_date: Option<u64>, // Unix timestamp in milliseconds

    #[serde(rename = "lastHappenedEventDate")]
    pub last_happened_event_date: Option<u64>, // Unix timestamp in milliseconds

    #[serde(rename = "contactSource")]
    pub contact_source: u32,

    #[serde(rename = "lastTaughtDate")]
    pub last_taught_date: Option<u64>, // Unix timestamp in milliseconds

    #[serde(rename = "createDate")]
    pub create_date: u64, // Unix timestamp in milliseconds

    #[serde(rename = "preferredLanguageId")]
    pub preferred_language_id: Option<u16>,

    #[serde(rename = "visibilityTypeId")]
    pub visibility_type_id: u32,

    #[serde(rename = "ownerStatus")]
    pub owner_status: u32,

    #[serde(rename = "prosAreaId")]
    pub pros_area_id: Option<u32>,

    #[serde(rename = "lastReassignedDate")]
    pub last_reassigned_date: Option<i64>, // Unix timestamp in milliseconds

    #[serde(rename = "foundByPersonId")]
    pub found_by_person_id: Option<String>, // Assuming this is an ID stored as a string

    #[serde(rename = "membershipCreationDate")]
    pub membership_creation_date: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CommandsResponse {
    #[serde(rename = "personEvents")]
    pub person_events: Vec<PersonEvent>,
    pub events: Vec<Event>,
    #[serde(rename = "personDrops")]
    pub person_drops: Vec<Drop>,
    #[serde(rename = "personResets")]
    pub person_resets: Vec<Reset>,
    #[serde(rename = "sacramentAttendance")]
    pub sacrament_attendance: Vec<SacramentAttendance>,
    pub contacts: Vec<Contact>,
    #[serde(rename = "prosAreaName")]
    pub pros_area_name: String,
}

#[derive(Debug)]
pub struct TimelineEntry {
    pub time: u64,
    pub details: String,
    pub kind: TimelineEventKind,
}

#[derive(Debug)]
pub enum TimelineEventKind {
    Event,
    Drop,
    Reset,
    Sacrament,
}

pub struct Person {
    pub name: String,
    pub language: Option<u16>,
    pub area: String,
    pub timeline: Vec<TimelineEntry>,
}
