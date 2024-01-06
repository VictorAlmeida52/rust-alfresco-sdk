use reqwest::Client;
use alfresco_sdk::alfresco::authentication_api::AuthenticationApi;
use alfresco_sdk::models::enums::TicketValidationResult;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let alf_url = std::env::var("ALFRESCO_URL").expect("ALFRESCO_URL must be set");
    let alf_username = std::env::var("ALFRESCO_USERNAME").expect("ALFRESCO_USERNAME must be set");
    let alf_password = std::env::var("ALFRESCO_PASSWORD").expect("ALFRESCO_PASSWORD must be set");

    let auth_api = AuthenticationApi::new(&alf_url, Client::new());

    let auth_response =
        match auth_api.login(&alf_username, &alf_password).await
        {
            Ok(response) => response,
            Err(error) => {
                eprintln!("{:?}", error);
                std::process::exit(1);
            }
        };

    let ticket = auth_response.clone().entry.id.unwrap_or("INVALID_TICKET".to_string());
    let auth_validation_response =
        match auth_api.validate_ticket(&ticket).await
        {
            Ok(TicketValidationResult::Valid(response)) => response,
            Err(error) => {
                eprintln!("{:?}", error);
                std::process::exit(1);
            }
            _ => {
                eprintln!("Erro inesperado. Alguma coisa deu MUITO errado.");
                std::process::exit(1);
            }
        };

    println!("Login: {:?}", auth_response.clone());
    println!("Validate ticket: {:?}", auth_validation_response.clone());

    match auth_api.logout(&ticket).await {
        Ok(_) => {
            print!("Logout realizado com sucesso. Confirmando... ");
            match auth_api.validate_ticket(&ticket).await {
                Ok(TicketValidationResult::Valid(_)) => {
                    println!("Something went wrong, the ticket is still valid even after logout.");
                    std::process::exit(1);
                },
                Ok(TicketValidationResult::InvalidTicket) => {
                    println!("Logged out successfully!");
                    std::process::exit(0);
                },
                Err(error) => {
                    eprintln!("{:?}", error);
                    std::process::exit(1);
                }
            }

        },
        Err(error) => {
            eprintln!("{:?}", error);
            std::process::exit(1);
        }
    };
}