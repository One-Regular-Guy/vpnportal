<script lang="ts">
  import { onMount } from 'svelte';
  import { fly, fade, scale } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';
	import { enhance } from '$app/forms';
	import { redirect } from '@sveltejs/kit';
	import { goto } from '$app/navigation';

  let email = $state('');
  let password = $state('');
  let isLoading = $state(false);
  onMount(() => {
    document.body.classList.add('overflow-hidden');
    return () => document.body.classList.remove('overflow-hidden');
  });
</script>

<svelte:head>
  <title>Bem-vindo • Login</title>
</svelte:head>

<!-- Fundo animado com gradiente + partículas -->
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
    out:fly={{ y: -60, duration: 600 }}
    class="relative max-w-md w-full"
  >
    <!-- Card principal com glassmorphism e animações -->
    <div
      class="relative backdrop-blur-2xl bg-white/10 border border-white/20 rounded-3xl shadow-2xl overflow-hidden
             before:absolute before:inset-0 before:bg-gradient-to-br before:from-white/10 before:to-transparent before:rounded-3xl
             animate-border-glow"
    >
      <div class="absolute inset-0 bg-gradient-to-br from-purple-600/20 via-transparent to-pink-600/20 animate-pulse"></div>

      <div class="relative p-10">
        <!-- Título com animação escalonada -->
        <div in:fade={{ delay: 300, duration: 800 }}>
          <h1 class="text-5xl font-bold text-white text-center mb-2 tracking-tight">
            {#each "Bem-vindo" .split('') as char, i}
              <span
                in:fly={{ y: 50, delay: 500 + i * 100, duration: 800, easing: quintOut }}
                class="inline-block hover:text-cyan-300 transition-colors duration-300"
              >
                {char === ' ' ? '\u00A0' : char}
              </span>
            {/each}
          </h1>
        </div>

        <p in:fade={{ delay: 1200, duration: 1000 }} class="text-center text-white/70 mb-10">
          Entre com sua conta
        </p>

        <!-- Formulário -->
        <form method="POST" class="space-y-6" use:enhance={() => {
            isLoading = true;
            return async ({ result }) => {
              isLoading = false;
              if (result.type === 'success' && result.data?.success) {
                goto('/users');
              }
            };
          }}>
          <div in:scale={{ delay: 1400, duration: 600, start: 0.9 }}>
            <label class="block text-sm font-medium text-white/80 mb-2">Email</label>
            <input
              type="email"
              bind:value={email}
              id="email"
              name="email"
              required
              placeholder="seu@email.com"
              class="w-full px-5 py-4 bg-white/10 border border-white/20 rounded-xl text-white placeholder-white/40
                     focus:outline-none focus:border-cyan-400 focus:ring-4 focus:ring-cyan-400/30
                     transition-all duration-500 hover:bg-white/20"
            />
            
          </div>

          <div in:scale={{ delay: 1600, duration: 600, start: 0.9 }}>
            <label class="block text-sm font-medium text-white/80 mb-2">Senha</label>
            <input
              type="password"
              bind:value={password}
              id="password"
              name="password"
              required
              placeholder="••••••••"
              class="w-full px-5 py-4 bg-white/10 border border border-white/20 rounded-xl text-white placeholder-white/40
                     focus:outline-none focus:border-purple-400 focus:ring-4 focus:ring-purple-400/30
                     transition-all duration-500 hover:bg-white/20"
            />
          </div>

          <!-- Botão com múltiplas animações -->
          <button
            type="submit"
            disabled={isLoading}
            class="relative w-full py-5 mt-8 rounded-xl font-semibold text-lg overflow-hidden
                   bg-gradient-to-r from-cyan-500 to-purple-600 text-white
                   shadow-lg hover:shadow-cyan-500/50 transform hover:scale-105 active:scale-95
                   transition-all duration-500 group"
          >
            <span class="relative z-10 flex items-center justify-center gap-3">
              {#if isLoading}
                <svg class="w-6 h-6 animate-spin" viewBox="0 0 24 24">
                  <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none" opacity="0.3"/>
                  <path fill="currentColor" d="M4 12a8 8 0 018-8v8z"/>
                </svg>
                Entrando...
              {:else}
                Entrar
                <span class="inline-block transition-transform group-hover:translate-x-2">→</span>
              {/if}
            </span>

            <!-- Efeito de brilho no botão -->
            <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white to-transparent opacity-0 
                          group-hover:opacity-50 translate-x-[-150%] group-hover:translate-x-[150%] 
                          transition-transform duration-1000 skew-x-12"></div>
          </button>
        </form>

        <p in:fade={{ delay: 2000 }} class="text-center text-white/50 text-sm mt-8">
          Não tem conta? <a href="#" class="text-cyan-300 hover:underline">Cadastre-se</a>
        </p>
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
    animation:global(&) {
      animation: gradient-xy 15s ease infinite;
    }
  }

  .delay-700 {
    animation-delay: 0.7s;
  }

  @keyframes float {
    0%, 100% { transform: translateY(0) translateX(0); }
    50% { transform: translateY(-30px) translateX(20px); }
  }

  .animate-float {
    animation: float linear infinite;
  }

  @keyframes border-glow {
    0%, 100% { box-shadow: 0 0 20px rgba(168, 85, 247, 0.5); }
    50% { box-shadow: 0 0 40px rgba(34, 211, 238, 0.8); }
  }

  .animate-border-glow {
    animation: border-glow 8s ease-infinite;
  }
</style>