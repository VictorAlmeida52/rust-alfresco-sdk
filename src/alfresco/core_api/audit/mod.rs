use reqwest::Client;
use base64::{Engine as _, engine::general_purpose};
use serde::Serialize;
use serde_json::json;
use crate::models::structs::{AlfError, AuditApp, AuditAppPagingList, AuditEntryEntry, AuditEntryPaging};

use anyhow::Result;

pub struct AuditApi {
    pub base_url: String,
    pub client: Client,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ListAuditApplicationsQueryParams {
    skip_count: u32,
    max_items: u32,
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

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ListAuditEntriesForAuditAppQueryParams {
    skip_count: u32,
    omit_total_items: bool,
    order_by: Option<Vec<String>>,
    max_items: u32,
    #[serde(alias = "where")]
    where_filter: Option<String>,
    include: Option<Vec<String>>,
    fields: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct DeleteAuditEntriesForAuditAppQueryParams {
    #[serde(alias = "where")]
    where_filter: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GetAuditEntryQueryParams {
    fields: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ListAuditEntriesForNodeQueryParams {
    skip_count: u32,
    order_by: Option<Vec<String>>,
    max_items: u32,
    #[serde(alias = "where")]
    where_filter: Option<String>,
    include: Option<Vec<String>>,
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
        skip_count: Option<u32>,
        max_items: Option<u32>,
        fields: Option<Vec<String>>
    ) -> Result<AuditAppPagingList, AlfError> {
        let formatted_url = format!("{}/audit-applications", self.base_url);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let params = ListAuditApplicationsQueryParams {
            skip_count: skip_count.unwrap_or(0),
            max_items: max_items.unwrap_or(100),
            fields,
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
        audit_application_id: &str,
        fields: Option<Vec<String>>,
        include: Option<Vec<String>>,
    ) -> Result<AuditApp, AlfError> {
        let formatted_url = format!("{}/audit-applications/{}", self.base_url, audit_application_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let params = GetAuditApplicationInfoQueryParams {
            fields,
            include,
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
        audit_application_id: &str,
        fields: Option<Vec<String>>,
        is_enabled: &bool,
    ) -> Result<AuditApp, AlfError> {
        let formatted_url = format!("{}/audit-applications/{}", self.base_url, audit_application_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let params = UpdateAuditApplicationInfoQueryParams {
            fields,
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

    pub async fn list_audit_entries_for_audit_app(
        &self,
        ticket: &str,
        audit_application_id: &str,
        skip_count: Option<u32>,
        omit_total_items: Option<bool>,
        order_by: Option<Vec<String>>,
        max_items: Option<u32>,
        where_filter: Option<String>,
        include: Option<Vec<String>>,
        fields: Option<Vec<String>>,
    ) -> Result<AuditEntryPaging, AlfError> {
        let formatted_url = format!("{}/audit-applications/{}/audit-entries", self.base_url, audit_application_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let params = ListAuditEntriesForAuditAppQueryParams {
            skip_count: skip_count.unwrap_or(0),
            omit_total_items: omit_total_items.unwrap_or(false),
            order_by,
            max_items: max_items.unwrap_or(100),
            where_filter,
            include,
            fields,
        };


        let raw_response = self.client.get(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params)
            .send()
            .await?;

        let parsed = raw_response.json::<AuditEntryPaging>().await?;

        Ok(parsed)
    }

    pub async fn delete_audit_entries_for_audit_app(
        &self,
        ticket: &str,
        audit_application_id: &str,
        where_filter: String,
    ) -> Result<AuditEntryPaging, AlfError> {
        let formatted_url = format!("{}/audit-applications/{}/audit-entries", self.base_url, audit_application_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let params = DeleteAuditEntriesForAuditAppQueryParams {
            where_filter,
        };

        let raw_response = self.client.delete(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params)
            .send()
            .await?;

        let parsed = raw_response.json::<AuditEntryPaging>().await?;

        Ok(parsed)
    }

    pub async fn get_audit_entry(
        &self,
        ticket: &str,
        audit_application_id: &str,
        audit_entry_id: &str,
        fields: Option<Vec<String>>,
    ) -> Result<AuditEntryEntry, AlfError> {
        let formatted_url = format!("{}/audit-applications/{}/audit-entries/{}", self.base_url, audit_application_id, audit_entry_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let params = GetAuditEntryQueryParams {
            fields,
        };

        let raw_response = self.client.get(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params)
            .send()
            .await?;

        let parsed = raw_response.json::<AuditEntryEntry>().await?;

        Ok(parsed)
    }

    pub async fn permanently_delete_audit_entry(
        &self,
        ticket: &str,
        audit_application_id: &str,
        audit_entry_id: &str,
    ) -> Result<String, AlfError> {
        let formatted_url = format!("{}/audit-applications/{}/audit-entries/{}", self.base_url, audit_application_id, audit_entry_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let raw_response = self.client.delete(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .send()
            .await?;

        let status_code = raw_response.status().as_u16();
        let response_body_empty_unless_error = raw_response.text().await?;

        match status_code {
            204 => Ok("Successfully deleted audit entry".to_string()),
            400 => Ok("Invalid parameter: auditApplicationId  or auditEntryId is not a valid format".to_string()),
            401 => Ok("Authentication failed".to_string()),
            403 => Ok("Current user does not have permission to delete audit information".to_string()),
            404 => Ok("auditApplicationId or auditEntryId does not exist".to_string()),
            501 => Ok("Audit is disabled for the system".to_string()),
            _ => {
                let error: AlfError = serde_json::from_str(&response_body_empty_unless_error)?;
                Err(error)
            }
        }
    }

    pub async fn list_audit_entries_for_node(
        &self,
        ticket: &str,
        node_id: &str,
        skip_count: Option<u32>,
        order_by: Option<Vec<String>>,
        max_items: Option<u32>,
        where_filter: Option<String>,
        include: Option<Vec<String>>,
        fields: Option<Vec<String>>,
    ) -> Result<AuditEntryPaging, AlfError> {
        let formatted_url = format!("{}/nodes/{}/audit-entries", self.base_url, node_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let params = ListAuditEntriesForNodeQueryParams {
            skip_count:skip_count.unwrap_or(0),
            order_by,
            max_items: max_items.unwrap_or(100),
            where_filter,
            include,
            fields,
        };

        let raw_response = self.client.get(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params)
            .send()
            .await?;

        let parsed = raw_response.json::<AuditEntryPaging>().await?;

        Ok(parsed)
    }
}