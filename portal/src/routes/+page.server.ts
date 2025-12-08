import type { Actions } from './$types';
import { fail, redirect } from '@sveltejs/kit';

export const actions = {
	default: async ({ cookies, request }) => {
		// Extrai e valida os dados do formulário
        const data = await request.formData();
        const email = data.get('email')?.toString();
        const password = data.get('password')?.toString();

        // Validação básica
        if (!email || !password) {
        return fail(400, { error: 'Email e senha são obrigatórios' });
        }

        try {
            // Faz a requisição à API
            const response = await fetch('http://localhost:8080/login', {
                method: 'POST',
                headers: {
                'Content-Type': 'application/json'
                },
                body: JSON.stringify({ email, password })
            });
            console.log(response.status);
            // Verifica se a resposta foi bem-sucedida
            if (response.status != 200) {
                const errorData = await response.json().catch(() => ({}));
                return fail(response.status, {
                error: errorData.message || 'Falha na autenticação'
                });
            }

            // Extrai os dados da resposta (ex.: token, session ID)
            const result = await response.json();
            const sessionId = result.token; // Ajuste conforme a resposta da API

            // Configura o cookie de sessão
            cookies.set('sessionid', sessionId, {
                path: '/',
                httpOnly: true,
                secure: process.env.NODE_ENV === 'production', // Apenas HTTPS em produção
                sameSite: 'strict',
                maxAge: 60 * 60 * 24 // 1 dia
            });

            // Opcional: redireciona após login bem-sucedido
            //throw redirect(303, '/users');

            // Ou retorna sucesso se não for redirecionar
            return { success: true };
        } catch (err) {
        // Trata erros de rede ou outros
        return fail(500, { error: 'Erro interno do servidor' });
        }
	}
} satisfies Actions;