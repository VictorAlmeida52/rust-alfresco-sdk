use reqwest::Client;
use base64::{Engine as _, engine::general_purpose};
use serde_json::json;
use crate::models::structs::{AuditApp, AuditAppPagingList, AuditEntryEntry, AuditEntryPaging};

use anyhow::Result;
use crate::alfresco::params::QueryParamsBuilder;

pub struct AuditApi {
    pub base_url: String,
    pub client: Client,
}

impl AuditApi {
    pub fn new(base_url: &str, client: Client) -> Self {
        AuditApi {
            base_url: format!("{}/alfresco/api/-default-/public/alfresco/versions/1", base_url),
            client,
        }
    }

    pub async fn list_audit_applications(&self, ticket: &str, skip_count: Option<&u32>, max_items: Option<&u32>, fields: Option<&Vec<String>>) -> Result<AuditAppPagingList> {
        let formatted_url = format!("{}/audit-applications", self.base_url);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let mut params_builder = QueryParamsBuilder::new()
            .set_skip_count(skip_count)
            .set_max_items(max_items);

        if let Some(field_list) = fields {
            for field in field_list {
                params_builder.push_field(Some(field));
            }
        }


        let raw_response = self.client.get(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params_builder.build())
            .send()
            .await?;

        let parsed = raw_response.json::<AuditAppPagingList>().await?;

        Ok(parsed)
    }

    pub async fn get_audit_application_info(&self, ticket: &str, audit_application_id: &str, fields: Option<&Vec<String>>, include: Option<&Vec<String>>) -> Result<AuditApp> {
        let formatted_url = format!("{}/audit-applications/{}", self.base_url, audit_application_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let mut params_builder = QueryParamsBuilder::new();

        if let Some(field_list) = fields {
            for field in field_list {
                params_builder.push_field(Some(field));
            }
        }

        if let Some(include_list) = include {
            for include_value in include_list {
                params_builder.push_field(Some(include_value));
            }
        }

        let raw_response = self.client.get(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params_builder.build())
            .send()
            .await?;

        let parsed = raw_response.json::<AuditApp>().await?;

        Ok(parsed)
    }

    pub async fn update_audit_application_info(&self, ticket: &str, audit_application_id: &str, fields: Option<&Vec<String>>, is_enabled: &bool) -> Result<AuditApp> {
        let formatted_url = format!("{}/audit-applications/{}", self.base_url, audit_application_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let mut params_builder = QueryParamsBuilder::new();

        if let Some(field_list) = fields {
            for field in field_list {
                params_builder.push_field(Some(field));
            }
        }

        let body = json!({
            "isEnabled": &is_enabled,
        });

        let raw_response = self.client.put(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params_builder.build())
            .json(&body)
            .send()
            .await?;

        let parsed = raw_response.json::<AuditApp>().await?;

        Ok(parsed)
    }

    pub async fn list_audit_entries_for_audit_app(&self, ticket: &str,audit_application_id: &str, skip_count: Option<&u32>, omit_total_items: Option<&bool>, order_by: Option<&Vec<String>>, max_items: Option<&u32>, where_filter: Option<&String>, include: Option<&Vec<String>>, fields: Option<&Vec<String>>) -> Result<AuditEntryPaging> {
        let formatted_url = format!("{}/audit-applications/{}/audit-entries", self.base_url, audit_application_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let mut params_builder = QueryParamsBuilder::new()
            .set_skip_count(skip_count)
            .set_omit_total_items(omit_total_items)
            .set_max_items(max_items)
            .set_where_filter(where_filter);

        if let Some(order_list) = order_by {
            for order_value in order_list {
                params_builder.push_order_by(Some(order_value));
            }
        }

        if let Some(field_list) = fields {
            for field in field_list {
                params_builder.push_field(Some(field));
            }
        }

        if let Some(include_list) = include {
            for include_value in include_list {
                params_builder.push_include(Some(include_value));
            }
        }

        let raw_response = self.client.get(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params_builder.build())
            .send()
            .await?;

        let parsed = raw_response.json::<AuditEntryPaging>().await?;

        Ok(parsed)
    }

    pub async fn permanently_delete_audit_entries_for_audit_app(&self, ticket: &str, audit_application_id: &str, where_filter: &String, ) -> Result<AuditEntryPaging> {
        let formatted_url = format!("{}/audit-applications/{}/audit-entries", self.base_url, audit_application_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let mut params_builder = QueryParamsBuilder::new()
            .set_where_filter(Some(where_filter));

        let raw_response = self.client.delete(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params_builder.build())
            .send()
            .await?;

        let parsed = raw_response.json::<AuditEntryPaging>().await?;

        Ok(parsed)
    }

    pub async fn get_audit_entry(&self, ticket: &str, audit_application_id: &str, audit_entry_id: &str, fields: Option<&Vec<String>>) -> Result<AuditEntryEntry> {
        let formatted_url = format!("{}/audit-applications/{}/audit-entries/{}", self.base_url, audit_application_id, audit_entry_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let mut params_builder = QueryParamsBuilder::new();

        if let Some(field_list) = fields {
            for field in field_list {
                params_builder.push_field(Some(field));
            }
        }

        let raw_response = self.client.get(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params_builder.build())
            .send()
            .await?;

        let parsed = raw_response.json::<AuditEntryEntry>().await?;

        Ok(parsed)
    }

    pub async fn permanently_delete_audit_entry(&self, ticket: &str, audit_application_id: &str, audit_entry_id: &str) -> Result<String> {
        let formatted_url = format!("{}/audit-applications/{}/audit-entries/{}", self.base_url, audit_application_id, audit_entry_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let raw_response = self.client.delete(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .send()
            .await?;

        let status_code = raw_response.status().as_u16();

        match status_code {
            204 => Ok("Successfully deleted audit entry".to_string()),
            400 => Ok("Invalid parameter: auditApplicationId  or auditEntryId is not a valid format".to_string()),
            401 => Ok("Authentication failed".to_string()),
            403 => Ok("Current user does not have permission to delete audit information".to_string()),
            404 => Ok("auditApplicationId or auditEntryId does not exist".to_string()),
            501 => Ok("Audit is disabled for the system".to_string()),
            _ => Err(anyhow::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "Something went wrong and I don't know what it is")))
        }
    }

    pub async fn list_audit_entries_for_node(&self, ticket: &str, node_id: &str, skip_count: Option<&u32>, order_by: Option<&Vec<String>>, max_items: Option<&u32>, where_filter: Option<&String>, include: Option<&Vec<String>>, fields: Option<&Vec<String>>) -> Result<AuditEntryPaging> {
        let formatted_url = format!("{}/nodes/{}/audit-entries", self.base_url, node_id);
        let ticket_b64 = general_purpose::STANDARD.encode(ticket);

        let mut params_builder = QueryParamsBuilder::new()
            .set_skip_count(skip_count)
            .set_max_items(max_items)
            .set_where_filter(where_filter);


        if let Some(order_list) = order_by {
            for order_value in order_list {
                params_builder.push_order_by(Some(order_value));
            }
        }

        if let Some(field_list) = fields {
            for field in field_list {
                params_builder.push_field(Some(field));
            }
        }

        if let Some(include_list) = include {
            for include_value in include_list {
                params_builder.push_include(Some(include_value));
            }
        }

        let raw_response = self.client.get(&formatted_url)
            .header("Authorization", format!("Basic {}", ticket_b64))
            .query(&params_builder.build())
            .send()
            .await?;

        let parsed = raw_response.json::<AuditEntryPaging>().await?;

        Ok(parsed)
    }
}