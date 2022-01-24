use crate::errors::{TicxError, TicxResult};
use crate::metrics::*;
use actix_web::{
    dev::Payload,
    get,
    http::HeaderValue,
    web::{block, Data},
    FromRequest, HttpRequest,
};
use db::Db;
use futures::future::{err, ok, Ready};
use std::fmt::Formatter;
use std::{convert::TryFrom, str::FromStr, sync::Arc};

pub(crate) const ISS: &'static str = "TicX server";
pub(crate) const AUD: &'static str = "TicX user";

pub(crate) struct Secret(pub String);

pub(crate) struct Credentials(http_auth_basic::Credentials);
impl Credentials {
    pub fn username(&self) -> &str {
        self.0.user_id.as_str()
    }

    pub fn password(&self) -> &str {
        self.0.password.as_str()
    }
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // this is kinda hacky but dunno how else to do that :/
        let username = format!(
            "Credentials {{ user_id: \"{}\", password: \"censored\" }}",
            self.0.user_id
        );
        f.debug_tuple("Credentials").field(&username).finish()
    }
}

impl TryFrom<&HeaderValue> for Credentials {
    type Error = TicxError;

    fn try_from(hv: &HeaderValue) -> Result<Self, Self::Error> {
        hv.to_str()
            .map_err(|err| TicxError::InvalidHeader {
                header: "AUTHORIZATION",
                value: "failed to decode".into(),
                error: err.to_string(),
            })
            .and_then(|value| {
                Ok(Credentials(
                    http_auth_basic::Credentials::from_str(value).map_err(|err| {
                        TicxError::InvalidHeader {
                            header: "AUTHORIZATION",
                            value: value.into(),
                            error: err.to_string(),
                        }
                    })?,
                ))
            })
    }
}

impl FromRequest for Credentials {
    type Error = TicxError;
    type Future = Ready<TicxResult<Credentials>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .ok_or(TicxError::MissingAuthHeader)
            .and_then(|header_value| Credentials::try_from(header_value))
        {
            Ok(creds) => ok(creds),
            Err(e) => err(e),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    iss: &'static str,
    aud: &'static str,
    sub: String,
    iat: i64,
    exp: i64,
    nbf: i64,
    jti: String,
}

impl Claims {
    fn new(sub: String) -> Self {
        let timestamp = chrono::Local::now().timestamp();
        let exp = timestamp + (7 * 24 * 60 * 60);

        Claims {
            iss: ISS,
            aud: AUD,
            sub,
            iat: timestamp,
            exp,
            nbf: timestamp,
            jti: "Some constant random ID".to_string(),
        }
    }
}

#[get("/login")]
#[tracing::instrument(skip(db, secret))]
pub(crate) async fn login(
    credentials: Option<Credentials>,
    db: Data<Arc<Db>>,
    secret: Data<Arc<Secret>>,
) -> TicxResult<String> {
    let span = tracing::span::Span::current();
    let credentials = credentials.unwrap(); // we can do this since credentials are validated by middleware, if we panic here it means there is bug in middleware

    let timer = DB_QUERY_HISTOGRAM
        .with_label_values(&[DB_TABLE_USERS, "SELECT"])
        .start_timer();

    let user = block(move || {
        let _guard = span.enter();
        db.check_credentials(credentials.username(), credentials.password())
    })
    .await
    .unwrap(); // same as above

    timer.observe_duration();

    jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512),
        &Claims::new(user.id().to_string()),
        &jsonwebtoken::EncodingKey::from_secret(secret.0.as_bytes()),
    )
    .map_err(|err| {
        tracing::trace!(%err, "failed to encode JWT token");
        TicxError::GenericError {
            what: "encode JWT token",
            error: err.to_string(),
        }
    })
}
