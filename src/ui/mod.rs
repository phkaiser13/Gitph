/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo Raiz do Wrapper Git
//
// Este módulo é a única interface autorizada para interagir com o executável
// do Git no sistema. Ele abstrai a complexidade da execução de comandos
// externos, captura de saída e tratamento de erros específicos do Git.
//
// RESPONSABILIDADES:
// - Construir e executar comandos `git`.
// - Capturar `stdout` e `stderr` dos processos Git.
// - Converter códigos de saída e erros de `stderr` em erros Rust significativos.
// - Fornecer uma API segura e idiomática para operações Git comuns.
//
// Nenhum outro módulo na aplicação deve chamar `std::process::Command` para
// executar `git` diretamente. Toda a interação deve passar por este wrapper.
// ==============================================================================

// --- Declaração de Sub-módulos ---
// Cada arquivo dentro do diretório `git_wrapper` representa uma categoria de
// comandos Git. O `pub mod` torna o sub-módulo e seu conteúdo público
// acessível para outros módulos que usam `git_wrapper`.

/// Módulo contendo a lógica para o comando `git status`.
pub mod status;

/// Módulo contendo a lógica para os comandos `git add` e `git commit`.
pub mod commit;

/// Módulo contendo a lógica para o comando `git push`.
pub mod push;

// À medida que implementarmos mais funcionalidades, adicionaremos mais módulos aqui.
// Ex: pub mod clone;

/// Módulo para renderizar menus de seleção interativos.
pub mod menus;

/// Módulo para solicitar entradas de texto, senhas e confirmações do usuário.
pub mod prompts; // Adicionamos esta linha para declarar o novo módulo.