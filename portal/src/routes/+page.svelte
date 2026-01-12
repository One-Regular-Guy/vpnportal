<script>
  let email = $state('');
  let password = $state('');
  let loading = $state(false);
  let error = $state('');

  async function handleLogin() {
    loading = true;
    error = '';

    // Simulação de login (substitua por sua lógica real, ex: fetch para API)
    try {
      await new Promise(resolve => setTimeout(resolve, 1000)); // delay fake
      if (email === 'teste@exemplo.com' && password === '123') {
        alert('Login sucesso!');
        // Redirecione com goto('/dashboard') ou similar
      } else {
        error = 'Credenciais inválidas';
      }
    } catch {
      error = 'Erro ao conectar';
    } finally {
      loading = false;
    }
  }
</script>

<div class="min-h-screen bg-gradient-to-br from-blue-950 to-indigo-900 flex items-center justify-center p-4">
  <div class="w-full max-w-md">
    <form
      on:submit|preventDefault={handleLogin}
      class="backdrop-blur-xl bg-white/10 rounded-3xl shadow-2xl border border-sky-400/30 p-8 space-y-8"
    >
      <div class="text-center">
        <h1 class="text-4xl font-bold text-amber-400 tracking-wide">Bem-vindo</h1>
        <p class="mt-2 text-sky-200/80">Entre com suas credenciais</p>
      </div>

      <div class="space-y-6">
        <div>
          <label class="block text-amber-300 text-sm font-medium mb-2" for="email">Email</label>
          <input
            id="email"
            type="email"
            bind:value={email}
            required
            class="w-full px-4 py-3 rounded-xl bg-white/5 border border-sky-500/40 text-white placeholder-sky-300/50 focus:outline-none focus:border-sky-300 focus:ring-4 focus:ring-sky-300/20 transition"
            placeholder="seu@email.com"
          />
        </div>

        <div>
          <label class="block text-amber-300 text-sm font-medium mb-2" for="password">Senha</label>
          <input
            id="password"
            type="password"
            bind:value={password}
            required
            class="w-full px-4 py-3 rounded-xl bg-white/5 border border-sky-500/40 text-white placeholder-sky-300/50 focus:outline-none focus:border-sky-300 focus:ring-4 focus:ring-sky-300/20 transition"
            placeholder="••••••••"
          />
        </div>
      </div>

      {#if error}
        <p class="text-red-300 text-center text-sm">{error}</p>
      {/if}

      <button
        type="submit"
        disabled={loading}
        class="w-full py-4 rounded-xl bg-gradient-to-r from-sky-500 to-sky-400 text-white font-semibold text-lg shadow-lg hover:from-sky-400 hover:to-sky-300 focus:outline-none focus:ring-4 focus:ring-sky-300/50 transition disabled:opacity-70"
      >
        {loading ? 'Entrando...' : 'Entrar'}
      </button>

      <p class="text-center text-sky-200/70 text-sm">
        Não tem conta? <a href="/register" class="text-amber-300 hover:underline">Cadastre-se</a>
      </p>
    </form>
  </div>
</div>