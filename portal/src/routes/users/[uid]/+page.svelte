<script lang="ts">
  import { fly, fade, slide } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';

  // Simulação de dados do usuário
  const user = {
    uid: 'joao.silva',
    cn: 'João Silva',
    mail: 'joao.silva@empresa.com',
    dn: 'uid=joao.silva,ou=users,dc=empresa,dc=com',
    status: 'active' as 'active' | 'disabled',
    lastLogin: '2025-12-04 14:32:11',
    created: '2025-01-15 09:12:44'
  };

  let status = $state(user.status);
  let showAttempts = $state(false);
  let showDetails = $state(true);

  // Simulação de tentativas de login
  const loginAttempts = [
    { date: '2025-12-04 14:32:11', ip: '187.45.22.101', success: true },
    { date: '2025-12-04 10:15:03', ip: '187.45.22.101', success: true },
    { date: '2025-12-03 23:58:44', ip: '189.10.55.201', success: false },
    { date: '2025-12-03 23:58:30', ip: '189.10.55.201', success: false },
    { date: '2025-12-03 23:58:15', ip: '189.10.55.201', success: false },
  ];

  function toggleStatus() {
    status = status === 'active' ? 'disabled' : 'active';
  }
</script>

<svelte:head>
  <title>Usuário • {user.cn}</title>
</svelte:head>

