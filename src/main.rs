/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Ponto de Entrada da Aplicação 'gitph'
//
// Este arquivo contém a função `main`, que é o início da execução do programa.
//
// RESPONSABILIDADES:
// - Declarar a estrutura de módulos da aplicação.
// - Orquestrar o fluxo inicial, iniciando a interface de usuário interativa.
// - Servir como o ponto mais alto para o tratamento de erros propagados.
// ==============================================================================

// --- Declaração de Módulos ---
mod api_client;
mod cli;
mod config;
mod git_wrapper;
mod native_bindings;
mod ui;

// --- Importações (`use`) ---
use anyhow::Result;

/// Função principal que é executada quando o programa inicia.
///
/// Retorna um `anyhow::Result<()>` que permite o uso do operador `?` para
/// propagar erros de forma concisa de qualquer parte da aplicação até o topo,
// onde serão impressos de forma amigável caso ocorram.
fn main() -> Result<()> {
    // Inicia o painel interativo principal da aplicação.
    // Toda a lógica da aplicação flui a partir desta chamada.
    ui::menus::show_main_menu()?;

    // Se a execução chegar até aqui sem erros (o usuário saiu do menu),
    // retornamos `Ok(())` para indicar um encerramento bem-sucedido.
    Ok(())
}