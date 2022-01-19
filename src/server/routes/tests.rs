use super::{super::middlewares, *};
use actix_web::http::StatusCode;
use actix_web::test;
use db::Db;
use std::sync::Arc;

struct UserFixture {
    db: Arc<Db>,
    user: db::dbo::User,
}

impl UserFixture {
    pub fn new() -> UserFixture {
        let username = uuid::Uuid::new_v4();
        let db = Arc::new(Db::connect("postgres://user:password@localhost:5432/ticx").unwrap());
        let mut user = db
            .insert_user(db::dbo::NewUser::new(
                username.to_string(),
                "test_password".to_string(),
                "Tester".to_string(),
                "Testerson".to_string(),
            ))
            .unwrap();

        user.password = "test_password".to_string(); // from DB we get hashed password which is useless for testing

        UserFixture { db, user }
    }

    pub fn username(&self) -> &str {
        self.user.username.as_str()
    }

    pub fn password(&self) -> &str {
        self.user.password.as_str()
    }
}

impl Drop for UserFixture {
    fn drop(&mut self) {
        let _ = self.db.delete_user(self.user.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_login() {
        let f = UserFixture::new();

        let credentials = http_auth_basic::Credentials::new(f.username(), f.password());

        let secret = Arc::new(crate::server::routes::auth::Secret(String::from(
            "my_super_jwt_secret",
        )));

        let mut app = test::init_service(
            actix_web::App::new()
                .data(f.db.clone())
                .data(secret.clone())
                .service(
                    super::auth_routes()
                        .wrap(middlewares::BasicAuthMiddleware { db: f.db.clone() }),
                ),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/auth/login")
            .header("Authorization", format!("{}", credentials.as_http_header()))
            .to_request();

        let resp_ok = test::call_service(&mut app, req).await;

        drop(f); // if we do not drop it here manually it will get automatically dropped before request is done

        assert_eq!(resp_ok.status(), StatusCode::OK, "expected 200 OK");
    }

    #[actix_rt::test]
    async fn test_get_user_ok() {
        let f = UserFixture::new();

        let mut app = test::init_service(
            actix_web::App::new()
                .data(f.db.clone())
                .service(super::user_routes()),
        )
        .await;

        let req = test::TestRequest::get()
            .uri(format!("/user/{}", f.user.id).as_str())
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        drop(f);

        assert_eq!(resp.status(), StatusCode::OK);
        let user: super::user::User = test::read_body_json(resp).await;
        assert_eq!(user.password.as_str(), "*censored*")
    }

    #[actix_rt::test]
    async fn test_get_user_not_found() {
        let f = UserFixture::new();

        let mut app = test::init_service(
            actix_web::App::new()
                .data(f.db.clone())
                .service(super::user_routes()),
        )
        .await;

        let req = test::TestRequest::get().uri("/user/99999").to_request();

        let resp = test::call_service(&mut app, req).await;
        drop(f);

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