<!-- Mesmo fundo épico -->
<div class="fixed inset-0 -z-10 overflow-hidden">
  <div class="absolute inset-0 bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 animate-gradient-xy"></div>
  <div class="absolute inset-0 bg-gradient-to-tr from-pink-500/20 via-transparent to-cyan-500/20 animate-gradient-xy delay-700"></div>
  {#each Array(60) as _, i}
    <div class="absolute w-1 h-1 bg-white/30 rounded-full animate-float"
         style="left: {Math.random()*100}%; top: {Math.random()*100}%; animation-delay: {i*0.1}s; animation-duration: {(8 + Math.random()*12)}s;"></div>
  {/each}
</div>

<div class="min-h-screen py-10 px-4">
  <div class="max-w-5xl mx-auto">
    <div in:fly={{ y: 60, duration: 1000, easing: quintOut }}
         class="backdrop-blur-2xl bg-white/10 border border-white/20 rounded-3xl shadow-2xl overflow-hidden animate-border-glow">

      <div class="absolute inset-0 bg-gradient-to-br from-purple-600/20 via-transparent to-pink-600/20 animate-pulse"></div>

      <div class="relative p-8 md:p-12">

        <!-- Cabeçalho com nome e status -->
        <div class="text-center mb-12">
          <h1 class="text-5xl md:text-6xl font-bold text-white mb-4">
            {#each user.cn.split('') as char, i}
              <span in:fly={{ y: 50, delay: 200 + i*60, duration: 800, easing: quintOut }}
                     class="inline-block hover:text-cyan-300 transition-colors">
                {char === ' ' ? '\u00A0' : char}
              </span>
            {/each}
          </h1>
          <p class="text-white/60 text-lg">{user.mail}</p>
        </div>

        <!-- Card expansível: Informações principais -->
        <div class="mb-6">
          <button
            on:click={() => showDetails = !showDetails}
            class="w-full text-left bg-white/5 hover:bg-white/10 rounded-2xl p-6 transition-all duration-500 border border-white/10 group"
          >
            <div class="flex items-center justify-between">
              <h2 class="text-2xl font-bold text-white">Informações do Usuário</h2>
              <span class="text-3xl text-white/60 group-hover:text-cyan-300 transition-transform duration-500 {showDetails ? 'rotate-90' : ''}">›</span>
            </div>
          </button>

          {#if showDetails}
            <div transition:slide={{ duration: 600, easing: quintOut }}
              class="bg-white/5 border border-white/10 rounded-2xl p-8 mt-3 space-y-5">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div><span class="text-white/70">DN</span><p class="text-white font-mono text-sm break-all">{user.dn}</p></div>
                <div><span class="text-white/70">UID</span><p class="text-cyan-300 font-bold">{user.uid}</p></div>
                <div><span class="text-white/70">Criado em</span><p class="text-white">{user.created}</p></div>
                <div><span class="text-white/70">Último acesso</span><p class="text-green-400">{user.lastLogin}</p></div>
              </div>

              <!-- Botão Ativar/Desativar -->
              <div class="text-center mt-8">
                <button
                  on:click={toggleStatus}
                  class="px-10 py-4 rounded-xl font-bold text-lg transition-all duration-500 transform hover:scale-105
                         {status === 'active'
                           ? 'bg-red-600 hover:bg-red-700 text-white shadow-red-500/50'
                           : 'bg-emerald-600 hover:bg-emerald-700 text-white shadow-emerald-500/50'}"
                >
                  {status === 'active' ? 'Desativar Conta' : 'Ativar Conta'}
                </button>
                <p class="mt-3 text-white/60">
                  Status atual: 
                  <span class="{status === 'active' ? 'text-emerald-400' : 'text-red-400'} font-bold">
                    {status === 'active' ? 'ATIVA' : 'DESATIVADA'}
                  </span>
                </p>
              </div>
            </div>
          {/if}
        </div>

        <!-- Card expansível: Últimas tentativas -->
        <div>
          <button
            on:click={() => showAttempts = !showAttempts}
            class="w-full text-left bg-white/5 hover:bg-white/10 rounded-2xl p-6 transition-all duration-500 border border-white/10 group"
          >
            <div class="flex items-center justify-between">
              <h2 class="text-2xl font-bold text-white">Últimas Tentativas de Acesso</h2>
              <span class="text-3xl text-white/60 group-hover:text-cyan-300 transition-transform duration-500 {showAttempts ? 'rotate-90' : ''}">›</span>
            </div>
          </button>

         

          {#if showAttempts}
            <div transition:slide={{ duration: 600, easing: quintOut }}
                 class="bg-white/5 border border-white/10 rounded-2xl p-8 mt-3 overflow-hidden">
              <div class="space-y-4">
                {#each loginAttempts as attempt, i}
                  <div in:fly={{ y: 30, delay: i * 100, duration: 600 }}
                       class="flex items-center justify-between py-4 px-6 bg-white/5 rounded-xl border {attempt.success ? 'border-green-500/30' : 'border-red-500/30'}">
                    <div class="flex items-center gap-5">
                      <div class="w-3 h-3 rounded-full {attempt.success ? 'bg-emerald-400' : 'bg-red-400'} animate-pulse"></div>
                      <div>
                        <p class="text-white font-medium">{attempt.date}</p>
                        <p class="text-white/60 text-sm">IP: {attempt.ip}</p>
                      </div>
                    </div>
                    <span class="px-4 py-2 rounded-lg text-sm font-bold {attempt.success ? 'bg-emerald-500/20 text-emerald-300' : 'bg-red-500/20 text-red-300'}">
                      {attempt.success ? 'SUCESSO' : 'FALHA'}
                    </span>
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        </div>

      </div>
    </div>
  </div>
</div>

<style>
  @keyframes gradient-xy { 0%,100% {background-position:0% 0%} 50% {background-position:100% 100%} }
  .animate-gradient-xy { background-size:400% 400%; animation:gradient-xy 15s ease infinite; }
  .delay-700 { animation-delay:0.7s; }
  @keyframes float { 0%,100%{transform:translateY(0) translateX(0)} 50%{transform:translateY(-30px) translateX(20px)} }
  .animate-float { animation:float linear infinite; }
  @keyframes border-glow { 0%,100%{box-shadow:0 0 20px rgba(168,85,247,.5)} 50%{box-shadow:0 0 40px rgba(34,211,238,.8)} }
  .animate-border-glow { animation:border-glow 8s ease-in-out infinite; }
</style>