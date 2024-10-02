use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct WithdrawRequest {
    pub amount: f64,
    pub currency: String,
    pub destination: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct WithdrawResponse {
    pub status: u16,
    pub message: String,
    pub data: WithdrawData,
}

#[derive(Deserialize, Debug)]
pub struct WithdrawData {
    pub id: String,
    pub amount_sats: u64,
    pub recipient: String,
    pub fee_sats: f64,
    pub network_type: String,
    pub rail_type: String,
    pub status: String,
    pub lightning_payment: Option<LNDTrackPaymentResponse>,
    pub chain_tx_id: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct EstimateWithdrawalRequest {
    pub amount: f64,
    pub currency: String,
    pub destination: String,
}

#[derive(Deserialize, Debug)]
pub struct EstimateWithdrawalResponse {
    pub status: u16,
    pub message: String,
    pub data: EstimateWithdrawalData,
}

#[derive(Deserialize, Debug)]
pub struct EstimateWithdrawalData {
    pub recipient: String,
    pub recipient_type: String,
    pub amount_sats: u64,
    pub bitvora_fee_sats: f64,
    pub success_probability: f64,
}

#[derive(Serialize)]
pub struct CreateLightningInvoiceRequest {
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub expiry_seconds: u64,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct CreateLightningInvoiceResponse {
    pub status: u16,
    pub message: String,
    pub data: CreateLightningInvoiceData,
}

#[derive(Deserialize, Debug)]
pub struct CreateLightningInvoiceData {
    pub id: String,
    pub node_id: String,
    pub memo: String,
    pub r_preimage: String,
    pub r_hash: String,
    pub amount_sats: u64,
    pub settled: bool,
    pub payment_request: String,
    pub metadata: Option<HashMap<String, String>>,
    pub lightning_address_id: Option<String>,
}

#[derive(Serialize)]
pub struct CreateLightningAddressRequest {
    pub handle: String,
    pub domain: String,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct CreateLightningAddressResponse {
    pub status: u16,
    pub message: String,
    pub data: CreateLightningAddressData,
}

#[derive(Deserialize, Debug)]
pub struct CreateLightningAddressData {
    pub id: String,
    pub handle: String,
    pub domain: String,
    pub address: String,
    pub metadata: Option<HashMap<String, String>>,
    pub created_at: String,
    pub last_used_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Serialize)]
pub struct CreateOnChainAddressRequest {
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct CreateOnChainAddressResponse {
    pub status: u16,
    pub message: String,
    pub data: CreateOnChainAddressData,
}

#[derive(Deserialize, Debug)]
pub struct CreateOnChainAddressData {
    pub id: String,
    pub address: String,
    pub metadata: Option<HashMap<String, String>>,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct GetDepositResponse {
    pub status: u16,
    pub message: String,
    pub data: GetDepositData,
}

#[derive(Deserialize, Debug)]
pub struct GetDepositData {
    pub id: String,
    pub ledger_tx_id: String,
    pub recipient: String,
    pub amount_sats: u64,
    pub fee_sats: f64,
    pub chain_tx_id: Option<String>,
    pub rail_type: String,
    pub network_type: String,
    pub status: String,
    pub metadata: Option<HashMap<String, String>>,
    pub lightning_invoice_id: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct GetBalanceResponse {
    pub status: u16,
    pub message: String,
    pub data: GetBalanceData,
}

#[derive(Deserialize, Debug)]
pub struct GetBalanceData {
    pub balance: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LNDTrackPaymentResponse {
    pub payment_hash: String,
    pub value: String,
    pub creation_date: String,
    pub fee: String,
    pub payment_preimage: String,
    pub value_sat: String,
    pub value_msat: String,
    pub payment_request: String,
    pub status: String,
    pub fee_sat: String,
    pub fee_msat: String,
    pub creation_time_ns: String,
    pub htlcs: Vec<LNDHTLCAttempt>,
    pub payment_index: String,
    pub failure_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LNDHTLCAttempt {
    pub attempt_id: String,
    pub status: String,
    pub route: LNDPaymentRoute,
    pub attempt_time_ns: String,
    pub resolve_time_ns: String,
    pub failure: LNDPaymentFailure,
    pub preimage: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LNDPaymentRoute {
    pub total_time_lock: i32,
    pub total_fees: String,
    pub total_fees_msat: String,
    pub total_amt: String,
    pub hops: Vec<LNDHop>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LNDHop {
    pub chan_id: String,
    pub chan_capacity: String,
    pub amt_to_forward: String,
    pub expiry: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LNDPaymentFailure {
    pub code: String,
    pub channel_update: LNDChannelUpdate,
    pub htlc_msat: String,
    pub onion_sha_256: String,
    pub cltv_expiry: i32,
    pub flags: i32,
    pub failure_source_index: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LNDChannelUpdate {
    pub signature: String,
    pub chain_hash: String,
    pub chan_id: String,
    pub timestamp: i32,
    pub message_flags: i32,
    pub channel_flags: i32,
    pub time_lock_delta: i32,
    pub htlc_minimum_msat: String,
    pub base_fee: i32,
    pub fee_rate: i32,
    pub htlc_maximum_msat: String,
    pub extra_opaque_data: String,
}

#[derive(Deserialize, Debug)]
pub struct GetTransactionsResponse {
    pub status: u16,
    pub message: String,
    pub data: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub company_id: String,
    pub amount_sats: u64,
    pub recipient: String,
    pub rail_type: String,
    pub r#type: String, // `type` is a reserved keyword in Rust, so we prefix it with `r#`
    pub fee_microsats: u64,
    pub status: String,
    pub created_at: String,
}
