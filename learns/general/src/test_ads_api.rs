use chrono::{DateTime, Utc};
use oauth_client::Token;
use oauth_client::Url;
use oauth_client::Token;
use reqwest::header::ACCEPT;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;

// --- OAuth1 Credentials ---
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OAuthCredentials {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
}

fn create_oauth_token(credentials: &OAuthCredentials) -> (Token, Token) {
    let consumer = Token::new(
        credentials.consumer_key.clone(),
        credentials.consumer_secret.clone(),
    );
    let access = Token::new(
        credentials.access_token.clone(),
        credentials.access_token_secret.clone(),
    );
    (consumer, access)
}

const X_ADS_API_BASE_URL: &str = "https://ads-api.x.com/12/";

pub async fn main() {
    let credentials = OAuthCredentials {
        consumer_key: "your_consumer_key".to_string(),
        consumer_secret: "your_consumer_secret".to_string(),
        access_token: "your_access_token".to_string(),
        access_token_secret: "your_access_token_secret".to_string(),
    };
    let (consumer_token, access_token) = create_oauth_token(&credentials);
    let ads_account_id = "your_ads_account_id";
    let campaign_id = "your_campaign_id";
    let new_status = "ACTIVE"; // or "PAUSED", "ARCHIVED", etc.
        
    let url = Url::parse(X_ADS_API_BASE_URL)?.join(&format!(
        "accounts/{}/campaigns/{}?entity_status={}",
        ads_account_id, campaign_id, new_status
    ))?;

    let client = Client::new();

    let response = client
        .put(url)
        .header("Authorization", auth_header)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    if response.status().is_success() {
        let campaign_response = response.json::<SingleCampaignResponse>().await?;
        println!(
            "Campaign status updated successfully: {:?}",
            campaign_response.data
        );

    } else {
        println!(
            "Failed to update campaign status: {}",
            response.status()
        );
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error body".to_string());
        println!("Error body: {}", body);
        
    }

}
