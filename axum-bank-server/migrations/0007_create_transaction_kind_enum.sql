CREATE TYPE transaction_kind AS ENUM (
  'initial_deposit',
  'deposit',
  'withdrawal',
  'transfer_in',
  'transfer_out'
);
