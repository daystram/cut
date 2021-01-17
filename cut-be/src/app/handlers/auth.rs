use crate::app::{datatransfers::auth::TokenInfo, Module};
use crate::core::error::{HandlerError, HandlerErrorKind};
use actix_web::{client::Client, web, HttpRequest};

pub async fn authorize(m: &web::Data<Module>, req: &HttpRequest) -> Result<TokenInfo, HandlerError> {
    let access_token: &str = match req.headers().get("Authorization") {
        Some(header_value) => match header_value.to_str() {
            Ok(access_token) => access_token.trim_start_matches("Bearer "),
            Err(_) => return Err(HandlerErrorKind::GeneralError.into()),
        },
        _ => return Err(HandlerErrorKind::UnauthorizedError.into()),
    };
    match Client::default()
        .post(format!("{}/oauth/introspect", m.config.oauth_issuer))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send_body(format!(
            "token={}&client_id={}&client_secret={}&token_type_hint=access_token",
            access_token, m.config.client_id, m.config.client_secret
        ))
        .await
    {
        Ok(mut res) => match res.json::<TokenInfo>().await {
            Ok(token_info) => match token_info.active {
                true => Ok(token_info),
                false => Err(HandlerErrorKind::UnauthorizedError.into()),
            },
            Err(_) => Err(HandlerErrorKind::UnauthorizedError.into()),
        },
        Err(_) => Err(HandlerErrorKind::GeneralError.into()),
    }
}
