<script lang="ts">
	import { enhance } from "$app/forms";
  import { goto } from '$app/navigation';

  let loading = $state(false);
  let data = $props();
  $effect(() => {
		if (data.form.success) {
			goto('/login', {
				replaceState: true
			});
		}
	});
</script>

<div class="min-h-screen bg-linear-to-b from-gray-900 via-blue-950 to-gray-900 flex items-center justify-center p-4">
  <div class="w-full max-w-md">
    <div class="bg-gray-900/70 backdrop-blur-md border border-blue-900/40 rounded-xl shadow-2xl overflow-hidden">
      <!-- Cabeçalho -->
      <div class="bg-linear-to-r from-blue-900 to-blue-800 px-8 py-6 text-center border-b border-yellow-500/30">
        <h1 class="text-3xl font-bold text-white tracking-wider">
          FAB <span class="text-yellow-400">CADASTRO</span>
        </h1>
        <p class="text-blue-300/80 mt-1 text-sm">Novo Usuário</p>
      </div>

      <form method="POST" class="p-8 space-y-5" use:enhance>
        <div>
          <label for="nome" class="block text-sm font-medium text-gray-300 mb-1">
            Nome completo
          </label>
          <input
            id="nome"
            name="nome"
            type="text"
            required
            class="w-full px-4 py-3 bg-gray-800/60 border border-gray-700 rounded-lg 
                   text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 
                   focus:ring-2 focus:ring-blue-500/30 transition-all"
            placeholder="Nome Sobrenome"
          />
        </div>

        <div>
          <label for="email" class="block text-sm font-medium text-gray-300 mb-1">
            E-mail institucional
          </label>
          <input
            id="email"
            name="email"
            type="email"
            required
            class="w-full px-4 py-3 bg-gray-800/60 border border-gray-700 rounded-lg 
                   text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 
                   focus:ring-2 focus:ring-blue-500/30 transition-all"
            placeholder="seunome@fab.mil.br"
          />
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label for="password" class="block text-sm font-medium text-gray-300 mb-1">
              Senha
            </label>
            <input
              id="password"
              name="password"
              type="password"
              required
              minlength="8"
              class="w-full px-4 py-3 bg-gray-800/60 border border-gray-700 rounded-lg 
                     text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 
                     focus:ring-2 focus:ring-blue-500/30 transition-all"
              placeholder="••••••••"
            />
          </div>

          <div>
            <label for="passwordConfirm" class="block text-sm font-medium text-gray-300 mb-1">
              Confirme
            </label>
            <input
              id="passwordConfirm"
              name="passwordConfirm"
              type="password"
              required
              class="w-full px-4 py-3 bg-gray-800/60 border border-gray-700 rounded-lg 
                     text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 
                     focus:ring-2 focus:ring-blue-500/30 transition-all"
              placeholder="••••••••"
            />
          </div>
        </div>

        <button
          type="submit"
          disabled={loading}
          class="w-full mt-2 py-3 px-4 bg-blue-600 hover:bg-blue-500 active:bg-blue-700 
                 text-white font-medium rounded-lg shadow-lg transform transition-all
                 disabled:opacity-50 disabled:cursor-not-allowed
                 border-b-4 border-blue-800 hover:border-blue-700"
        >
          {#if loading}
            <span class="inline-block animate-spin mr-2">⟳</span>
            Cadastrando...
          {:else}
            CRIAR CONTA
          {/if}
        </button>

        <div class="text-center text-sm text-gray-400 mt-4">
          Já possui conta? 
          <a href="/login" class="text-blue-400 hover:text-blue-300 transition-colors">
            Entrar
          </a>
        </div>
      </form>
    </div>

    <div class="text-center mt-6 text-blue-800/30 text-xs tracking-widest">
      FORÇA AÉREA BRASILEIRA
    </div>
  </div>
</div>