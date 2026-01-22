# Atalhos Docker Compose para este projeto
# Execute: source ./aliases.sh

alias dcu='docker compose up --build'        # sobe e builda
alias dcud='docker compose up -d --build'    # sobe em background
alias dcd='docker compose down'               # para e remove
alias dcr='docker compose restart'            # reinicia serviço
alias dcl='docker compose logs -f'            # ver logs em tempo real
alias dcb='docker compose up -d --build'      # rebuild + start detached
alias dce='docker compose exec'               # exec no container
alias dcude='dcud && dce'                     # sobe em background e entra no container
