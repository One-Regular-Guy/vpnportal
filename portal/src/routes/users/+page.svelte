<script lang="ts">
  import { fly, fade } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';

  let users = $state([
    { uid: 'joao.silva', cn: 'João Silva', mail: 'joao.silva@empresa.com', status: 'active' as 'active' | 'disabled', lastLogin: '2025-12-04 14:32' },
    { uid: 'maria.oliveira', cn: 'Maria Oliveira', mail: 'maria@empresa.com', status: 'active', lastLogin: '2025-12-12 09:15' },
    { uid: 'carlos.santos', cn: 'Carlos Santos', mail: 'carlos@empresa.com', status: 'disabled', lastLogin: '2025-11-28 17:44' },
    { uid: 'ana.costa', cn: 'Ana Costa', mail: 'ana.costa@empresa.com', status: 'active', lastLogin: '2025-12-03 11:20' },
    { uid: 'pedro.almeida', cn: 'Pedro Almeida', mail: 'pedro@empresa.com', status: 'disabled', lastLogin: '2025-10-10 08:00' },
  ]);

  let search = $state('');
  let filterStatus = $state<'all' | 'active' | 'disabled'>('all');

  let filteredUsers = $derived(
    users.filter(user => {
      const matchesSearch = 
        user.cn.toLowerCase().includes(search.toLowerCase()) ||
        user.mail.toLowerCase().includes(search.toLowerCase()) ||
        user.uid.toLowerCase().includes(search.toLowerCase());
      const matchesStatus = filterStatus === 'all' || user.status === filterStatus;
      return matchesSearch && matchesStatus;
    })
  );
</script>

<svelte:head>
  <title>Usuários do Sistema</title>
</svelte:head>

<!-- Fundo principal -->
<div class="min-h-screen bg-gradient-to-br from-blue-950 to-indigo-900 py-12 px-4">
  <div class="max-w-7xl mx-auto">

    <!-- Título -->
    <h1 class="text-5xl md:text-6xl font-bold text-center text-amber-400 mb-16 tracking-wide">
      Usuários do Sistema
    </h1>

    <!-- Busca + Filtro -->
    <div in:fade={{ delay: 300 }} class="max-w-3xl mx-auto mb-12 flex flex-col md:flex-row gap-4">
      <input
        type="text"
        bind:value={search}
        placeholder="Buscar por nome, email ou uid..."
        class="flex-1 px-6 py-4 rounded-xl bg-white/5 border border-sky-500/40 text-white placeholder-sky-300/60
               focus:outline-none focus:border-sky-300 focus:ring-4 focus:ring-sky-300/30 transition"
      />

      <select bind:value={filterStatus}
              class="px-6 py-4 rounded-xl bg-white/5 border border-sky-500/40 text-white focus:outline-none focus:border-sky-300 focus:ring-4 focus:ring-sky-300/30 transition">
        <option value="all">Todos os usuários</option>
        <option value="active">Ativos</option>
        <option value="disabled">Desativados</option>
      </select>
    </div>

    <!-- Lista de usuários -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
      {#each filteredUsers as user, i (user.uid)}
        <button
          in:fly={{ y: 50, delay: i * 100, duration: 800, easing: quintOut }}
          class="group relative backdrop-blur-xl bg-white/10 rounded-3xl border border-sky-400/30 p-8
                 shadow-2xl hover:shadow-sky-400/40 hover:-translate-y-2 transition-all duration-500 cursor-pointer text-left"
          onclick={() => {window.location.href = `/users/${user.uid}`}}
        >
          <!-- Overlay sutil no hover -->
          <div class="absolute inset-0 rounded-3xl bg-gradient-to-br from-sky-500/0 to-sky-400/0 
                        group-hover:from-sky-500/10 group-hover:to-sky-400/20 transition-all duration-700 pointer-events-none"></div>

          <div class="relative">
            <div class="flex items-center justify-between mb-6">
              <div class="w-16 h-16 bg-gradient-to-br from-sky-400 to-sky-600 rounded-2xl flex items-center justify-center 
                           text-2xl font-bold text-white shadow-lg">
                {user.cn.split(' ').map(n => n[0]).join('').toUpperCase()}
              </div>

              <span class="px-4 py-2 rounded-full text-sm font-semibold
                           {user.status === 'active' 
                             ? 'bg-sky-500/20 text-sky-300 border border-sky-400/50' 
                             : 'bg-red-500/20 text-red-300 border border-red-400/50'}">
                {user.status === 'active' ? 'ATIVO' : 'DESATIVADO'}
              </span>
            </div>

            <h3 class="text-2xl font-bold text-amber-400 mb-2 group-hover:text-amber-300 transition-colors">
              {user.cn}
            </h3>
            <p class="text-sky-200/80 mb-1">{user.mail}</p>
            <p class="text-sky-300/60 text-sm font-mono">{user.uid}</p>

            <p class="mt-6 text-sky-200/70 text-sm">
              Último acesso: <span class="text-amber-300">{user.lastLogin}</span>
            </p>

            <div class="mt-6 flex justify-end">
              <span class="text-sky-300 group-hover:text-white flex items-center gap-2 text-sm font-medium transition-colors">
                Ver detalhes →
              </span>
            </div>
          </div>
        </button>
      {:else}
        <p class="col-span-full text-center text-sky-200/60 text-xl py-20">
          Nenhum usuário encontrado.
        </p>
      {/each}
    </div>

    <!-- Botão flutuante novo usuário -->
    <a href="/users/new"
       class="fixed bottom-8 right-8 w-16 h-16 bg-gradient-to-br from-sky-500 to-sky-400 rounded-full
              flex items-center justify-center text-4xl text-white font-light shadow-2xl
              hover:shadow-sky-400/60 hover:scale-110 transition-all duration-300 z-50">
      +
    </a>
  </div>
</div>