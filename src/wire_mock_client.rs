use serde::{Deserialize, Serialize};
use ureq;

#[derive(Serialize, Deserialize, Debug)]
pub struct WireMockStub {
    pub request: RequestPattern,
    pub response: ResponseDefinition,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPattern {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_path_pattern: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDefinition {
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StubMappings {
    pub mappings: Vec<StubMapping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StubMapping {
    pub id: String,
    pub request: RequestPattern,
    pub response: ResponseDefinition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub total: usize,
}

pub fn get_all_stubs(base_url: &str) -> Result<StubMappings, Box<dyn std::error::Error>> {
    let url = format!("{}/__admin/mappings", base_url);

    let mut response = ureq::get(&url)
        .header("Accept", "application/json")
        .call()?;

    if response.status() == 200 {
        let data: StubMappings = response.body_mut().read_json()?;
        Ok(data)
    } else {
        Err(format!("Failed to retrieve stubs: HTTP {}", response.status()).into())
    }
}
