<script lang="ts">
  let email = $state('');
  let password = $state('');
  let loading = $state(false);
</script>

<div class="min-h-screen bg-linear-to-b from-gray-900 via-blue-950 to-gray-900 flex items-center justify-center p-4">
  <div class="w-full max-w-md">
    <!-- Card principal -->
    <div class="bg-gray-900/70 backdrop-blur-md border border-blue-900/40 rounded-xl shadow-2xl overflow-hidden">
      <!-- Cabeçalho -->
      <div class="bg-linear-to-r from-blue-900 to-blue-800 px-8 py-6 text-center border-b border-yellow-500/30">
        <h1 class="text-3xl font-bold text-white tracking-wider">
          FAB <span class="text-yellow-400">LOGIN</span>
        </h1>
        <p class="text-blue-300/80 mt-1 text-sm">Sistemas de Gerenciamento</p>
      </div>

      <!-- Formulário -->
      <form method="POST" action="?/login" class="p-8 space-y-6">
        <div>
          <label for="email" class="block text-sm font-medium text-gray-300 mb-1">
            E-mail institucional
          </label>
          <input
            id="email"
            name="email"
            type="email"
            required
            bind:value={email}
            class="w-full px-4 py-3 bg-gray-800/60 border border-gray-700 rounded-lg 
                   text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 
                   focus:ring-2 focus:ring-blue-500/30 transition-all"
            placeholder="seunome@fab.mil.br"
          />
        </div>

        <div>
          <label for="password" class="block text-sm font-medium text-gray-300 mb-1">
            Senha
          </label>
          <input
            id="password"
            name="password"
            type="password"
            required
            bind:value={password}
            class="w-full px-4 py-3 bg-gray-800/60 border border-gray-700 rounded-lg 
                   text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 
                   focus:ring-2 focus:ring-blue-500/30 transition-all"
            placeholder="••••••••"
          />
        </div>

        <button
          type="submit"
          disabled={loading}
          class="w-full py-3 px-4 bg-blue-600 hover:bg-blue-500 active:bg-blue-700 
                 text-white font-medium rounded-lg shadow-lg transform transition-all
                 disabled:opacity-50 disabled:cursor-not-allowed
                 border-b-4 border-blue-800 hover:border-blue-700"
        >
          {#if loading}
            <span class="inline-block animate-spin mr-2">⟳</span>
            Autenticando...
          {:else}
            ENTRAR
          {/if}
        </button>

        <div class="text-center text-sm text-gray-400 mt-4">
          <a href="/recuperar-senha" class="text-blue-400 hover:text-blue-300 transition-colors">
            Esqueceu a senha?
          </a>
        </div>
      </form>
    </div>

    <!-- Marca d'água sutil -->
    <div class="text-center mt-6 text-blue-800/30 text-xs tracking-widest">
      FORÇA AÉREA BRASILEIRA
    </div>
  </div>
</div>