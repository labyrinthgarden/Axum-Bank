#[derive(Clone, PartialEq, Eq)]
pub enum Route {
    Root,
    Login,
    Dashboard,
    Accounts,
    CreateAccount,
    Transactions,
    Deposit,
    Withdraw,
    Transfer,
    NotFound,
}

impl Route {
    pub fn from_hash(hash: &str) -> Self {
        match hash {
            "" | "#" | "#/" => Self::Root,
            "#/login" => Self::Login,
            "#/dashboard" => Self::Dashboard,
            "#/accounts" => Self::Accounts,
            "#/accounts/new" => Self::CreateAccount,
            "#/transactions" => Self::Transactions,
            "#/operations/deposit" => Self::Deposit,
            "#/operations/withdraw" => Self::Withdraw,
            "#/operations/transfer" => Self::Transfer,
            _ => Self::NotFound,
        }
    }

    pub fn fragment(&self) -> &'static str {
        match self {
            Self::Root => "/",
            Self::Login => "/login",
            Self::Dashboard => "/dashboard",
            Self::Accounts => "/accounts",
            Self::CreateAccount => "/accounts/new",
            Self::Transactions => "/transactions",
            Self::Deposit => "/operations/deposit",
            Self::Withdraw => "/operations/withdraw",
            Self::Transfer => "/operations/transfer",
            Self::NotFound => "/404",
        }
    }

    pub fn href(&self) -> String {
        format!("#{}", self.fragment())
    }

    pub fn requires_auth(&self) -> bool {
        matches!(
            self,
            Self::Dashboard
                | Self::Accounts
                | Self::CreateAccount
                | Self::Transactions
                | Self::Deposit
                | Self::Withdraw
                | Self::Transfer
                | Self::NotFound
        )
    }
}
