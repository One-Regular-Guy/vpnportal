<script>
  let name = $state('');
  let email = $state('');
  let password = $state('');
  let confirmPassword = $state('');
  let loading = $state(false);
  let error = $state('');

  async function handleRegister() {
    loading = true;
    error = '';

    // Validações básicas
    if (!name.trim()) {
      error = 'Nome é obrigatório';
    } else if (!email.includes('@')) {
      error = 'Email inválido';
    } else if (password.length < 6) {
      error = 'Senha deve ter pelo menos 6 caracteres';
    } else if (password !== confirmPassword) {
      error = 'As senhas não coincidem';
    } else {
      // Simulação de registro (substitua pela sua lógica real)
      try {
        await new Promise(resolve => setTimeout(resolve, 1000));
        alert('Cadastro realizado com sucesso!');
        // Redirecione com goto('/login') ou similar
      } catch {
        error = 'Erro ao cadastrar';
      }
    }

    loading = false;
  }
</script>

<div class="min-h-screen bg-gradient-to-br from-blue-950 to-indigo-900 flex items-center justify-center p-4">
  <div class="w-full max-w-md">
    <form
      on:submit|preventDefault={handleRegister}
      class="backdrop-blur-xl bg-white/10 rounded-3xl shadow-2xl border border-sky-400/30 p-8 space-y-8"
    >
      <div class="text-center">
        <h1 class="text-4xl font-bold text-amber-400 tracking-wide">Criar Conta</h1>
        <p class="mt-2 text-sky-200/80">Preencha os dados abaixo</p>
      </div>

      <div class="space-y-6">
        <!-- Nome -->
        <div>
          <label class="block text-amber-300 text-sm font-medium mb-2" for="name">Nome</label>
          <input
            id="name"
            type="text"
            bind:value={name}
            required
            class="w-full px-4 py-3 rounded-xl bg-white/5 border border-sky-500/40 text-white placeholder-sky-300/50 focus:outline-none focus:border-sky-300 focus:ring-4 focus:ring-sky-300/20 transition"
            placeholder="Seu nome completo"
          />
        </div>

        <!-- Email -->
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

        <!-- Senha -->
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

        <!-- Confirmação de Senha -->
        <div>
          <label class="block text-amber-300 text-sm font-medium mb-2" for="confirm">Confirmar Senha</label>
          <input
            id="confirm"
            type="password"
            bind:value={confirmPassword}
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
        {loading ? 'Criando conta...' : 'Cadastrar'}
      </button>

      <p class="text-center text-sky-200/70 text-sm">
        Já tem conta? <a href="/login" class="text-amber-300 hover:underline">Entre aqui</a>
      </p>
    </form>
  </div>
</div>