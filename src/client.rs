use crate::errors::APIError;
use crate::models::{WithdrawRequest, WithdrawResponse};
use crate::{
    CreateLightningAddressRequest, CreateLightningAddressResponse, CreateLightningInvoiceRequest,
    CreateLightningInvoiceResponse, CreateOnChainAddressRequest, CreateOnChainAddressResponse,
    EstimateWithdrawalRequest, EstimateWithdrawalResponse, GetBalanceResponse, GetDepositResponse,
    GetTransactionsResponse,
};
use reqwest::Client as HttpClient;

pub struct BitvoraClient {
    base_url: String,
    api_key: String,
    client: HttpClient,
}

impl BitvoraClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            client: HttpClient::new(),
        }
    }

    pub async fn withdraw(&self, request: WithdrawRequest) -> Result<WithdrawResponse, APIError> {
        let url = format!("{}/v1/bitcoin/withdraw/confirm", self.base_url);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request) // Send the request body as JSON
            .send()
            .await?;

        let status = response.status();
        let raw_body = response.text().await?;

        if status.is_success() {
            match serde_json::from_str::<WithdrawResponse>(&raw_body) {
                Ok(withdraw_response) => Ok(withdraw_response),
                Err(err) => {
                    println!(
                        "Failed to deserialize response: {}\nRaw body: {}",
                        err, raw_body
                    );
                    Err(APIError::Serialization(err))
                }
            }
        } else {
            Err(APIError::BadRequest(status, raw_body))
        }
    }

    pub async fn estimate_withdrawal(
        &self,
        request: EstimateWithdrawalRequest,
    ) -> Result<EstimateWithdrawalResponse, APIError> {
        let url = format!("{}/v1/bitcoin/withdraw/estimate", self.base_url);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request) // Send the request body as JSON
            .send()
            .await?;

        let status = response.status();
        let raw_body = response.text().await?;

        if status.is_success() {
            match serde_json::from_str::<EstimateWithdrawalResponse>(&raw_body) {
                Ok(estimate_response) => Ok(estimate_response),
                Err(err) => {
                    println!(
                        "Failed to deserialize response: {}\nRaw body: {}",
                        err, raw_body
                    );
                    Err(APIError::Serialization(err))
                }
            }
        } else {
            // Capture the full response body for debugging
            Err(APIError::BadRequest(status, raw_body))
        }
    }

    pub async fn create_lightning_invoice(
        &self,
        request: CreateLightningInvoiceRequest,
    ) -> Result<CreateLightningInvoiceResponse, APIError> {
        let url = format!("{}/v1/bitcoin/deposit/lightning-invoice", self.base_url);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request) // Send the request body as JSON
            .send()
            .await?;

        let status = response.status();
        let raw_body = response.text().await?;

        if status.is_success() {
            match serde_json::from_str::<CreateLightningInvoiceResponse>(&raw_body) {
                Ok(invoice_response) => Ok(invoice_response),
                Err(err) => {
                    println!(
                        "Failed to deserialize response: {}\nRaw body: {}",
                        err, raw_body
                    );
                    Err(APIError::Serialization(err))
                }
            }
        } else {
            // Capture the full response body for debugging
            Err(APIError::BadRequest(status, raw_body))
        }
    }

    pub async fn create_lightning_address(
        &self,
        request: CreateLightningAddressRequest,
    ) -> Result<CreateLightningAddressResponse, APIError> {
        let url = format!("{}/v1/bitcoin/deposit/lightning-address", self.base_url);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request) // Send the request body as JSON
            .send()
            .await?;

        let status = response.status();
        let raw_body = response.text().await?;

        if status.is_success() {
            match serde_json::from_str::<CreateLightningAddressResponse>(&raw_body) {
                Ok(invoice_response) => Ok(invoice_response),
                Err(err) => {
                    println!(
                        "Failed to deserialize response: {}\nRaw body: {}",
                        err, raw_body
                    );
                    Err(APIError::Serialization(err))
                }
            }
        } else {
            // Capture the full response body for debugging
            Err(APIError::BadRequest(status, raw_body))
        }
    }

    pub async fn create_onchain_address(
        &self,
        request: CreateOnChainAddressRequest,
    ) -> Result<CreateOnChainAddressResponse, APIError> {
        let url = format!("{}/v1/bitcoin/deposit/on-chain", self.base_url);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request) // Send the request body as JSON
            .send()
            .await?;

        let status = response.status();
        let raw_body = response.text().await?;

        if status.is_success() {
            match serde_json::from_str::<CreateOnChainAddressResponse>(&raw_body) {
                Ok(invoice_response) => Ok(invoice_response),
                Err(err) => {
                    println!(
                        "Failed to deserialize response: {}\nRaw body: {}",
                        err, raw_body
                    );
                    Err(APIError::Serialization(err))
                }
            }
        } else {
            // Capture the full response body for debugging
            Err(APIError::BadRequest(status, raw_body))
        }
    }

    pub async fn get_withdrawal(&self, id: &str) -> Result<WithdrawResponse, APIError> {
        let url = format!("{}/v1/transactions/withdrawals/{}", self.base_url, id);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        let status = response.status();
        let raw_body = response.text().await?;

        if status.is_success() {
            match serde_json::from_str::<WithdrawResponse>(&raw_body) {
                Ok(withdraw_response) => Ok(withdraw_response),
                Err(err) => {
                    println!(
                        "Failed to deserialize response: {}\nRaw body: {}",
                        err, raw_body
                    );
                    Err(APIError::Serialization(err))
                }
            }
        } else {
            Err(APIError::BadRequest(status, raw_body))
        }
    }

    pub async fn get_deposit(&self, id: &str) -> Result<GetDepositResponse, APIError> {
        let url = format!("{}/v1/transactions/deposits/{}", self.base_url, id);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        let status = response.status();
        let raw_body = response.text().await?;

        if status.is_success() {
            match serde_json::from_str::<GetDepositResponse>(&raw_body) {
                Ok(withdraw_response) => Ok(withdraw_response),
                Err(err) => {
                    println!(
                        "Failed to deserialize response: {}\nRaw body: {}",
                        err, raw_body
                    );
                    Err(APIError::Serialization(err))
                }
            }
        } else {
            Err(APIError::BadRequest(status, raw_body))
        }
    }

    pub async fn get_balance(&self) -> Result<GetBalanceResponse, APIError> {
        let url = format!("{}/v1/transactions/balance", self.base_url);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        let status = response.status();
        let raw_body = response.text().await?;

        if status.is_success() {
            match serde_json::from_str::<GetBalanceResponse>(&raw_body) {
                Ok(balance_response) => Ok(balance_response),
                Err(err) => {
                    println!(
                        "Failed to deserialize response: {}\nRaw body: {}",
                        err, raw_body
                    );
                    Err(APIError::Serialization(err))
                }
            }
        } else {
            Err(APIError::BadRequest(status, raw_body))
        }
    }

    pub async fn get_transactions(&self) -> Result<GetTransactionsResponse, APIError> {
        let url = format!("{}/v1/transactions", self.base_url);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        let status = response.status();
        let raw_body = response.text().await?;

        if status.is_success() {
            match serde_json::from_str::<GetTransactionsResponse>(&raw_body) {
                Ok(transactions_response) => Ok(transactions_response),
                Err(err) => {
                    println!(
                        "Failed to deserialize response: {}\nRaw body: {}",
                        err, raw_body
                    );
                    Err(APIError::Serialization(err))
                }
            }
        } else {
            Err(APIError::BadRequest(status, raw_body))
        }
    }
}

