CREATE TABLE accounts (
  id UUID PRIMARY KEY,
  account_number TEXT NOT NULL UNIQUE,
  currency CHAR(3) NOT NULL,
  balance NUMERIC(20, 4) NOT NULL DEFAULT 0,
  status TEXT NOT NULL DEFAULT 'active',
  type TEXT NOT NULL DEFAULT 'checking',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT accounts_balance_non_negative CHECK (balance >= 0),
  CONSTRAINT accounts_currency_iso CHECK (currency ~ '^[A-Z]{3}$'),
  CONSTRAINT accounts_status_valid CHECK (status IN ('active', 'frozen', 'closed')),
  CONSTRAINT accounts_type_valid CHECK (type IN ('checking', 'savings', 'investment', 'credit'))
);
