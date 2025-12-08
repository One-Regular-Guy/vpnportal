use pasetors::{claims::Claims, keys::AsymmetricKeyPair, public, version4::V4};
use sqlx::PgPool;
use tracing::{debug, info};


use crate::{models::{user_entity::{self, Login, Register, User}, user_repository::Repository}, services::user_repository::{self, UserSqlxPGRepository}};

pub async fn execute_login(login: Login, kp: AsymmetricKeyPair::<V4>, pool: &PgPool) -> (bool, String){
    let repo = user_repository::UserSqlxPGRepository::new();
    let creds_valid: User = repo.get_user_by_email(pool, login.email.clone()).await.unwrap();
    debug!("Login: Retrieving User");
    // Generate the keys and sign the claims.
    // Decide how we want to validate the claims after verifying the token itself.
    // The default verifies the `nbf`, `iat` and `exp` claims. `nbf` and `iat` are always
    // expected to be present.
    // NOTE: Custom claims, defined through `add_additional()`, are not validated. This must be done
    // manually.
    // validation_rules = ClaimsValidationRules::new();
    //let untrusted_token = UntrustedToken::<Public, V4>::try_from(&pub_token).unwrap();
    //let trusted_token = public::verify(&kp.public, &untrusted_token, &validation_rules, None, Some(b"implicit assertion")).unwrap();
    //assert_eq!(&claims, trusted_token.payload_claims().unwrap());
    //let _claims = trusted_token.payload_claims().unwrap();
    
    debug!("Login: Verifying Credentials");
    if bcrypt::verify(&login.password, creds_valid.password()).unwrap_or(false) && (login.email == creds_valid.email().to_owned()) {
        let mut claims = Claims::new().unwrap();
        claims.add_additional("user",creds_valid.id().to_string()).map_err(|_| info!("Failed to create claims")).unwrap();
        claims.add_additional("name",creds_valid.name()).map_err(|_| info!("Failed to create claims")).unwrap();
        debug!("Login: Claims Created");
        let pub_token = public::sign(
            &kp.secret, 
            &claims, 
            None, 
            Some(b"implicit assertion")).map_err(|_| info!("Failed to sign token")).unwrap();
        debug!("Login: Token Generated");
        (true, pub_token)
    } else {
        debug!("Login: Authentication failed");
        (false, "".to_string())

    }
}
pub async fn execute_register(register: Register, pool: &PgPool) -> bool{
    let repo: UserSqlxPGRepository = user_repository::UserSqlxPGRepository::new();
    let user: User = User::new_from_register(register);
    repo.create_user(pool, user).await.unwrap();
    true
}