#[cfg(test)]
mod live_tests {
    use crate::CreateLightningAddressRequest;

    use super::*;
    use dotenv::dotenv;
    use std::{collections::HashMap, env};

    #[tokio::test]
    async fn test_withdraw_live() {
        dotenv().ok();

        let api_key =
            env::var("BITVORA_API_KEY").expect("BITVORA_API_KEY must be set in the environment");

        let client = BitvoraClient::new("https://api.signet.bitvora.com", &api_key);

        // Create request data
        let mut metadata = HashMap::new();
        metadata.insert("userID".to_string(), "1234".to_string());

        let request = WithdrawRequest {
            amount: 21.0,
            currency: "sats".to_string(),
            destination: "sillyzebu667@signet.bitvora.me".to_string(),
            metadata,
        };

        match client.withdraw(request).await {
            Ok(response) => {
                println!("Withdrawal successful: {:?}", response);
                assert_eq!(response.status, 201);
            }
            Err(e) => {
                eprintln!("Error during withdrawal: {}", e);
                panic!("Withdrawal failed!");
            }
        }
    }

    #[tokio::test]
    async fn test_estimate_withdrawal_live() {
        dotenv().ok();

        let api_key =
            env::var("BITVORA_API_KEY").expect("BITVORA_API_KEY must be set in the environment");

        let client = BitvoraClient::new("https://api.signet.bitvora.com", &api_key);

        let request = EstimateWithdrawalRequest {
            amount: 21.0,
            currency: "sats".to_string(),
            destination: "sillyzebu667@signet.bitvora.me".to_string(),
        };

        match client.estimate_withdrawal(request).await {
            Ok(response) => {
                println!("Estimate successful: {:?}", response);
                assert_eq!(response.status, 200);
            }
            Err(e) => {
                eprintln!("Error during estimate: {}", e);
                panic!("Estimate failed!");
            }
        }
    }

