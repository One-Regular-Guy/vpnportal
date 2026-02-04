use std::time::Duration;

use ldap3::{LdapConnAsync, LdapConnSettings, Scope, SearchEntry};
use sqlx::PgPool;
use tokio::time::timeout;

use crate::{models::{cachain_entity::CaChain, client_entity::{Client, LdapInfo, RegisterLdap}, client_repository::Repository, errors::ApiError}, services::client_repository};


pub async fn execute_register_not_ldap(pool: &PgPool, incomming_client: Client) -> Result<(),ApiError> {
    let repository = client_repository::ClientSqlxPGRepository::new();
    repository.create_client(pool, incomming_client).await.map_err(|e| ApiError::DatabaseError(e))
}
pub async fn execute_register_ldap(pool: &PgPool, incomming_client_from_ldap: RegisterLdap) -> Result<(),ApiError> {
    let repository = client_repository::ClientSqlxPGRepository::new();
    let converted_client = Client::new_from_register(incomming_client_from_ldap);
    repository.create_client(pool, converted_client).await.map_err(|e| ApiError::DatabaseError(e))
}
pub async fn execute_ldap_client(ldap_info: LdapInfo, mail: &str) -> Result<RegisterLdap,ApiError> {
    let ldap_url = "ldap://".to_owned()+&ldap_info.ip+":"+&ldap_info.port;

    let settings = LdapConnSettings::new()
        .set_conn_timeout(Duration::from_secs(10));

    println!("Tentando conectar em {} ...", ldap_url);

    let connect_future = LdapConnAsync::with_settings(settings, &ldap_url);
    let (conn, mut ldap) = match timeout(Duration::from_secs(12), connect_future).await {
        Ok(Ok(res)) => res,
        Ok(Err(e)) => return Err(ApiError::LdapError(anyhow::anyhow!("Falha na conexão: {}", e))),
        Err(_) => return Err(ApiError::LdapError(anyhow::anyhow!("Timeout na conexão (12s)"))),
    };
    ldap3::drive!(conn);
    println!("Conexão estabelecida!");
    let base   = ldap_info.ou.clone()+&ldap_info.base;
    let filter = "(mail=".to_owned()+mail+")";
    let attrs: Vec<&str> = vec!["cn", "mail", "uid","sn"];

    println!("Executando search...");
    let search_future = ldap.search(&base, Scope::Subtree, &filter, attrs);
    let (search_result, _result) = match timeout(Duration::from_secs(30), search_future).await {
        Ok(Ok(res)) => res.success().map_err(|e|ApiError::LdapError(anyhow::anyhow!("Search falhou: {}", e)))?,
        Ok(Err(e)) => return Err(ApiError::LdapError(anyhow::anyhow!("Search falhou: {}", e))),
        Err(_) => return Err(ApiError::LdapError(anyhow::anyhow!("Timeout no search (30s)"))),
    };

    println!("Resultados encontrados: {}", search_result.len());
    let raw_entry = search_result.into_iter().next().unwrap();
    let entry = SearchEntry::construct(raw_entry);

    let get_first = |attr: &str| -> String {
        entry
            .attrs
            .get(attr)
            .and_then(|vals| vals.first().cloned())
            .unwrap_or_default()
    };

    let uid = get_first("uid");
    let cn  = get_first("cn");
    let mail_found = get_first("mail");
    let sn = get_first("sn");

    // Extrai dc=... do base (ldap_info.base)
    let dc_parts: Vec<String> = ldap_info.base
        .split(',')
        .filter_map(|part| part.strip_prefix("dc=").map(str::to_string))
        .collect();

    let (dc1, dc2) = match dc_parts.as_slice() {
        [d1, d2] => (d1.clone(), d2.clone()),
        _ => ("".to_string(), "".to_string()),
    };

    // Monta o struct de retorno
    let register = RegisterLdap {
        uid,
        cn,
        sn,
        mail: mail_found,
        dc1,
        dc2,
        ou: ldap_info.ou.clone(),
        is_ldap: true
    };

    println!("Desconectando...");
    ldap.unbind().await.map_err(|e|ApiError::LdapError(anyhow::anyhow!("Search falhou: {}", e)))?;
    Ok(register)
}