/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo Raiz da Interface de Usuário (UI) do Terminal
//
// Este módulo encapsula toda a lógica relacionada à apresentação de informações
// e à interação com o usuário no console.
//
// RESPONSABILIDADES:
// - Desenhar menus de seleção interativos.
// - Solicitar entradas de texto, confirmações e edições multi-linha (prompts).
// - Exibir tabelas de dados formatadas.
// - Mostrar indicadores de progresso (spinners, barras).
// - Controlar cores e estilos do texto no terminal.
//
// Este módulo se concentra no "COMO" as informações são apresentadas, separando
// a lógica de apresentação da lógica de negócio da aplicação.
// ==============================================================================

// --- Declaração de Sub-módulos ---
// Declaramos os arquivos que compõem o módulo `ui`. O `pub` na frente de `mod`
// torna o sub-módulo acessível a partir de outros módulos fora de `ui`
// (por exemplo, para que `main.rs` possa chamar `ui::menus::...` ou `ui::prompts::...`).

/// Módulo para renderizar menus de seleção interativos.
pub mod menus;

/// Módulo para solicitar entradas de texto, senhas e confirmações do usuário.
pub mod prompts;

// NOTA DE ARQUITETURA:
// Mantemos os namespaces explícitos (ex: `ui::menus::show_main_menu()` em vez de
// `ui::show_main_menu()`) para maior clareza sobre de onde cada funcionalidade
// de UI está vindo. Isso torna o código mais fácil de ler e entender, especialmente
// à medida que o módulo `ui` cresce.