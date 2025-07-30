/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Módulo Raiz para Clientes de API
//
// Este módulo serve como um ponto de entrada e encapsulamento para todos os
// clientes que interagem com APIs web externas, como GitHub, GitLab, etc.
//
// A principal responsabilidade arquitetural deste módulo é isolar completamente
// a lógica de comunicação de rede do resto da aplicação. Se uma API mudar,
// apenas este módulo (e seus sub-módulos) precisará ser atualizado.
// ==============================================================================

/// Módulo para interações com a API REST do GitHub.
pub mod github;

// No futuro, poderíamos adicionar outros clientes aqui, mantendo a organização:
// pub mod gitlab;