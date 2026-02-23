### before compilation, you will need:
- linux bash (wsl on windows, nothing else on macOS)
- npm, nvm, node
- mkcert
- rust, cargo

### steps to compile app:
- run scripts/generate_cert.sh
- run scripts/run_database.sh in a separate console
- run scripts/build.sh in a separate console
- open https://localhost:8080