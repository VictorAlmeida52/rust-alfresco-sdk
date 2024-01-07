use reqwest::{Client};
use alfresco_sdk::alfresco::params::QueryParamsBuilder;


#[tokio::main]
async fn main() {
    let query_params = QueryParamsBuilder::new()
        .push_field(Some("Field 1"))
        .set_skip_count(Some(&0))
        .set_max_items(Some(&100))
        .push_field(Some("Field 2"))
        .build();

    let client = Client::new();
    let res = client.get("https://webhook.site/2e513f78-2e3d-4c8b-b585-a334d9f23693")
        .query(&query_params)
        .send();

    match res.await  {
        Ok(response) => {

            match response.text().await {
                Ok(text) => {
                    println!("{}", text);
                }
                Err(error) => {
                    println!("Erro 2: {}", error)
                }
            }

        }
        Err(error) => {
            println!("Erro 1: {}", error)
        }
    }


    // dotenv::dotenv().ok();
    // let alf_url = std::env::var("ALFRESCO_URL").expect("ALFRESCO_URL must be set");
    // let alf_username = std::env::var("ALFRESCO_USERNAME").expect("ALFRESCO_USERNAME must be set");
    // let alf_password = std::env::var("ALFRESCO_PASSWORD").expect("ALFRESCO_PASSWORD must be set");
    //
    // let auth_api = AuthenticationApi::new(&alf_url, Client::new());
    //
    // let auth_response =
    //     match auth_api.login(&alf_username, &alf_password).await
    //     {
    //         Ok(response) => response,
    //         Err(error) => {
    //             eprintln!("{:?}", error);
    //             std::process::exit(1);
    //         }
    //     };
    //
    // let ticket = auth_response.clone().entry.id.unwrap_or("INVALID_TICKET".to_string());
    // let auth_validation_response =
    //     match auth_api.validate_ticket(&ticket).await
    //     {
    //         Ok(TicketValidationResult::Valid(response)) => response,
    //         Err(error) => {
    //             eprintln!("{:?}", error);
    //             std::process::exit(1);
    //         }
    //         _ => {
    //             eprintln!("Erro inesperado. Alguma coisa deu MUITO errado.");
    //             std::process::exit(1);
    //         }
    //     };
    //
    // println!("Login: {:?}", auth_response.clone());
    // println!("Validate ticket: {:?}", auth_validation_response.clone());
    //
    // match auth_api.logout(&ticket).await {
    //     Ok(_) => {
    //         print!("Logout realizado com sucesso. Confirmando... ");
    //         match auth_api.validate_ticket(&ticket).await {
    //             Ok(TicketValidationResult::Valid(_)) => {
    //                 println!("Something went wrong, the ticket is still valid even after logout.");
    //                 std::process::exit(1);
    //             },
    //             Ok(TicketValidationResult::InvalidTicket) => {
    //                 println!("Logged out successfully!");
    //                 std::process::exit(0);
    //             },
    //             Err(error) => {
    //                 eprintln!("{:?}", error);
    //                 std::process::exit(1);
    //             }
    //         }
    //
    //     },
    //     Err(error) => {
    //         eprintln!("{:?}", error);
    //         std::process::exit(1);
    //     }
    // };
}