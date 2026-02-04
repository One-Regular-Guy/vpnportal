import type { Actions } from './$types';
import { fail, redirect } from '@sveltejs/kit';

export const actions = {
    default: async ({ cookies, request }) => {
        // Extrai e valida os dados do formulário
        const data = await request.formData();
        const name = data.get('nome')?.toString();
        const email = data.get('email')?.toString();
        const password = data.get('password')?.toString();
        const passwordConfirm = data.get('passwordConfirm')?.toString();
        console.log("email: "+email+"\nnome: "+name+"\npassword: "+password+"\nconfirma: "+passwordConfirm);

        // Validação básica
        if (!name || !email || !password) {
            return fail(400, { error: 'Nome, Email e senha são obrigatórios' });
        }

        if (password != passwordConfirm){
            return fail(400, { error: 'Senhas nao coincidem' });
        }
        
        let corpo = JSON.stringify({ name, email, password });
        try {
            // Faz a requisição à API
            const response = await fetch('http://localhost:8080/register', {
                method: 'POST',
                headers: {
                'Content-Type': 'application/json'
                },
                body: corpo
            });
            console.log(response.status);
            console.log(corpo);
            // Verifica se a resposta foi bem-sucedida
            if (response.status != 201) {
                const errorData = await response.json().catch(() => ({}));
                return fail(response.status, {
                error: errorData.message || 'Falha no Registro'
                });
            }
            return { success: true };
        } catch (err) {
        // Trata erros de rede ou outros
        return fail(500, { error: 'Erro interno do servidor' });
        }
    }
} satisfies Actions;