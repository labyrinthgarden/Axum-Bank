use std::sync::Arc;
use crate::application::use_cases::create_account::CreateAccountUseCase;
use crate::application::use_cases::get_account::GetAccountUseCase;
use crate::application::use_cases::deposit::DepositUseCase;
use crate::application::use_cases::transfer::TransferUseCase;

#[derive(Clone)]
pub struct AppState {
  pub create_account: Arc<CreateAccountUseCase>,
  pub get_account: Arc<GetAccountUseCase>,
  pub deposit: Arc<DepositUseCase>,
  pub transfer: Arc<TransferUseCase>
}
