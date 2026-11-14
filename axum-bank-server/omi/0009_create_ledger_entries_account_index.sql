CREATE INDEX idx_ledger_entries_account_created ON ledger_entries (account_id, created_at DESC);
