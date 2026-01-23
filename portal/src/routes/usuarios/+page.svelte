<script lang="ts">
  import type { PageData } from './$types';
  import { enhance } from '$app/forms';
  import type { Client } from '$lib/types';

  let { data }: { data: PageData } = $props();
  
  let users = $state<Client[]>(data.users ?? []);
  
  let showConfirmModal = $state(false);
  let userToToggle = $state<Client | null>(null);

  function prepareToggle(user: Client) {
    userToToggle = user;
    showConfirmModal = true;
  }
  
  function cancelToggle() {
    showConfirmModal = false;
    userToToggle = null;
  }
</script>

<div class="min-h-screen bg-gradient-to-b from-gray-900 via-blue-950 to-gray-900 p-6">
  <div class="max-w-7xl mx-auto">
    <div class="flex justify-between items-center mb-8">
      <h1 class="text-3xl font-bold text-white tracking-wide">
        Usuários <span class="text-yellow-400">LDAP</span>
      </h1>
      
      <a href="/usuarios/novo" 
         class="px-6 py-3 bg-blue-600 hover:bg-blue-500 active:bg-blue-700 
                text-white font-medium rounded-lg shadow-lg border-b-4 border-blue-800 
                transition-all hover:border-blue-700">
        + Novo Usuário
      </a>
    </div>

    <div class="bg-gray-900/70 backdrop-blur-md border border-blue-900/40 rounded-xl overflow-hidden shadow-2xl">
      <div class="overflow-x-auto">
        <table class="w-full text-gray-200">
          <thead class="bg-blue-950/70">
            <tr>
              <th class="px-6 py-4 text-left font-medium">Nome</th>
              <th class="px-6 py-4 text-left font-medium">E-mail</th>
              <th class="px-6 py-4 text-left font-medium">UID</th>
              <th class="px-6 py-4 text-center font-medium">Status</th>
              <th class="px-6 py-4 text-center font-medium">Ações</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-800">
            {#each users as user (user.uid)}
              <tr class="hover:bg-blue-950/30 transition-colors">
                <td class="px-6 py-4">{user.cn} {user.sn}</td>
                <td class="px-6 py-4">{user.mail}</td>
                <td class="px-6 py-4 font-mono text-sm">{user.uid}</td>
                <td class="px-6 py-4 text-center">
                  <span class={`inline-block px-3 py-1 rounded-full text-xs font-medium
                    ${user.is_active 
                      ? 'bg-green-900/70 text-green-300 border border-green-700/50' 
                      : 'bg-red-900/70 text-red-300 border border-red-700/50'}`}>
                    {user.is_active ? 'ATIVO' : 'INATIVO'}
                  </span>
                </td>
                <td class="px-6 py-4 text-center flex justify-center gap-3">
                  <button
                    on:click={() => prepareToggle(user)}
                    class={`px-4 py-1.5 rounded-lg text-sm font-medium transition-all
                      ${user.is_active 
                        ? 'bg-red-700/80 hover:bg-red-600 border-b-2 border-red-900' 
                        : 'bg-green-700/80 hover:bg-green-600 border-b-2 border-green-900'}
                      text-white`}
                  >
                    {user.is_active ? 'Desativar' : 'Ativar'}
                  </button>
                  
                  <a href={`/usuarios/${user.uid}`}
                     class="px-4 py-1.5 bg-gray-700 hover:bg-gray-600 rounded-lg text-sm font-medium
                            border-b-2 border-gray-900 transition-all text-white">
                    Detalhes
                  </a>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <!-- Modal de confirmação -->
    {#if showConfirmModal && userToToggle}
      <div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50 p-4">
        <div class="bg-gray-900 border border-blue-900/50 rounded-xl p-8 max-w-md w-full shadow-2xl">
          <h2 class="text-xl font-bold text-white mb-4">Confirmação</h2>
          
          <p class="text-gray-300 mb-6">
            Deseja realmente {#if userToToggle.is_active}desativar{:else}ativar{/if} o usuário<br/>
            <span class="font-semibold text-blue-300">{userToToggle.cn} {userToToggle.sn}</span>?
          </p>

          <div class="flex justify-end gap-4">
            <button
              on:click={cancelToggle}
              class="px-6 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg text-white">
              Cancelar
            </button>
            
            <form method="POST" action="?/toggleStatus" use:enhance>
              <input type="hidden" name="uid" value={userToToggle.uid} />
              <input type="hidden" name="current_status" value={userToToggle.is_active} />
              
              <button type="submit"
                      class={`px-6 py-2 rounded-lg text-white font-medium
                        ${userToToggle.is_active 
                          ? 'bg-red-600 hover:bg-red-500' 
                          : 'bg-green-600 hover:bg-green-500'}`}>
                Confirmar
              </button>
            </form>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>