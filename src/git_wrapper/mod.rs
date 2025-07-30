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
// ==============================================================================

/// Módulo contendo a lógica para o comando `git status`.
pub mod status;

/// Módulo contendo a lógica para os comandos `git add` e `git commit`.
pub mod commit;

/// Módulo contendo a lógica para o comando `git push`.
pub mod push;

/// Módulo contendo a lógica para criar e sincronizar tags Git.
pub mod tag;

/// Módulo para obter informações sobre repositórios remotos.
pub mod remote;

/// Módulo para criar, listar e mudar de branches.
pub mod branch;