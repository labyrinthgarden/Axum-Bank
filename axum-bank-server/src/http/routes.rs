use axum::{extract::{Path, State},http::StatusCode,routing::{get, post},Json,Router,};
use uuid::Uuid;
use crate::{
  state::AppState,
  error::AppError,
  http::{
    requests::{CreateUserRequest,CreateAccountRequest,DepositMoneyRequest,WithdrawMoneyRequest},
    responses::{CreateUserResponse,CreateAccountResponse,DepositMoneyResponse,WithdrawMoneyResponse,UserResponse},
  },
  commands::{self,
    create_user::CreateUserCommand,
    create_account::{AccountType, CreateAccountCommand},
    deposit_money::DepositMoneyCommand,
    withdraw_money::WithdrawMoneyCommand
  },
  queries::{self,
    get_user::GetUserQuery
  },
};

pub fn router()->Router<AppState>{
  Router::new()
  .route("/users",post(create_user_handler))
  .route("/users/{id}",get(get_user_handler))
  .route("/accounts",post(create_account_handler))
  .route("/deposit",post(deposit_money_handler))
  .route("/withdraw",post(withdraw_money_handler))
}

async fn create_user_handler(State(state):State<AppState>,Json(req):Json<CreateUserRequest>)->Result<(StatusCode,Json<CreateUserResponse>),AppError>{
  let cmd=CreateUserCommand{
    email:req.email,
    name:req.name,
  };

  let result=commands::create_user::execute(&state,cmd).await?;

  Ok((StatusCode::CREATED,Json(CreateUserResponse{user_id:result.user_id})))
}

async fn create_account_handler(State(state):State<AppState>,Json(req):Json<CreateAccountRequest>)->Result<(StatusCode,Json<CreateAccountResponse>),AppError>{
  let account_type=req.account_type.parse::<AccountType>()?;

  let cmd=CreateAccountCommand{
    owner_id:req.owner_id,
    currency:req.currency,
    account_type
  };

  let result=commands::create_account::execute(&state,cmd).await?;

  Ok((StatusCode::CREATED,Json(CreateAccountResponse{account_number:result.account_number})))
}

async fn deposit_money_handler(State(state):State<AppState>,Json(req):Json<DepositMoneyRequest>)->Result<(StatusCode,Json<DepositMoneyResponse>),AppError>{
  let cmd=DepositMoneyCommand{
    account_id:req.account_id,
    amount:req.amount,
    description:req.description
  };

  let result=commands::deposit_money::execute(&state,cmd).await?;

  Ok((StatusCode::CREATED,Json(DepositMoneyResponse{ledger_entry_id:result.ledger_entry_id})))
}

async fn withdraw_money_handler(State(state):State<AppState>,Json(req):Json<WithdrawMoneyRequest>)->Result<(StatusCode,Json<WithdrawMoneyResponse>),AppError>{
  let cmd=WithdrawMoneyCommand{
    account_id:req.account_id,
    amount:req.amount,
    description:req.description
  };

  let result=commands::withdraw_money::execute(&state,cmd).await?;

  Ok((StatusCode::CREATED,Json(WithdrawMoneyResponse{ledger_entry_id:result.ledger_entry_id})))
}

async fn get_user_handler(State(state):State<AppState>,Path(user_id):Path<Uuid>)->Result<Json<UserResponse>,AppError>{
  let query=GetUserQuery{user_id};
  let user=queries::get_user::execute(&state, query).await?;
  Ok(Json(UserResponse::from(user)))
}
