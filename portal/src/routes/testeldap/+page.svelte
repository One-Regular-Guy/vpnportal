<script lang="ts">
  import { fly, fade, scale, slide, blur /* etc */ } from 'svelte/transition';
  import { backInOut, quintOut, elasticOut /* etc */ } from 'svelte/easing';
  let step = $state(1) // 1 = só email, 2 = formulário completo

  // Passo 1
  let email = $state('')

  // Passo 2 - todos os campos LDAP
  let dn = $state('')
  let uid = $state('')
  let ou = $state('')
  let dc1 = $state('')
  let dc2 = $state('')
  let cn = $state('')
  let sn = $state('')
  let mail = $state('')
  let userPassword = $state('')

  // Simulação: se email não existir → passo 2
  function checkEmail() {
    // Aqui você faria a busca no LDAP (ignoramos a lógica)
    // Simulando que o email NÃO foi encontrado
    step = 2
    mail = email // pré-preenche o mail
    cn = email.split('@')[0] // sugestão de cn
    uid = email.split('@')[0]
  }

  function voltar() {
    step = 1
    email = ''
  }
</script>

<svelte:head>
  <title>Novo Usuário LDAP</title>
</svelte:head>

<!-- Mesmo fundo mágico do login -->
<div class="fixed inset-0 -z-10 overflow-hidden">
  <div class="absolute inset-0 bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 animate-gradient-xy"></div>
  <div class="absolute inset-0 bg-gradient-to-tr from-pink-500/20 via-transparent to-cyan-500/20 animate-gradient-xy delay-700"></div>

  {#each Array(60) as _, i}
    <div
      class="absolute w-1 h-1 bg-white/30 rounded-full animate-float"
      style="left: {Math.random() * 100}%; top: {Math.random() * 100}%; animation-delay: {i * 0.1}s; animation-duration: {(8 + Math.random() * 12)}s;"
    ></div>
  {/each}
</div>

<div class="min-h-screen flex items-center justify-center p-4">
  <div
    in:fly={{ y: 60, duration: 1000, easing: quintOut }}
    class="relative max-w-2xl w-full"
  >
    <div
      class="relative backdrop-blur-2xl bg-white/10 border border-white/20 rounded-3xl shadow-2xl overflow-hidden
             animate-border-glow"
    >
      <div class="absolute inset-0 bg-gradient-to-br from-purple-600/20 via-transparent to-pink-600/20 animate-pulse"></div>

      <div class="relative p-10">
        <!-- Título -->
        <h1 class="text-5xl font-bold text-white text-center mb-10 tracking-tight">
          {#each "Novo Usuário LDAP".split('') as char, i}
            <span
              in:fly={{ y: 50, delay: 300 + i * 80, duration: 800, easing: quintOut }}
              class="inline-block hover:text-cyan-300 transition-colors duration-300"
            >
              {char === ' ' ? '\u00A0' : char}
            </span>
          {/each}
        </h1>

        <!-- Passo 1: Apenas email -->
        {#if step === 1}
          <div in:fade={{ delay: 800 }}>
            <p class="text-center text-white/70 mb-8">Digite o e-mail do novo usuário</p>
            <form on:submit|preventDefault={checkEmail} class="max-w-md mx-auto">
              <input
                type="email"
                bind:value={email}
                required
                placeholder="novo.usuario@empresa.com"
                class="w-full px-6 py-5 bg-white/10 border border-white/20 rounded-2xl text-white text-lg
                       placeholder-white/40 focus:outline-none focus:border-cyan-400 focus:ring-4 focus:ring-cyan-400/30
                       transition-all duration-500"
                autofocus
              />

              <button
                type="submit"
                class="w-full mt-8 py-5 rounded-2xl font-bold text-lg bg-gradient-to-r from-cyan-500 to-purple-600
                       text-white shadow-lg hover:shadow-cyan-2xl transform hover:scale-105 transition-all duration-500 group"
              >
                <span class="flex items-center justify-center gap-3">
                  Continuar
                  <span class="inline-block group-hover:translate-x-3 transition-transform">→</span>
                </span>
              </button>
            </form>
          </div>
        {/if}

        <!-- Passo 2: Formulário completo (aparece só se email não existir) -->
        {#if step === 2}
          <div in:fade={{ duration: 800 }}>
            <button
              on:click={voltar}
              class="mb-6 text-cyan-300 hover:text-cyan-100 flex items-center gap-2 transition-colors"
            >
              ← Voltar
            </button>

            <form class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div in:scale={{ delay: 200, duration: 600, start: 0.9 }}>
                <label class="block text-sm font-medium text-white/80 mb-2">DN completo</label>
                <input bind:value={dn} required placeholder="uid=joao,ou=users,dc=empresa,dc=com" class="input-ldap" />
              </div>

              <div in:scale={{ delay: 300, duration: 600, start: 0.9 }}>
                <label class="block text-sm font-medium text-white/80 mb-2">uid</label>
                <input bind:value={uid} required placeholder="joao.silva" class="input-ldap" />
              </div>

              <div in:scale={{ delay: 400, duration: 600, start: 0.9 }}>
                <label class="block text-sm font-medium text-white/80 mb-2">ou</label>
                <input bind:value={ou} required placeholder="users" class="input-ldap" />
              </div>

              <div in:scale={{ delay: 500, duration: 600, start: 0.9 }}>
                <label class="block text-sm font-medium text-white/80 mb-2">dc (1)</label>
                <input bind:value={dc1} required placeholder="empresa" class="input-ldap" />
              </div>

              <div in:scale={{ delay: 600, duration: 600, start: 0.9 }}>
                <label class="block text-sm font-medium text-white/80 mb-2">dc (2)</label>
                <input bind:value={dc2} required placeholder="com" class="input-ldap" />
              </div>

              <div in:scale={{ delay: 700, duration: 600, start: 0.9 }}>
                <label class="block text-sm font-medium text-white/80 mb-2">cn</label>
                <input bind:value={cn} required placeholder="João Silva" class="input-ldap" />
              </div>

              <div in:scale={{ delay: 800, duration: 600, start: 0.9 }}>
                <label class="block text-sm font-medium text-white/80 mb-2">sn</label>
                <input bind:value={sn} required placeholder="Silva" class="input-ldap" />
              </div>

              <div in:scale={{ delay: 900, duration: 600, start: 0.9 }}>
                <label class="block text-sm font-medium text-white/80 mb-2">mail</label>
                <input type="email" bind:value={mail} required class="input-ldap" />
              </div>

              <div in:scale={{ delay: 1000, duration: 600, start: 0.9 }} class="md:col-span-2">
                <label class="block text-sm font-medium text-white/80 mb-2">Senha inicial</label>
                <input type="password" bind:value={userPassword} required placeholder="••••••••" class="input-ldap" />
              </div>

              <div class="md:col-span-2">
                <button
                  type="submit"
                  class="w-full py-5 mt-6 rounded-2xl font-bold text-lg bg-gradient-to-r from-emerald-500 to-cyan-600
                         text-white shadow-2xl hover:shadow-emerald-500/60 transform hover:scale-105
                         transition-all duration-500 group"
                >
                  Criar Usuário no LDAP
                  <span class="inline-block ml-3 group-hover:translate-x-3 transition-transform">✓</span>
                </button>
              </div>
            </form>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  

  @keyframes gradient-xy {
    0%, 100% { background-position: 0% 0%; }
    50% { background-position: 100% 100%; }
  }

  .animate-gradient-xy {
    background-size: 400% 400%;
    animation: gradient-xy 15s ease infinite;
  }

  .delay-700 { animation-delay: 0.7s; }

  @keyframes float {
    0%, 100% { transform: translateY(0) translateX(0); }
    50% { transform: translateY(-30px) translateX(20px); }
  }

  .animate-float { animation: float linear infinite; }

  @keyframes border-glow {
    0%, 100% { box-shadow: 0 0 20px rgba(168, 85, 247, 0.5); }
    50% { box-shadow: 0 0 40px rgba(34, 211, 238, 0.8); }
  }

  .animate-border-glow {
    animation: border-glow 8s ease-in-out infinite;
  }
</style>