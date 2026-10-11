CREATE TABLE account_holders (
  account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
  role TEXT NOT NULL DEFAULT 'owner',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (account_id, user_id),
  CONSTRAINT account_holders_role_valid CHECK (role IN ('owner', 'co_owner', 'beneficiary', 'tutor'))
);
