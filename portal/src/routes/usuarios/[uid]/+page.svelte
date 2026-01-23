<script lang="ts">
  import type { PageData } from './$types';
  import type { Client } from '$lib/types';
  let { data }: { data: PageData } = $props();
  let user: Client = $derived(data.user);
</script>

<div class="min-h-screen bg-gradient-to-b from-gray-900 via-blue-950 to-gray-900 p-6">
  <div class="max-w-4xl mx-auto">
    <div class="flex justify-between items-center mb-8">
      <h1 class="text-3xl font-bold text-white">
        {user.cn} <span class="text-yellow-400">{user.sn}</span>
      </h1>
      <a href="/usuarios" class="text-blue-400 hover:text-blue-300">
        ← Voltar para lista
      </a>
    </div>

    <div class="grid md:grid-cols-2 gap-6">
      <div class="bg-gray-900/70 backdrop-blur-md border border-blue-900/40 rounded-xl p-6 shadow-xl">
        <h2 class="text-xl font-semibold text-blue-300 mb-4">Informações Básicas</h2>
        
        <dl class="space-y-3 text-gray-300">
          <div>
            <dt class="text-sm text-gray-500">Nome completo</dt>
            <dd class="font-medium">{user.cn} {user.sn}</dd>
          </div>
          <div>
            <dt class="text-sm text-gray-500">E-mail</dt>
            <dd class="font-medium">{user.mail}</dd>
          </div>
          <div>
            <dt class="text-sm text-gray-500">UID / Login</dt>
            <dd class="font-mono">{user.uid}</dd>
          </div>
          <div>
            <dt class="text-sm text-gray-500">Unidade Organizacional</dt>
            <dd>{user.ou}</dd>
          </div>
        </dl>
      </div>

      <div class="bg-gray-900/70 backdrop-blur-md border border-blue-900/40 rounded-xl p-6 shadow-xl">
        <h2 class="text-xl font-semibold text-blue-300 mb-4">Status & Metadados</h2>
        
        <dl class="space-y-3 text-gray-300">
          <div>
            <dt class="text-sm text-gray-500">Status Atual</dt>
            <dd>
              <span class={`inline-block px-3 py-1 rounded-full text-sm
                ${user.is_active ? 'bg-green-900/70 text-green-300' : 'bg-red-900/70 text-red-300'}`}>
                {user.is_active ? 'ATIVO' : 'INATIVO'}
              </span>
            </dd>
          </div>
          <div>
            <dt class="text-sm text-gray-500">É usuário LDAP?</dt>
            <dd>{user.is_ldap ? 'Sim' : 'Não (local)'}</dd>
          </div>
          <div>
            <dt class="text-sm text-gray-500">Domínio</dt>
            <dd class="font-mono text-sm">{user.dc1}.{user.dc2}</dd>
          </div>
        </dl>
      </div>
    </div>

    <!-- Espaço reservado para futuros últimos acessos -->
    <div class="mt-8 bg-gray-900/70 border border-blue-900/40 rounded-xl p-6">
      <h2 class="text-xl font-semibold text-blue-300 mb-4">Últimos Acessos</h2>
      <p class="text-gray-500 italic">Em breve...</p>
    </div>
  </div>
</div>