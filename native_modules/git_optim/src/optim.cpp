/**
 * Copyright © Pedro H. Garcia (phkaiser13)
 * SPDX-License-Identifier: GPL-3.0
 * This file is licensed under the GNU General Public License v3.0.
 */

// ==============================================================================
// Implementação do Módulo Nativo 'git_optim'
//
// Este arquivo contém a lógica de negócio para as funções declaradas em `optim.h`.
// ==============================================================================

#include "optim.h" // Incluímos nosso próprio cabeçalho para validação pelo compilador.

#include <iostream> // Para `std::cout` e `std::endl`.
#include <cstring>  // Para `strlen`.

void hello_from_cpp() {
    // `std::endl` não apenas adiciona uma nova linha, mas também "flusha" o buffer
    // de saída, garantindo que a mensagem apareça imediatamente no console.
    std::cout << "[C++] Olá do mundo C++! A ligação FFI está funcionando." << std::endl;
}

int32_t perform_complex_calculation(int32_t input) {
    // Uma operação simples para provar que a manipulação de dados funciona.
    return (input * 2) + 10;
}

int32_t get_string_length_from_cpp(const char* text) {
    // A verificação de ponteiro nulo é a verificação de segurança mais
    // importante em uma fronteira FFI. Nunca confie em ponteiros de código externo.
    if (text == nullptr) {
        return -1; // Retorna um código de erro, conforme definido no contrato da API.
    }

    // `strlen` é a função padrão e otimizada para esta tarefa.
    size_t length = strlen(text);

    // Retornamos o comprimento como um `int32_t` para corresponder à nossa API
    // e ao `i32` do Rust, fazendo um cast explícito para deixar a conversão clara.
    return static_cast<int32_t>(length);
}