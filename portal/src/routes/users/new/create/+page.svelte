<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';

  const email = $page.url.searchParams.get('email') || '';

  if (!email) {
    goto('/users/new');
  }

  let uid = $state('');
  let cn = $state('');
  let sn = $state('');
  let password = $state('');
  let confirmPassword = $state('');
  let is_ldap = $state(false);
  let is_active = $state(true);
  let loading = $state(false);
  let error = $state('');

  async function handleCreate() {
    loading = true;
    error = '';

    if (!uid || !cn || !sn) {
      error = 'Preencha todos os campos obrigatórios';
      loading = false;
      return;
    }

    if (password !== confirmPassword) {
      error = 'As senhas não coincidem';
      loading = false;
      return;
    }

    if (password.length < 6) {
      error = 'A senha deve ter pelo menos 6 caracteres';
      loading = false;
      return;
    }

    try {
      await new Promise(r => setTimeout(r, 1000));
      alert('Usuário criado com sucesso!');
      goto('/users');
    } catch {
      error = 'Erro ao criar usuário';
    } finally {
      loading = false;
    }
  }
</script>

<div class="min-h-screen bg-gradient-to-br from-blue-950 to-indigo-900 flex items-center justify-center p-6">
  <div class="w-full max-w-2xl">
    <form
      on:submit|preventDefault={handleCreate}
      class="backdrop-blur-xl bg-white/10 rounded-3xl shadow-2xl border border-sky-400/30 p-10 space-y-8"
    >
      <div class="flex items-center justify-between mb-6">
        <h1 class="text-4xl font-bold text-amber-400">Criar Novo Usuário</h1>
        <a href="/users/new" class="text-sky-300 hover:text-white transition">← Voltar</a>
      </div>

      <p class="text-sky-200/90 text-lg">E-mail: <span class="text-amber-300 font-medium">{email}</span></p>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div>
          <label class="block text-amber-300 text-sm font-medium mb-2">UID (login)</label>
          <input bind:value={uid} required placeholder="joao.silva" class="w-full px-4 py-3 rounded-xl bg-white/5 border border-sky-500/40 text-white placeholder-sky-300/50 focus:outline-none focus:border-sky-300 focus:ring-4 focus:ring-sky-300/20 transition" />
        </div>

        <div>
          <label class="block text-amber-300 text-sm font-medium mb-2">Nome Completo</label>
          <input bind:value={cn} required placeholder="João Silva" class="w-full px-4 py-3 rounded-xl bg-white/5 border border-sky-500/40 text-white placeholder-sky-300/50 focus:outline-none focus:border-sky-300 focus:ring-4 focus:ring-sky-300/20 transition" />
        </div>

        <div>
          <label class="block text-amber-300 text-sm font-medium mb-2">Sobrenome</label>
          <input bind:value={sn} required placeholder="Silva" class="w-full px-4 py-3 rounded-xl bg-white/5 border border-sky-500/40 text-white placeholder-sky-300/50 focus:outline-none focus:border-sky-300 focus:ring-4 focus:ring-sky-300/20 transition" />
        </div>

        <div>
          <label class="block text-amber-300 text-sm font-medium mb-2">Senha</label>
          <input bind:value={password} type="password" required class="w-full px-4 py-3 rounded-xl bg-white/5 border border-sky-500/40 text-white placeholder-sky-300/50 focus:outline-none focus:border-sky-300 focus:ring-4 focus:ring-sky-300/20 transition" />
        </div>

        <div class="md:col-span-2">
          <label class="block text-amber-300 text-sm font-medium mb-2">Confirmar Senha</label>
          <input bind:value={confirmPassword} type="password" required class="w-full px-4 py-3 rounded-xl bg-white/5 border border-sky-500/40 text-white placeholder-sky-300/50 focus:outline-none focus:border-sky-300 focus:ring-4 focus:ring-sky-300/20 transition" />
        </div>
      </div>

      <div class="flex flex-col sm:flex-row items-center gap-8 py-4">
        <label class="flex items-center gap-3 text-sky-200 cursor-pointer">
          <input type="checkbox" bind:checked={is_ldap} class="w-5 h-5 rounded border-sky-400/50 text-sky-400 focus:ring-sky-300" />
          <span>Sincronizado via LDAP</span>
        </label>

        <label class="flex items-center gap-3 text-sky-200 cursor-pointer">
          <input type="checkbox" bind:checked={is_active} class="w-5 h-5 rounded border-sky-400/50 text-sky-400 focus:ring-sky-300" />
          <span>Ativar usuário imediatamente</span>
        </label>
      </div>

      {#if error}
        <p class="text-red-300 text-center">{error}</p>
      {/if}

      <div class="flex gap-4">
        <button
          type="submit"
          disabled={loading}
          class="flex-1 py-4 rounded-xl bg-gradient-to-r from-sky-500 to-sky-400 text-white font-semibold text-lg shadow-lg hover:from-sky-400 hover:to-sky-300 transition disabled:opacity-70"
        >
          {loading ? 'Criando...' : 'Criar Usuário'}
        </button>

        <a href="/users" class="px-8 py-4 rounded-xl bg-white/10 border border-sky-400/50 text-sky-300 hover:bg-white/20 transition text-center">
          Cancelar
        </a>
      </div>
    </form>
  </div>
</div>