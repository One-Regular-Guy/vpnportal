<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';

  // Simulação de dados (substitua por fetch real da API ou banco)
  // Exemplo: fetch(`/api/users/${$page.params.uid}`)
  const mockClients: Client[] = [
    {
      uid: 'joao.silva',
      ou: 'users',
      dc1: 'empresa',
      dc2: 'com',
      cn: 'João Silva',
      sn: 'Silva',
      mail: 'joao.silva@empresa.com',
      password: '', // nunca mostrar!
      is_ldap: true,
      is_active: true,
    },
    // ... outros
  ];

  const uid = $page.params.uid;
  const user = mockClients.find(u => u.uid === uid);

  // Se não encontrar, redireciona ou mostra erro
  if (!user) {
    goto('/users');
  }

  type Client = {
    uid: string;
    ou: string;
    dc1: string;
    dc2: string;
    cn: string;
    sn: string;
    mail: string;
    password: string;
    is_ldap: boolean;
    is_active: boolean;
  };
</script>

<div class="min-h-screen bg-gradient-to-br from-blue-950 to-indigo-900 flex items-center justify-center p-6">
  <div class="w-full max-w-2xl">
    <div class="backdrop-blur-xl bg-white/10 rounded-3xl shadow-2xl border border-sky-400/30 p-10">
      
      <div class="flex items-center justify-between mb-10">
        <h1 class="text-4xl font-bold text-amber-400">Perfil do Usuário</h1>
        <a href="/users" class="text-sky-300 hover:text-white transition">
          ← Voltar
        </a>
      </div>

      {#if user}
        <div class="space-y-8">
          <div class="flex items-center gap-6">
            <div class="w-24 h-24 bg-gradient-to-br from-sky-400 to-sky-600 rounded-3xl flex items-center justify-center text-4xl font-bold text-white shadow-xl">
              {user.cn.split(' ').map(n => n[0]).join('').toUpperCase()}
            </div>
            <div>
              <h2 class="text-3xl font-bold text-amber-300">{user.cn}</h2>
              <p class="text-sky-200/80 text-lg">{user.mail}</p>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <p class="text-sky-300 text-sm font-medium">UID</p>
              <p class="text-white font-mono text-lg">{user.uid}</p>
            </div>
            <div>
              <p class="text-sky-300 text-sm font-medium">Sobrenome</p>
              <p class="text-white text-lg">{user.sn}</p>
            </div>
            <div>
              <p class="text-sky-300 text-sm font-medium">DN Completo</p>
              <p class="text-white/80 text-sm font-mono break-all">
                uid={user.uid},ou={user.ou},dc={user.dc1},dc={user.dc2}
              </p>
            </div>
            <div>
              <p class="text-sky-300 text-sm font-medium">Origem</p>
              <p class="text-white text-lg">{user.is_ldap ? 'LDAP' : 'Local'}</p>
            </div>
            <div class="md:col-span-2">
              <p class="text-sky-300 text-sm font-medium">Status</p>
              <span class="inline-block px-5 py-2 rounded-full text-sm font-bold
                           {user.is_active 
                             ? 'bg-sky-500/20 text-sky-300 border border-sky-400/50' 
                             : 'bg-red-500/20 text-red-300 border border-red-400/50'}">
                {user.is_active ? 'ATIVO' : 'DESATIVADO'}
              </span>
            </div>
          </div>

          <div class="flex gap-4 pt-6">
            <button class="px-8 py-3 rounded-xl bg-gradient-to-r from-sky-500 to-sky-400 text-white font-semibold shadow-lg hover:from-sky-400 hover:to-sky-300 transition">
              Editar Usuário
            </button>
            <button class="px-8 py-3 rounded-xl bg-white/10 border border-sky-400/50 text-sky-300 hover:bg-white/20 transition">
              Redefinir Senha
            </button>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>