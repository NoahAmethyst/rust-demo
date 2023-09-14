use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct PodReq {
    #[serde(rename = "podName")]
    pub pod_name: Option<String>,

    #[serde(rename = "namespace")]
    pub namespace: Option<String>,

    #[serde(rename = "startTime")]
    pub start_time: Option<NaiveDateTime>,

    #[serde(rename = "endTime")]
    pub end_time: Option<NaiveDateTime>,
}