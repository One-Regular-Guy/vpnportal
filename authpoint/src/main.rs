use ldap3_server::{LdapCodec, LdapServer};
use ldap3_server::controls::*;
use ldap3_server::proto::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:389").await?;
    println!("LDAP fake rodando na porta 389");

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut server = LdapServer::new(LdapCodec::new(), MyLdapBackend);
            if let Err(e) = server.serve(stream).await {
                eprintln!("Erro na conexão: {e}");
            }
        });
    }
}

// Sua lógica interna — você decide tudo aqui
struct MyLdapBackend;

#[async_trait::async_trait]
impl ldap3_server::LdapBackend for MyLdapBackend {
    async fn bind(
        &self,
        _msg: &BindRequest,
    ) -> LdapResult<BindResponse> {
        // Exemplo: aceita qualquer senha para dn "cn=admin,dc=example,dc=com"
        if _msg.dn.as_deref() == Some("cn=admin,dc=example,dc=com") {
            Ok(BindResponse {
                res: LdapResultCode::Success,
                matched_dn: "".into(),
                error_message: "".into(),
                referral: vec![],
                sasl_creds: None,
            })
        } else {
            // Bind simples anônimo também aceito
            Ok(BindResponse {
                res: LdapResultCode::Success,
                ..Default::default()
            })
        }
    }

    async fn search(
        &self,
        msg: &SearchRequest,
    ) -> LdapResult<Vec<SearchEntry>> {
        // Exemplo: retorna usuários fictícios apenas no base "dc=example,dc=com"
        if msg.base_object != "dc=example,dc=com" {
            return Ok(vec![]);
        }

        let mut entries = vec![];

        // Usuário fake 1
        if msg.filter.accepts(&[("uid", "joao")].into()) || msg.filter == Filter::Equality("objectClass".into(), "inetOrgPerson".into()) {
            let mut attrs = AttributeList::new();
            attrs.insert("uid".into(), vec!["joao".into()].into());
            attrs.insert("cn".into(), vec!["João Silva".into()].into());
            attrs.insert("mail".into(), vec!["joao@example.com".into()].into());
            attrs.insert("objectClass".into(), vec!["inetOrgPerson".into(), "person".into()].into());

            entries.push(SearchEntry {
                dn: "uid=joao,ou=users,dc=example,dc=com".into(),
                attributes: attrs,
            });
        }

        // Adicione quantos quiser…

        Ok(entries)
    }

    // Você pode implementar ou deixar como "unwilling to perform" os outros:
    async fn add(&self, _msg: &AddRequest) -> LdapResult<AddResponse> {
        Err(LdapOpError::Result {
            code: LdapResultCode::UnwillingToPerform,
            msg: "ADD não implementado".into(),
        })
    }

    async fn modify(&self, _msg: &ModifyRequest) -> LdapResult<ModifyResponse> {
        Err(LdapOpErrorResult {
            code: LdapResultCode::UnwillingToPerform,
            msg: "MODIFY não implementado".into(),
        })
    }

    async fn delete(&self, _msg: &DeleteRequest) -> LdapResult<DeleteResponse> {
        Err(LdapOpErrorResult {
            code: LdapResultCode::UnwillingToPerform,
            msg: "DELETE não implementado".into(),
        })
    }

    // Extended operations (ex: WhoAmI, StartTLS, etc.)
    async fn extended(&self, _msg: &ExtendedRequest) -> LdapResult<ExtendedResponse> {
        {
        Err(LdapOpErrorResult {
            code: LdapResultCode::UnwillingToPerform,
            msg: "Extended operation não suportada".into(),
        })
    }
}