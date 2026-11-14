CREATE TABLE ledger_entries (
  id UUID PRIMARY KEY,
  correlation_id UUID NOT NULL,
  account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
  kind transaction_kind NOT NULL,
  amount NUMERIC(20, 4) NOT NULL,
  currency CHAR(3) NOT NULL,
  running_balance NUMERIC(20, 4) NOT NULL,
  counterparty_account_id UUID REFERENCES accounts(id) ON DELETE RESTRICT,
  description TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT ledger_entries_amount_positive CHECK (amount > 0),
  CONSTRAINT ledger_entries_currency_iso CHECK (currency ~ '^[A-Z]{3}$'),
  CONSTRAINT ledger_entries_description_length CHECK (description IS NULL OR length(description) <= 255)
);
