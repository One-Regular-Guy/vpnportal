<script lang="ts">
  let email = $state('');
  let loading = $state(false);
  let error = $state('');
  let userExists = $state(false);
  let showCreateForm = $state(false);

  async function checkEmail() {
    loading = true;
    error = '';

    if (!email.includes('@') || !email.includes('.')) {
      error = 'Digite um e-mail válido';
      loading = false;
      return;
    }

    // Substitua por sua requisição real à API Rust
    // Ex: const res = await fetch(`/api/users/check?mail=${encodeURIComponent(email)}`);
    try {
      await new Promise(r => setTimeout(r, 800)); // simulação

      // Simulação: se o email terminar com .admin existe, senão não
      const exists = email.endsWith('.admin');

      if (exists) {
        userExists = true;
        error = 'Este e-mail já está cadastrado';
      } else {
        showCreateForm = true; // Avança para o formulário completo
      }
    } catch {
      error = 'Erro ao verificar e-mail';
    } finally {
      loading = false;
    }
  }
</script>

<div class="min-h-screen bg-gradient-to-br from-blue-950 to-indigo-900 flex items-center justify-center p-6">
  <div class="w-full max-w-lg">
    <div class="backdrop-blur-xl bg-white/10 rounded-3xl shadow-2xl border border-sky-400/30 p-10 text-center">
      
      <h1 class="text-4xl font-bold text-amber-400 mb-4 tracking-wide">Novo Usuário</h1>
      <p class="text-sky-200/80 mb-10">Informe o e-mail para começar</p>

      {#if !showCreateForm}
        <form on:submit|preventDefault={checkEmail} class="space-y-6">
          <div>
            <input
              bind:value={email}
              type="email"
              required
              placeholder="email@empresa.com"
              class="w-full px-6 py-4 text-lg rounded-xl bg-white/5 border border-sky-500/40 text-white placeholder-sky-300/60
                     focus:outline-none focus:border-sky-300 focus:ring-4 focus:ring-sky-300/30 transition"
            />
          </div>

          {#if error}
            <p class="{userExists ? 'text-red-300' : 'text-orange-300'} text-sm">{error}</p>
          {/if}

          <button
            type="submit"
            disabled={loading || !email}
            class="w-full py-4 rounded-xl bg-gradient-to-r from-sky-500 to-sky-400 text-white font-semibold text-lg shadow-lg
                   hover:from-sky-400 hover:to-sky-300 focus:outline-none focus:ring-4 focus:ring-sky-300/50 transition disabled:opacity-70"
          >
            {loading ? 'Verificando...' : 'Continuar'}
          </button>
        </form>

        <a href="/users" class="block mt-8 text-sky-300 hover:text-white transition">
          ← Voltar para lista de usuários
        </a>
      {/if}

      <!-- Etapa 2: Formulário completo (aparece após verificação) -->
      {#if showCreateForm}
        <!-- Aqui vamos usar um componente ou transição, mas por minimalismo, vamos carregar dinamicamente -->
        <!-- Em produção, use <svelte:component> ou load() do SvelteKit -->
        <div class="text-left">
          <p class="text-sky-200 mb-8">E-mail válido! Agora complete os dados do novo usuário.</p>
          
          <!-- Inclua o formulário completo aqui ou use load em +page.server.ts -->
          <!-- Por enquanto, redirecionamos para uma subpágina (recomendado) -->
          <script>
            import { onMount } from 'svelte';
            onMount(() => {
              // Melhor abordagem: redirecionar para uma rota com parâmetro
              location.href = `/users/new/create?email=${encodeURIComponent(email)}`;
            });
          </script>
        </div>
      {/if}
    </div>
  </div>
</div>