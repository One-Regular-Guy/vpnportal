

use poem_openapi::Object;
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Client {
    pub uid: String,
    pub ou: String,
    pub dc1: String,
    pub dc2: String,
    pub cn: String,
    pub sn: String,
    pub mail: String,
    pub password: String,
    pub actual_download_id: Option<String>,
    pub certificate: Option<String>,
    pub totp_secret: Option<String>,
    pub is_ldap: bool,
    pub is_active: bool,
}

#[derive(Debug, Clone, serde::Deserialize, Object)]
pub struct LoginLdap {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Deserialize, Object)]
pub struct RegisterLdap {
    pub uid: String,
    pub ou: String,
    pub dc1: String,
    pub dc2: String,
    pub cn: String,
    pub sn: String,
    pub mail: String,
    pub password: String,
    pub is_ldap: bool,
}

impl Client {
    pub fn new(uid: impl Into<String>, ou: impl Into<String>, dc1: impl Into<String>, dc2: impl Into<String>, cn: impl Into<String>, sn: impl Into<String>, mail: impl Into<String>, password: impl Into<String>, is_ldap: impl Into<bool>, is_active: impl Into<bool>) -> Self {
        let uid: String = uid.into();
        let ou: String = ou.into(); 
        let dc1: String = dc1.into(); 
        let dc2: String = dc2.into(); 
        let cn: String = cn.into(); 
        let sn: String = sn.into(); 
        let mail: String = mail.into();
        let actual_download_id: Option<String> = None;
        let certificate: Option<String> = None;
        let totp_secret: Option<String> = None;
        let password: String = bcrypt::hash(password.into(),8).unwrap();
        let is_ldap: bool = is_ldap.into();
        let is_active: bool = is_active.into();
        Self { uid, ou, dc1, dc2, cn, sn, mail, actual_download_id, certificate, totp_secret, password, is_ldap, is_active }
    }
    pub fn new_from_register(register: RegisterLdap) -> Self {
        let uid: String = register.uid;
        let ou: String = register.ou; 
        let dc1: String = register.dc1; 
        let dc2: String = register.dc2; 
        let cn: String = register.cn; 
        let sn: String = register.sn; 
        let mail: String = register.mail;
        let actual_download_id: Option<String> = None;
        let certificate: Option<String> = None;
        let totp_secret: Option<String> = None;
        let password: String = bcrypt::hash("",8).unwrap();
        let is_ldap: bool = register.is_ldap;
        let is_active: bool = true;
        Self { uid, ou, dc1, dc2, cn, sn, mail, actual_download_id, certificate, totp_secret, password, is_ldap, is_active }
    }
    pub fn uid(&self) -> &str {
        &self.uid
    }

    pub fn mail(&self) -> &str {
        &self.mail
    }

    pub fn ou(&self) -> &str {
        &self.ou
    }

    pub fn dc1(&self) -> &str {
        &self.ou
    }

    pub fn dc2(&self) -> &str {
        &self.ou
    }

    pub fn cn(&self) -> &str {
        &self.ou
    }

    pub fn sn(&self) -> &str {
        &self.ou
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn is_ldap(&self) -> bool {
        self.is_ldap
    }

    pub fn password(&self) -> &str {
        &self.password
    }
    
}