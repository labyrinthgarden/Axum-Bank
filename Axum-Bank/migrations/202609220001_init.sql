CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY,
    owner_name VARCHAR(120) NOT NULL,
    balance NUMERIC(14,2) NOT NULL CHECK (balance >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    kind VARCHAR(32) NOT NULL CHECK (
        kind IN ('initial_deposit', 'deposit', 'withdrawal', 'transfer_in', 'transfer_out')
    ),
    amount NUMERIC(14,2) NOT NULL CHECK (amount > 0),
    counterparty_account_id UUID,
    description VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_transactions_account_created_at
    ON transactions(account_id, created_at DESC);
