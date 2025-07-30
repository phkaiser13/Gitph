# =============================================================================
# CMakeLists.txt (Diretório 'src/')
# -----------------------------------------------------------------------------
# Responsabilidade:
# 1. Orquestrar a compilação da aplicação principal 'gitph'.
# 2. Delegar a compilação das partes lógicas (core, modules) para seus
#    respectivos subdiretórios.
# 3. Definir o alvo executável final.
# 4. Lincar (vincular) todas as bibliotecas compiladas (core, modules) no
#    executável final.
# 5. Definir as regras de instalação para o executável.
#
# Filosofia de Código Limpo:
# Este arquivo atua como um "gerente de montagem". Ele não se preocupa com os
# detalhes de como cada peça é feita (isso é responsabilidade dos CMakeLists.txt
# nos subdiretórios). Ele apenas sabe quais peças são necessárias ('gitph_core',
# futuros módulos) e como montá-las para criar o produto final (o executável
# 'gitph').
# =============================================================================

# --- 1. Delegação para os Módulos de Código-Fonte ---

# Adiciona o diretório do núcleo da aplicação ('core/') ao processo de build.
# O CMake irá processar o 'CMakeLists.txt' dentro de 'core/', que será
# responsável por compilar todo o código C em uma biblioteca estática.
# Esta abordagem mantém a lógica de compilação do 'core' completamente
# encapsulada e isolada.
add_subdirectory(core)

# [MARCADOR PARA O FUTURO]
# Quando adicionarmos nosso módulo em Rust para a funcionalidade de clone,
# a linha para incluí-lo no build será adicionada aqui. Exemplo:
# add_subdirectory(modules/clone_handler)


# --- 2. Definição do Alvo Executável ---

# Cria o alvo executável final com o nome do projeto (definido na raiz como 'gitph').
# Note que não passamos nenhum arquivo-fonte diretamente aqui. Os arquivos-fonte
# são compilados em bibliotecas pelos submódulos. O executável é,
# essencialmente, um "invólucro" que será ligado às bibliotecas que contêm
# a lógica real.
add_executable(${PROJECT_NAME})


# --- 3. Vinculação (Linking) das Bibliotecas ---

# Vincula a biblioteca do núcleo ('gitph_core', que será criada por 'core/CMakeLists.txt')
# ao nosso executável principal.
# A palavra-chave 'PRIVATE' significa que 'gitph_core' é uma dependência de
# compilação para 'gitph', mas não é exposta como parte da interface pública
# do executável. Isso é uma excelente prática de encapsulamento.
target_link_libraries(${PROJECT_NAME} PRIVATE gitph_core)

# [MARCADOR PARA O FUTURO]
# Quando o módulo Rust for adicionado, sua biblioteca também será vinculada aqui.
# Exemplo:
# target_link_libraries(${PROJECT_NAME} PRIVATE clone_handler)


# --- 4. Regras de Instalação ---

# Define como o executável 'gitph' deve ser instalado no sistema do usuário.
# Quando o comando de instalação for executado (ex: 'cmake --install .'),
# o alvo (nosso executável compilado) será copiado para o diretório 'bin'
# dentro do prefixo de instalação padrão do sistema.
# (Ex: /usr/local/bin/gitph em Linux, C:/Program Files/gitph/bin/gitph em Windows).
install(TARGETS ${PROJECT_NAME} DESTINATION bin)