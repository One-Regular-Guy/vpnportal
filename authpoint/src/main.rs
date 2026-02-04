use ldap3::{LdapConnAsync, LdapConnSettings, Scope, SearchEntry};
use anyhow::Result;
use std::time::Duration;
use tokio::time::{self, timeout};

#[tokio::main]
async fn main() -> Result<()> {
    let ldap_url = "ldap://172.16.38.168:389";  // troque pelo real
    // let ldap_url = "ldaps://seu-servidor:636";  // se usar TLS

    let settings = LdapConnSettings::new()
        .set_conn_timeout(Duration::from_secs(10));

    println!("Tentando conectar em {} ...", ldap_url);

    let connect_future = LdapConnAsync::with_settings(settings, ldap_url);
    let (conn, mut ldap) = match timeout(Duration::from_secs(12), connect_future).await {
        Ok(Ok(res)) => res,
        Ok(Err(e)) => return Err(anyhow::anyhow!("Falha na conexão: {}", e)),
        Err(_) => return Err(anyhow::anyhow!("Timeout na conexão (12s)")),
    };
    ldap3::drive!(conn);
    println!("Conexão estabelecida!");

    println!("Fazendo bind...");
    let bind_future = ldap.simple_bind("cn=admin,dc=fab,dc=intraer", "%d3sp0mb@l1z@d0%");
    let bind_res = match timeout(Duration::from_secs(10), bind_future).await {
        Ok(Ok(res)) => res,
        Ok(Err(e)) => return Err(anyhow::anyhow!("Bind falhou: {}", e)),
        Err(_) => return Err(anyhow::anyhow!("Timeout no bind (10s)")),
    };
    bind_res.success()?;

    println!("Bind OK!");

    let base   = "ou=contas,dc=fab,dc=intraer";
    let filter = "(uid=49469856899)";
    let attrs: Vec<&str> = vec!["cn", "mail"];

    println!("Executando search...");
    let search_future = ldap.search(base, Scope::Subtree, filter, attrs);
    let (search_result, _result) = match timeout(Duration::from_secs(30), search_future).await {
        Ok(Ok(res)) => res.success()?,
        Ok(Err(e)) => return Err(anyhow::anyhow!("Search falhou: {}", e)),
        Err(_) => return Err(anyhow::anyhow!("Timeout no search (30s)")),
    };

    println!("Resultados encontrados: {}", search_result.len());

    for raw_entry in search_result {
        let entry = SearchEntry::construct(raw_entry);
        println!("DN: {}", entry.dn);
        println!("  CN:   {:?}", entry.attrs.get("cn"));
        println!("  Mail: {:?}", entry.attrs.get("mail"));
    }

    println!("Desconectando...");
    ldap.unbind().await?;

    Ok(())
}