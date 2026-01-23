# Portal VPN

## HADES (central de Gerenciamento e Disponibilização)

| Coluna    | Tipo         | Constraints          | Descrição          |
|-----------|--------------|----------------------|--------------------|
| id        | UUID         | PRIMARY KEY          | Identificador único |
| name      | varchar(255) | NOT NULL, UNIQUE     | Nome do usuário    |
| email     | varchar(255) | NOT NULL, UNIQUE     | Email do usuário   |
| password  | varchar(255) | NOT NULL             | Senha (hash)       |

| Coluna     | Tipo         | Constraints          | Descrição                        |
|------------|--------------|----------------------|----------------------------------|
| uid        | varchar(255) | PRIMARY KEY          | Identificador único do cliente   |
| ou         | varchar(255) | NOT NULL             | Organizational Unit              |
| dc1        | varchar(255) | NOT NULL             | Domain Component 1               |
| dc2        | varchar(255) | NOT NULL             | Domain Component 2               |
| cn         | varchar(255) | NOT NULL, UNIQUE     | Common Name                      |
| sn         | varchar(255) | NOT NULL, UNIQUE     | Surname / Sobrenome              |
| mail       | varchar(255) | NOT NULL, UNIQUE     | Email                            |
| password   | varchar(255) | NOT NULL             | Senha (hash)                     |
| is_ldap    | boolean      | NOT NULL             | Usuário gerenciado via LDAP?     |
| is_active  | boolean      | NOT NULL             | Conta está ativa?                |