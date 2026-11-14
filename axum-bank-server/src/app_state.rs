use std::sync::Arc;
use crate::application::use_cases::create_account::CreateAccountUseCase;

#[derive(Clone)]
pub struct AppState {
  pub create_account: Arc<CreateAccountUseCase>
}
