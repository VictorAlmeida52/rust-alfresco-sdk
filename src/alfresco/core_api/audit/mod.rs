use reqwest::Client;
use base64::{Engine as _, engine::general_purpose};
use serde::Serialize;
use serde_json::json;
use crate::models::structs::{AuditApp, AuditAppPagingList};

pub struct AuditApi {
    pub base_url: String,
    pub client: Client,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ListAuditApplicationsQueryParams {
    skip_count: usize,
    max_items: usize,
    fields: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GetAuditApplicationInfoQueryParams {
    fields: Option<Vec<String>>,
    include: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct UpdateAuditApplicationInfoQueryParams {
    fields: Option<Vec<String>>,
}

impl AuditApi {
    pub fn new(base_url: &str, client: Client) -> Self {
        AuditApi {
            base_url: format!("{}/alfresco/api/-default-/public/alfresco/versions/1", base_url),
            client,
        }
    }

    pub async fn list_audit_applications(
        &self,
        ticket: &str,
        skip_count: &Option<usize>,
        max_items: &Option<usize>,
        fields: &Option<Vec<String>>
    ) -> Result<AuditAppPagingList, anyhow::Error> {
        let formatted_url = format!("{}/audit-applications", self.base_url);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let params = ListAuditApplicationsQueryParams {
            skip_count: skip_count.unwrap_or(0),
            max_items: max_items.unwrap_or(100),
            fields: fields.to_owned(),
        };

        let raw_response = self.client.get(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params)
            .send()
            .await?;

        let parsed = raw_response.json::<AuditAppPagingList>().await?;

        Ok(parsed)
    }

    pub async fn get_audit_application_info(
        &self,
        ticket: &str,
        fields: &Option<Vec<String>>,
        include: &Option<Vec<String>>,
        audit_application_id: &str,
    ) -> Result<AuditApp, anyhow::Error> {
        let formatted_url = format!("{}/audit-applications/{}", self.base_url, audit_application_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let params = GetAuditApplicationInfoQueryParams {
            fields: fields.to_owned(),
            include: include.to_owned(),
        };

        let raw_response = self.client.get(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params)
            .send()
            .await?;

        let parsed = raw_response.json::<AuditApp>().await?;

        Ok(parsed)
    }

    pub async fn update_audit_application_info(
        &self,
        ticket: &str,
        fields: &Option<Vec<String>>,
        is_enabled: &bool,
        audit_application_id: &str,
    ) -> Result<AuditApp, anyhow::Error> {
        let formatted_url = format!("{}/audit-applications/{}", self.base_url, audit_application_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let params = UpdateAuditApplicationInfoQueryParams {
            fields: fields.to_owned(),
        };

        let body = json!({
            "isEnabled": &is_enabled,
        });

        let raw_response = self.client.put(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params)
            .json(&body)
            .send()
            .await?;

        let parsed = raw_response.json::<AuditApp>().await?;

        Ok(parsed)
    }
}