use reqwest::Client;
use crate::models::structs::{TicketBodyCreate, TicketEntry, ValidTicketEntry};
use base64::{Engine as _, engine::general_purpose};
use crate::models::enums::TicketValidationResult;

pub struct AuthenticationApi {
    pub base_url: String,
    pub client: Client,
}

impl AuthenticationApi {
    pub fn new(base_url: &str, client: Client) -> Self {
        AuthenticationApi {
            base_url: format!("{}/alfresco/api/-default-/public/authentication/versions/1", base_url),
            client,
        }
    }

    pub async fn login(&self, user_id: &str, password: &str) -> Result<TicketEntry, anyhow::Error> {
        let formatted_url = format!("{}/tickets", self.base_url);

        let body = TicketBodyCreate {
            user_id: Some(user_id.to_string()),
            password: Some(password.to_string()),
        };

        let raw_response = self.client.post(&formatted_url)
            .json(&body)
            .send()
            .await?;

        let parsed = raw_response.json::<TicketEntry>().await?;

        Ok(parsed)
    }

    pub async fn validate_ticket(&self, ticket: &str) -> Result<TicketValidationResult, anyhow::Error> {
        let formatted_url = format!("{}/tickets/-me-", self.base_url);

        let b64 = general_purpose::STANDARD.encode(ticket);
        let raw_response = self.client.get(formatted_url)
            .header("Authorization", &format!("Basic {}", b64))
            .send()
            .await?;

        let status_code = raw_response.status().as_u16();

        match status_code {
            404 | 401 => Ok(TicketValidationResult::InvalidTicket),
            _ => {
                let parsed = raw_response.json::<ValidTicketEntry>().await?;
                Ok(TicketValidationResult::Valid(parsed))
            },
        }
    }

    pub async fn logout(&self, ticket: &str) -> Result<(), anyhow::Error> {
        let formatted_url = format!("{}/tickets/-me-", self.base_url);

        let b64 = general_purpose::STANDARD.encode(ticket);
        self.client.delete(formatted_url)
            .header("Authorization", &format!("Basic {}", b64))
            .send()
            .await?;

        Ok(())
    }
}