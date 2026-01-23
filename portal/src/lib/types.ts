// src/lib/types.ts
export interface Client {
  uid: string;
  ou: string;
  dc1: string;
  dc2: string;
  cn: string;
  sn: string;
  mail: string;
  password: string;     // ← provavelmente não vem do banco em produção
  is_ldap: boolean;
  is_active: boolean;
}