    #[tokio::test]
    async fn test_create_lightning_invoice_live() {
        dotenv().ok();

        let api_key =
            env::var("BITVORA_API_KEY").expect("BITVORA_API_KEY must be set in the environment");

        let client = BitvoraClient::new("https://api.signet.bitvora.com", &api_key);

        let request = CreateLightningInvoiceRequest {
            amount: 21.0,
            currency: "sats".to_string(),
            description: "Test invoice".to_string(),
            expiry_seconds: 3600,
            metadata: None,
        };

        match client.create_lightning_invoice(request).await {
            Ok(response) => {
                println!("Invoice created: {:?}", response);
                assert_eq!(response.status, 201);
            }
            Err(e) => {
                eprintln!("Error during invoice creation: {}", e);
                panic!("Invoice creation failed!");
            }
        }
    }

    #[tokio::test]
    async fn test_create_lightning_address_live() {
        dotenv().ok();

        let api_key =
            env::var("BITVORA_API_KEY").expect("BITVORA_API_KEY must be set in the environment");

        let client = BitvoraClient::new("https://api.signet.bitvora.com", &api_key);

        let request = CreateLightningAddressRequest {
            handle: "".to_string(),
            domain: "".to_string(),
            metadata: None,
        };

        match client.create_lightning_address(request).await {
            Ok(response) => {
                println!("Lightning address created: {:?}", response);
                assert_eq!(response.status, 201);
            }
            Err(e) => {
                eprintln!("Error during lightning address creation: {}", e);
                panic!("Lightning address creation failed!");
            }
        }
    }

    #[tokio::test]
    async fn test_create_onchain_address_live() {
        dotenv().ok();

        let api_key =
            env::var("BITVORA_API_KEY").expect("BITVORA_API_KEY must be set in the environment");

        let client = BitvoraClient::new("https://api.signet.bitvora.com", &api_key);

        let request = CreateOnChainAddressRequest { metadata: None };

        match client.create_onchain_address(request).await {
            Ok(response) => {
                println!("On-chain address created: {:?}", response);
                assert_eq!(response.status, 201);
            }
            Err(e) => {
                eprintln!("Error during on-chain address creation: {}", e);
                panic!("On-chain address creation failed!");
            }
        }
    }

    #[tokio::test]
    async fn test_get_withdrawal_live() {
        dotenv().ok();

        let api_key = env::var("BITVORA_API_KEY").expect("BITVORA_API");
        let client = BitvoraClient::new("https://api.signet.bitvora.com", &api_key);

        let withdrawal_id = "0f92dad1-3897-4a7f-bf1f-a00d6aa6a814";

        match client.get_withdrawal(withdrawal_id).await {
            Ok(response) => {
                println!("Withdrawal details: {:?}", response);
                assert_eq!(response.status, 200);
            }
            Err(e) => {
                eprintln!("Error during withdrawal retrieval: {}", e);
                panic!("Withdrawal retrieval failed!");
            }
        }
    }

    #[tokio::test]
    async fn test_get_deposit_live() {
        dotenv().ok();

        let api_key = env::var("BITVORA_API_KEY").expect("BITVORA_API");
        let client = BitvoraClient::new("https://api.signet.bitvora.com", &api_key);

        let deposit_id = "ddef8269-b99d-414b-9696-033ce5c878e1";

        match client.get_deposit(deposit_id).await {
            Ok(response) => {
                println!("Deposit details: {:?}", response);
                assert_eq!(response.status, 200);
            }
            Err(e) => {
                eprintln!("Error during deposit retrieval: {}", e);
                panic!("Deposit retrieval failed!");
            }
        }
    }

    #[tokio::test]
    async fn test_get_balance_live() {
        dotenv().ok();

        let api_key = env::var("BITVORA_API_KEY").expect("BITVORA_API");
        let client = BitvoraClient::new("https://api.signet.bitvora.com", &api_key);

        match client.get_balance().await {
            Ok(response) => {
                println!("Balance details: {:?}", response);
                assert_eq!(response.status, 200);
            }
            Err(e) => {
                eprintln!("Error during balance retrieval: {}", e);
                panic!("Balance retrieval failed!");
            }
        }
    }

    #[tokio::test]
    async fn test_get_transactions_live() {
        dotenv().ok();

        let api_key = env::var("BITVORA_API_KEY").expect("BITVORA_API");
        let client = BitvoraClient::new("https://api.signet.bitvora.com", &api_key);

        match client.get_transactions().await {
            Ok(response) => {
                println!("Transactions: {:?}", response);
                assert_eq!(response.status, 200);
            }
            Err(e) => {
                eprintln!("Error during transactions retrieval: {}", e);
                panic!("Transactions retrieval failed!");
            }
        }
    }
}
