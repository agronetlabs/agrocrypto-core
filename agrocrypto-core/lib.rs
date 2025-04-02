pub fn tokenize_asset(asset_id: &str, amount: u64) -> String {
    format!("Asset {} tokenized with amount {}", asset_id, amount)
}

pub fn validate_carbon_credit(doc_hash: &str) -> bool {
    // validação simplificada
    doc_hash.len() == 64
}
