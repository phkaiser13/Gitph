#!/bin/sh
# ==============================================================================
# Script de Instalação do 'gitph' para Linux e macOS
#
# Este script realiza as seguintes tarefas:
# 1. Verifica se o Git está instalado e acessível no PATH.
# 2. Se o Git não for encontrado, instrui o usuário sobre como instalá-lo.
# 3. Adiciona o diretório do executável 'gitph' ao PATH do usuário,
#    modificando o arquivo de perfil do shell apropriado (.bashrc, .zshrc).
#
# Autor: Pedro H. Garcia (phkaiser13)
# Licença: GPL-3.0
# Uso: Execute com `sh install.sh` ou `./install.sh` (após `chmod +x install.sh`)
#      a partir do diretório raiz do projeto 'gitph'.
# ==============================================================================

# --- Configuração de Cores e Funções de Log ---
# Usamos códigos de escape ANSI para colorir a saída.
C_INFO='\033[0;36m'    # Ciano
C_SUCCESS='\033[0;32m' # Verde
C_WARN='\033[0;33m'    # Amarelo
C_ERROR='\033[0;31m'   # Vermelho
C_NC='\033[0m'         # Sem Cor

# Funções de log para padronizar as mensagens.
info() {
    printf "${C_INFO}%s${C_NC}\n" "$1"
}
success() {
    printf "${C_SUCCESS}%s${C_NC}\n" "$1"
}
warn() {
    printf "${C_WARN}%s${C_NC}\n" "$1"
}
error() {
    printf "${C_ERROR}%s${C_NC}\n" "$1"
}

# --- Funções Principais ---

# Função para verificar se o Git está disponível.
check_git_installation() {
    info "1. Verificando a instalação do Git..."
    # `command -v` é a maneira padrão e portável de verificar se um comando existe.
    if ! command -v git >/dev/null 2>&1; then
        error "ERRO: O Git não foi encontrado no seu PATH."
        warn "O 'gitph' depende do Git para funcionar."
        warn "Por favor, instale o Git usando o gerenciador de pacotes do seu sistema:"
        warn "  - Debian/Ubuntu: sudo apt-get install git"
        warn "  - Fedora:        sudo dnf install git"
        warn "  - macOS (Homebrew): brew install git"
        warn "Depois, execute este script novamente."
        exit 1
    else
        GIT_PATH=$(command -v git)
        success "   ✔ Git encontrado em: ${GIT_PATH}"
    fi
}

# Função para adicionar o diretório do 'gitph' ao PATH do usuário.
setup_path() {
    info "2. Configurando o PATH do ambiente..."

    # O diretório do executável 'gitph' é 'target/release'.
    # `readlink -f` ou `pwd` para obter o caminho absoluto.
    SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
    INSTALL_DIR="${SCRIPT_DIR}/../../target/release"

    if [ ! -f "${INSTALL_DIR}/gitph" ]; then
        error "ERRO: O executável 'gitph' não foi encontrado em '${INSTALL_DIR}'."
        warn "Por favor, compile o projeto em modo de release primeiro com: cargo build --release"
        exit 1
    fi

    info "   Diretório de instalação: ${INSTALL_DIR}"

    # --- Lógica de Atualização do PATH ---
    # Esta é a parte mais complexa, pois depende do shell do usuário.
    # Detectamos o shell e escolhemos o arquivo de perfil apropriado.
    SHELL_PROFILE=""
    if [ -n "$BASH_VERSION" ]; then
        SHELL_PROFILE="$HOME/.bashrc"
    elif [ -n "$ZSH_VERSION" ]; then
        SHELL_PROFILE="$HOME/.zshrc"
    else
        # Um fallback razoável para outros shells POSIX.
        SHELL_PROFILE="$HOME/.profile"
    fi

    info "   Detectado arquivo de perfil do shell: ${SHELL_PROFILE}"

    # A linha a ser adicionada ao arquivo de perfil.
    EXPORT_LINE="export PATH=\"${INSTALL_DIR}:\$PATH\""

    # Verificamos se a linha já existe para evitar duplicação.
    if grep -q "export PATH=\"${INSTALL_DIR}:\$PATH\"" "$SHELL_PROFILE" 2>/dev/null; then
        success "   ✔ O diretório do 'gitph' já está configurado em seu arquivo de perfil."
    else
        info "   Adicionando linha de exportação ao '${SHELL_PROFILE}'..."
        # Adicionamos a linha ao final do arquivo.
        printf "\n# Adicionado pelo instalador do gitph\n%s\n" "${EXPORT_LINE}" >> "$SHELL_PROFILE"
        success "   ✔ Arquivo de perfil atualizado com sucesso."
        warn "   AVISO: Você precisará reiniciar seu shell ou executar 'source ${SHELL_PROFILE}' para que as alterações entrem em vigor."
    fi
}

# --- Execução Principal ---
printf "${C_NC}Iniciando a instalação do gitph...\n"
printf "------------------------------------\n"

check_git_installation
setup_path

printf "------------------------------------\n"
success "Instalação do gitph concluída!"