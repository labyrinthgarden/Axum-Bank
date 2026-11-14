pub fn prettify_kind(kind: &str) -> &'static str {
    match kind {
        "initial_deposit" => "Initial Deposit",
        "deposit" => "Deposit",
        "withdrawal" => "Withdrawal",
        "transfer_in" => "Transfer In",
        "transfer_out" => "Transfer Out",
        _ => "Transaction",
    }
}

pub fn human_time(raw: &str) -> String {
    raw.replace('T', " ").replace('Z', " UTC")
}

pub fn format_money(raw: &str) -> String {
    format!("${raw}")
}
