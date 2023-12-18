use std::env;

use actix_web::{error::ErrorUnauthorized, Error, FromRequest};
use chrono::Utc;
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct JwToken {
    pub user_id: i32,
    pub exp: usize,
}
impl JwToken {
    // gets the secret key for the serialization and deserialization from the config.yml file.
    pub fn get_key() -> String {
        let key_str = env::var("SECRET_KEY").unwrap();
        key_str
    }
    // encodes the data from the JwToken struct as a token
    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwToken::get_key().as_ref());
        let token = encode(&Header::default(), &self, &key).unwrap();
        token
    }
    // creates a new JwToken struct
    pub fn new(user_id: i32) -> Self {
        let minutes = env::var("EXPIRE_MINUTES").unwrap()
            .parse::<i64>().unwrap();
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::minutes(minutes))
            .expect("valid timestamp")
            .timestamp();
        JwToken {
            user_id,
            exp: expiration as usize,
        }
    }
    // creates a JwToken struct from a token.
    // If there is a failure in the deserialization it returns a None as there can be failures in deserialization.
    pub fn from_token(token: String) -> Result<Self, String> {
        let key = DecodingKey::from_secret(JwToken::get_key().as_ref());
        let toke_result = decode::<JwToken>(&token.as_str(), &key, &Validation::default());
        match toke_result {
            Ok(data) => Ok(data.claims),
            Err(err) => {
                let message = format!("{}", err);
                Err(message)
            }
        }
    }
}
impl FromRequest for JwToken {
    type Error = Error;

    type Future = Ready<Result<JwToken, Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();

                let token_result = JwToken::from_token(raw_token);
                match token_result {
                    Ok(token) => return ok(token),
                    Err(message) => {
                        if message == "ExpiredSignature".to_owned() {
                            return err(ErrorUnauthorized("token expired"));
                        }
                        return err(ErrorUnauthorized("token can't be decoded"));
                    }
                }
            }
            None => err(ErrorUnauthorized("token not in header under 'token'")),
        }
    }
}


#[cfg(test)]
mod jwt_tests {
    use std::{str::FromStr, env};

    use actix_web::{HttpRequest, HttpResponse, test::{init_service, TestRequest, call_service}, App, web, http::header::{ContentType, HeaderName, HeaderValue}};
    use serde::{Deserialize, Serialize};
    use serde_json::json;


    use super::JwToken;

    #[derive(Debug,Serialize,Deserialize)]
    pub struct ResponseFromTest {
        pub user_id: i32,
        pub exp_minutes: i32,
    }
    #[test]
    fn get_key() {
        assert_eq!(String::from("secret"),JwToken::get_key());
    }

    #[test]
    fn get_exp() {
        let minutes = env::var("EXPIRE_MINUTES").unwrap()
        .parse::<i64>().unwrap();
        assert_eq!(120,minutes);
    }

    #[test]
    fn decode_incorect_token(){
        let encoded_token = String::from("invalid token") ;

        match JwToken::from_token(encoded_token) {
            Err(message) => assert_eq!("InvalidToken",message),
            _ => panic!("Incorrect token should not be able to be encoded")
        }
    }

    #[test]
    fn encode_decode(){
        let test_token = JwToken::new(15);
        let encoded_token = test_token.encode();
        let new_token = JwToken::from_token(encoded_token).unwrap();
        assert_eq!(15,new_token.user_id);
    }

    async fn test_handler(token: JwToken,_ : HttpRequest) -> HttpResponse {
        HttpResponse::Ok().json(json!({
            "user_id": token.user_id,
            "exp_minutes": 60
        }))
    }

    #[actix_web::test]
    async fn test_no_token_request() {
        let app = init_service(App::new()
            .route("/", web::get().to(test_handler)))
            .await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = call_service(&app, req).await;
        assert_eq!("401",resp.status().as_str());
    }

    #[actix_web::test]
    async fn test_passing_token_request() {
        let token =JwToken::new(15);
        let encoded_token = token.encode();
        let app = init_service(App::new()
        .route("/", web::get().to(test_handler)))
        .await; 
        let mut req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let header_name = HeaderName::from_str("token").unwrap();
        let header_value = HeaderValue::from_str(encoded_token.as_str()).unwrap();
        
        req.headers_mut().insert(header_name, header_value);

        let resp: ResponseFromTest = actix_web::test::call_and_read_body_json(&app, req).await;

        assert_eq!(15,resp.user_id);
    }

    #[actix_web::test]
    async fn test_false_token_request() {
        let app = init_service(App::new()
        .route("/", web::get().to(test_handler)))
        .await; 
        let mut req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let header_name = HeaderName::from_str("token").unwrap();
        let header_value = HeaderValue::from_str("test").unwrap();

        req.headers_mut().insert(header_name, header_value);

        let resp = call_service(&app, req).await;
        assert_eq!("401",resp.status().as_str());
    }
}