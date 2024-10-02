# Bitvora Rust SDK

This README provides a comprehensive guide on how to use the Bitvora Rust SDK to interact with the Bitvora API. This SDK simplifies common tasks such as managing withdrawals, creating invoices, and retrieving account balances.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenv = "0.15"
```

## Usage

This SDK utilizes the `reqwest` crate for HTTP requests and `serde` for JSON serialization/deserialization. Ensure you have these dependencies installed.

Before using the SDK, you need to obtain your Bitvora API key from your Bitvora account dashboard.

### 1. Initialization

```rust
use bitvora_client::{BitvoraClient, WithdrawRequest}; // Assuming your client is in a module called bitvora_client
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load environment variables from .env file

    let api_key = env::var("BITVORA_API_KEY").expect("BITVORA_API_KEY must be set");
    let client = BitvoraClient::new("https://api.signet.bitvora.com", &api_key); // Replace with your API endpoint

    // ... your code here ...

    Ok(())
}
```

Remember to replace `"https://api.signet.bitvora.com"` with your actual Bitvora API endpoint. Create a `.env` file in the same directory as your main application file to store your API key:

```
BITVORA_API_KEY=your_actual_api_key
```

### 2. Key API Methods

The SDK provides asynchronous methods for various API endpoints:

#### 2.1 Withdraw Funds

```rust
use bitvora_client::{BitvoraClient, WithdrawRequest, WithdrawResponse};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (Initialization as above) ...

    let mut metadata = HashMap::new();
    metadata.insert("userID".to_string(), "1234".to_string());

    let request = WithdrawRequest {
        amount: 21.0,
        currency: "sats".to_string(),
        destination: "your_destination_address",
        metadata,
    };

    let response: WithdrawResponse = client.withdraw(request).await?;

    println!("Withdrawal Status: {}", response.status);
    println!("Withdrawal Message: {}", response.message);
    println!("Withdrawal ID: {}", response.data.id);
    println!("Withdrawal Amount (sats): {}", response.data.amount_sats);
    println!("Withdrawal Recipient: {}", response.data.recipient);
    println!("Withdrawal Fee (sats): {}", response.data.fee_sats);
    // Access other fields in response.data as needed...


    Ok(())
}
```

#### 2.2 Estimate Withdrawal Fees

```rust
use bitvora_client::{BitvoraClient, EstimateWithdrawalRequest, EstimateWithdrawalResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (Initialization as above) ...

    let request = EstimateWithdrawalRequest {
        amount: 21.0,
        currency: "sats".to_string(),
        destination: "your_destination_address",
    };

    let response: EstimateWithdrawalResponse = client.estimate_withdrawal(request).await?;

    println!("Estimate Status: {}", response.status);
    println!("Estimate Message: {}", response.message);
    println!("Recipient: {}", response.data.recipient);
    println!("Amount (sats): {}", response.data.amount_sats);
    println!("Bitvora Fee (sats): {}", response.data.bitvora_fee_sats);
    //Access other fields as needed

    Ok(())
}
```

#### 2.3 Create Lightning Invoice

```rust
use bitvora_client::{BitvoraClient, CreateLightningInvoiceRequest, CreateLightningInvoiceResponse};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (Initialization as above) ...

    let request = CreateLightningInvoiceRequest {
        amount: 1000.0,
        currency: "sats".to_string(),
        description: "Test Invoice".to_string(),
        expiry_seconds: 3600,
        metadata: None,
    };

    let response: CreateLightningInvoiceResponse = client.create_lightning_invoice(request).await?;

    println!("Invoice Creation Status: {}", response.status);
    println!("Invoice Creation Message: {}", response.message);
    println!("Invoice Payment Request: {}", response.data.payment_request);
    //Access other fields as needed

    Ok(())
}
```

#### 2.4 Create Lightning Address

```rust
use bitvora_client::{BitvoraClient, CreateLightningAddressRequest, CreateLightningAddressResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (Initialization as above) ...

    let request = CreateLightningAddressRequest {
        handle: "your_handle".to_string(),
        domain: "bitvora.com".to_string(), //or your custom domain
        metadata: None,
    };

    let response: CreateLightningAddressResponse = client.create_lightning_address(request).await?;

    println!("Lightning Address Creation Status: {}", response.status);
    println!("Lightning Address Creation Message: {}", response.message);
    println!("Lightning Address: {}", response.data.address);
    //Access other fields as needed

    Ok(())
}
```

#### 2.5 Create On-Chain Address

```rust
use bitvora_client::{BitvoraClient, CreateOnChainAddressRequest, CreateOnChainAddressResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (Initialization as above) ...

    let request = CreateOnChainAddressRequest { metadata: None };

    let response: CreateOnChainAddressResponse = client.create_onchain_address(request).await?;

    println!("On-Chain Address Creation Status: {}", response.status);
    println!("On-Chain Address Creation Message: {}", response.message);
    println!("On-Chain Address: {}", response.data.address);
    //Access other fields as needed

    Ok(())
}
```

#### 2.6 Get Withdrawal Details

```rust
use bitvora_client::{BitvoraClient, WithdrawResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (Initialization as above) ...

    let withdrawal_id = "your_withdrawal_id";
    let response: WithdrawResponse = client.get_withdrawal(withdrawal_id).await?;

    println!("Withdrawal Status: {}", response.status);
    println!("Withdrawal Message: {}", response.message);
    println!("Withdrawal ID: {}", response.data.id);
    // Access other fields in response.data as needed

    Ok(())
}
```

#### 2.7 Get Deposit Details

```rust
use bitvora_client::{BitvoraClient, GetDepositResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (Initialization as above) ...

    let deposit_id = "your_deposit_id";
    let response: GetDepositResponse = client.get_deposit(deposit_id).await?;

    println!("Deposit Status: {}", response.status);
    println!("Deposit Message: {}", response.message);
    println!("Deposit ID: {}", response.data.id);
    // Access other fields in response.data as needed

    Ok(())
}
```

#### 2.8 Get Balance

```rust
use bitvora_client::{BitvoraClient, GetBalanceResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (Initialization as above) ...

    let response: GetBalanceResponse = client.get_balance().await?;

    println!("Balance Status: {}", response.status);
    println!("Balance Message: {}", response.message);
    println!("Balance: {}", response.data.balance);

    Ok(())
}
```

#### 2.9 Get Transactions

```rust
use bitvora_client::{BitvoraClient, GetTransactionsResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (Initialization as above) ...

    let response: GetTransactionsResponse = client.get_transactions().await?;

    println!("Transactions Status: {}", response.status);
    println!("Transactions Message: {}", response.message);

    for transaction in response.data {
        println!("Transaction ID: {}", transaction.id);
        println!("Transaction Type: {}", transaction.r#type);
        println!("Amount (sats): {}", transaction.amount_sats);
        // Access other transaction fields as needed
    }

    Ok(())
}
```

### 3. Error Handling

The SDK uses a custom `APIError` enum to handle potential errors:

```rust
// ... within your bitvora_client module ...

#[derive(Debug)]
pub enum APIError {
    Reqwest(reqwest::Error),
    Serialization(serde_json::Error),
    BadRequest(reqwest::StatusCode, String),
    Other(String),
}
```

Remember to handle these errors appropriately in your application.

## Live Tests (Requires Environment Variables)

The provided `live_tests` module demonstrates how to use the SDK in a testing environment. **Remember to replace placeholder values with your actual data and API key.** It also requires the `dotenv` crate to load environment variables.

This comprehensive guide should help you effectively utilize the Bitvora Rust SDK. Remember to consult the Bitvora API documentation for detailed information on each endpoint and its parameters.
