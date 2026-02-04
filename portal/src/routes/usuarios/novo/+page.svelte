<script>
</script>

<div class="min-h-screen bg-linear-to-b from-gray-900 via-blue-950 to-gray-900 flex items-center justify-center p-4">
  <div class="w-full max-w-lg">
    <div class="bg-gray-900/70 backdrop-blur-md border border-blue-900/40 rounded-xl shadow-2xl overflow-hidden">
      <div class="bg-linear-to-r from-blue-900 to-blue-800 px-8 py-6 text-center border-b border-yellow-500/30">
        <h1 class="text-3xl font-bold text-white tracking-wider">
          Novo Usuário <span class="text-yellow-400">LDAP</span>
        </h1>
      </div>

      <form method="POST" action="?/create" class="p-8 space-y-5">
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label for="name" class="block text-sm text-gray-300 mb-1">Nome</label>
            <input type="text" name="cn" required 
                   class="w-full px-4 py-3 bg-gray-800/60 border border-gray-700 rounded-lg text-white" />
          </div>
          <div>
            <label for="sn" class="block text-sm text-gray-300 mb-1">Sobrenome</label>
            <input type="text" name="sn" required 
                   class="w-full px-4 py-3 bg-gray-800/60 border border-gray-700 rounded-lg text-white" />
          </div>
        </div>

        <div>
          <label for="mail" class="block text-sm text-gray-300 mb-1">E-mail institucional</label>
          <input type="email" name="mail" required 
                 placeholder="nome.sobrenome@fab.mil.br"
                 class="w-full px-4 py-3 bg-gray-800/60 border border-gray-700 rounded-lg text-white" />
        </div>

        <div>
          <label for="uid" class="block text-sm text-gray-300 mb-1">UID / Login</label>
          <input type="text" name="uid" required 
                 placeholder="nomeguerra"
                 class="w-full px-4 py-3 bg-gray-800/60 border border-gray-700 rounded-lg text-white font-mono" />
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label for="password" class="block text-sm text-gray-300 mb-1">Senha</label>
            <input type="password" name="password" required minlength="8" 
                   class="w-full px-4 py-3 bg-gray-800/60 border border-gray-700 rounded-lg text-white" />
          </div>
          <div>
            <label for="password_confirm" class="block text-sm text-gray-300 mb-1">Confirme</label>
            <input type="password" name="password_confirm" required 
                   class="w-full px-4 py-3 bg-gray-800/60 border border-gray-700 rounded-lg text-white" />
          </div>
        </div>

        <div class="flex justify-between mt-8">
          <a href="/usuarios" class="px-6 py-3 bg-gray-700 hover:bg-gray-600 rounded-lg text-white">
            Voltar
          </a>
          <button type="submit"
                  class="px-8 py-3 bg-blue-600 hover:bg-blue-500 active:bg-blue-700 
                         text-white font-medium rounded-lg shadow-lg border-b-4 border-blue-800">
            Criar Usuário
          </button>
        </div>
      </form>
    </div>
  </div>
</div>