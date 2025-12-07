<script lang="ts">
  import { fly, fade } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';

  // Simulação de lista de usuários (substitua por fetch real depois)
  let users = $state([
    { uid: 'joao.silva', cn: 'João Silva', mail: 'joao.silva@empresa.com', status: 'active' as 'active' | 'disabled', lastLogin: '2025-12-04 14:32' },
    { uid: 'maria.oliveira', cn: 'Maria Oliveira', mail: 'maria@empresa.com', status: 'active', lastLogin: '2025-12-12-05 09:15' },
    { uid: 'carlos.santos', cn: 'Carlos Santos', mail: 'carlos@empresa.com', status: 'disabled', lastLogin: '2025-11-28 17:44' },
    { uid: 'ana.costa', cn: 'Ana Costa', mail: 'ana.costa@empresa.com', status: 'active', lastLogin: '2025-12-03 11:20' },
    { uid: 'pedro.almeida', cn: 'Pedro Almeida', mail: 'pedro@empresa.com', status:'disabled', lastLogin: '2025-10-10 08:00' },
  ]);

  let search = $state('');
  let filterStatus = $state<'all' | 'active' | 'disabled'>('all');

  // Busca + filtro reativo
  let filteredUsers = $derived(
    users.filter(user => {
        const matchesSearch = user.cn.toLowerCase().includes(search.toLowerCase()) ||
                            user.mail.toLowerCase().includes(search.toLowerCase()) ||
                            user.uid.toLowerCase().includes(search.toLowerCase());
        const matchesStatus = filterStatus === 'all' || user.status === filterStatus;
        return matchesSearch && matchesStatus;
    })
  );
</script>

<svelte:head>
  <title>Usuários LDAP</title>
</svelte:head>

<!-- Fundo mágico eterno -->
<div class="fixed inset-0 -z-10 overflow-hidden">
  <div class="absolute inset-0 bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 animate-gradient-xy"></div>
  <div class="absolute inset-0 bg-gradient-to-tr from-pink-500/20 via-transparent to-cyan-500/20 animate-gradient-xy delay-700"></div>
  {#each Array(60) as _, i}
    <div class="absolute w-1 h-1 bg-white/30 rounded-full animate-float"
         style="left:{Math.random()*100}%;top:{Math.random()*100}%;animation-delay:{i*0.1}s;animation-duration:{(8+Math.random()*12)}s;"></div>
  {/each}
</div>

<div class="min-h-screen py-10 px-4">
  <div class="max-w-7xl mx-auto">

    <!-- Título épico -->
    <h1 class="text-5xl md:text-7xl font-bold text-center text-white mb-16">
      {#each "Usuários do Sistema".split('') as char, i}
        <span in:fly={{ y: 60, delay: 100 + i*70, duration: 900, easing: quintOut }}
              class="inline-block hover:text-cyan-300 transition-colors duration-300">
          {char === ' ' ? '\u00A0' : char}
        </span>
      {/each}
    </h1>

    <!-- Barra de busca + filtro -->
    <div in:fade={{ delay:  800 }} class="max-w-3xl mx-auto mb-12 flex flex-col md:flex-row gap-4">
      <input
        type="text"
        bind:value={search}
        placeholder="Buscar por nome, e-mail ou uid..."
        class="flex-1 px-6 py-4 bg-white/10 border border-white/20 rounded-2xl text-white placeholder-white/40
               focus:outline-none focus:border-cyan-400 focus:ring-4 focus:ring-cyan-400/30 transition-all"
      />

      <select bind:value={filterStatus}
              class="px-6 py-4 bg-white/10 border border-white/20 rounded-2xl text-white">
        <option value="all">Todos os usuários</option>
        <option value="active">Ativos</option>
        <option value="disabled">Desativados</option>
      </select>
    </div>

    <!-- Cards de usuários -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
      {#each filteredUsers as user, i (user.uid)}
        <div
          in:fly={{ y: 50, delay: 200 + i * 100, duration: 800, easing: quintOut }}
          class="group relative backdrop-blur-xl bg-white/10 border border-white/20 rounded-3xl overflow-hidden
                 shadow-2xl hover:shadow-cyan-500/40 transform hover:-translate-y-3 transition-all duration-700
                 cursor-pointer animate-border-glow"
          on:click={() => window.location.href = `/users/${user.uid}`} 
        >
        <!-- ou use <a> com <svelte:fragment> se preferir -->
          <!-- Glow no hover -->
          <div class="absolute inset-0 bg-gradient-to-br from-cyan-500/0 via-purple-500/0 to-pink-500/0
          group-hover:from-cyan-500/20 group-hover:via-purple-500/20 group-hover:to-pink-500/20 transition-all duration-1000"></div>

          <div class="relative p-8">
            <div class="flex items-center justify-between mb-6">
              <div class="w-16 h-16 bg-gradient-to-br from-cyan-400 to-purple-600 rounded-2xl flex items-center justify-center text-3xl font-bold text-white shadow-lg">
                {user.cn.split(' ').map(n => n[0]).join('').toUpperCase()}
              </div>
              <span class="px-4 py-2 rounded-full text-sm font-bold
                           {user.status === 'active' ? 'bg-emerald-500/20 text-emerald-300' : 'bg-red-500/20 text-red-300'}">
                {user.status === 'active' ? 'ATIVO' : 'DESATIVADO'}
              </span>
            </div>

            <h3 class="text-2xl font-bold text-white mb-2 group-hover:text-cyan-300 transition-colors">
              {user.cn}
            </h3>
            <p class="text-white/70 mb-1">{user.mail}</p>
            <p class="text-white/50 text-sm font-mono">{user.uid}</p>

            <p class="mt-6 text-white/60 text-sm">
              Último acesso: <span class="text-cyan-300">{user.lastLogin}</span>
            </p>

            <div class="mt-6 flex justify-end">
              <span class="text-cyan-300 group-hover:text-white transition-colors flex items-center gap-2">
                Ver perfil → 
              </span>
            </div>
          </div>
        </div>
      {:else}
        <p class="col-span-full text-center text-white/60 text-xl py-20">
          Nenhum usuário encontrado.
        </p>
      {/each}
    </div>

    <!-- Botão flutuante de novo usuário -->
    <a href="/users/new"
       class="fixed bottom-10 right-10 w-16 h-16 bg-gradient-to-br from-cyan-500 to-purple-600 rounded-full
              flex items-center justify-center text-4xl text-white shadow-2xl hover:shadow-cyan-500/60
              hover:scale-110 transition-all duration-500 z-50">
      +
    </a>
  </div>
</div>

<style>
  @keyframes gradient-xy { 0%,100%{background-position:0% 0%} 50%{background-position:100% 100%} }
  .animate-gradient-xy { background-size:400% 400%; animation:gradient-xy 15s ease infinite; }
  .delay-700 { animation-delay:.7s; }
  @keyframes float { 0%,100%{transform:translateY(0) translateX(0)} 50%{transform:translateY(-30px) translateX(20px)} }
  .animate-float { animation:float linear infinite; }
  @keyframes border-glow { 0%,100%{box-shadow:0 0 20px rgba(168,85,247,.5)} 50%{box-shadow:0 0 40px rgba(34,211,238,.8)} }
  .animate-border-glow { animation:border-glow 8s ease-in-out infinite; }
</style>