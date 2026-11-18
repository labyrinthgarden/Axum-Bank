use std::sync::Arc;
use crate::application::use_cases::create_account::CreateAccountUseCase;
use crate::application::use_cases::get_account::GetAccountUseCase;

#[derive(Clone)]
pub struct AppState {
  pub create_account: Arc<CreateAccountUseCase>,
  pub get_account: Arc<GetAccountUseCase>
}
