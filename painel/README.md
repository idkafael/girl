# Girl · Painel

Lançador desktop da Girl Agência: abre um navegador isolado (perfil próprio) por criadora, separado do Chrome pessoal. Feito em Rust, sem instalação.

## Baixar
Pegue o `GirlPainel.zip` mais recente em **[Releases](../../releases/latest)**, extraia e rode o `GirlPainel.exe`.

## Como funciona
- Um botão por criadora; o clique abre o Chrome (ou Edge) com `--user-data-dir` + `--profile-directory` só dela.
- A lista fica em `criadoras.json`, ao lado do `.exe`.
- Sem proxy e sem fingerprint — apenas perfis nativos do navegador.
