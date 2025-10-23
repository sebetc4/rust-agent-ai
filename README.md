# 🤖 agents-rs

> Un environnement local et interopérable pour exécuter des **agents LLM** entièrement **offline**

[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app/)
[![React](https://img.shields.io/badge/React-18-61DAFB.svg)](https://reactjs.org/)
[![MCP](https://img.shields.io/badge/MCP-2024--11--05-green.svg)](https://modelcontextprotocol.io)

## 🎯 Objectif

Créer une plateforme d'agents LLM locale permettant :
- 🔒 **Confidentialité totale** : Tout s'exécute localement
- 🔌 **Interopérabilité** : Compatible avec le protocole MCP (Claude, LM Studio)
- 🚀 **Performance** : Backend Rust natif
- 🧰 **Extensible** : Système de plugins pour ajouter des outils

## ✨ Fonctionnalités actuelles

### ✅ Backend Rust complet
- Moteur LLM modulaire (prêt pour llama.cpp)
- Gestionnaire de contexte conversationnel
- Serveur MCP JSON-RPC sur HTTP
- Système de plugins/outils dynamique
- 9 commandes Tauri exposées au frontend

### 🔧 Outils intégrés
- `echo` : Test de communication
- `file_reader` : Lecture de fichiers locaux
- `file_writer` : Écriture de fichiers

## 🏗️ Architecture

```
┌───────────────────────────┐
│   Tauri Frontend (React)  │
│   Interface utilisateur   │
└────────────┬──────────────┘
             │ IPC Commands
┌────────────▼──────────────┐
│    Backend Rust (Core)    │
│ ├─ Moteur LLM             │
│ ├─ Serveur MCP (port 3000)│
│ ├─ Gestionnaire contexte  │
│ └─ Registre d'outils      │
└────────────┬──────────────┘
             │
┌────────────▼──────────────┐
│   Plugins MCP externes    │
│  (fichiers, API, scripts) │
└───────────────────────────┘
```

Voir [BACKEND_ARCHITECTURE.md](./BACKEND_ARCHITECTURE.md) pour plus de détails.

## 🚀 Démarrage rapide

### Prérequis
- **Rust** 1.70+ ([installer](https://rustup.rs/))
- **Node.js** 18+ et **pnpm** ([installer pnpm](https://pnpm.io/installation))
- **Tauri dependencies** ([voir la doc](https://tauri.app/start/prerequisites/))
- **CMake** et **Clang** (pour llama.cpp):
  ```bash
  # Fedora/RHEL
  sudo dnf install cmake clang
  
  # Ubuntu/Debian
  sudo apt install cmake libclang-dev
  
  # Arch
  sudo pacman -S cmake clang
  ```

### Installation

```bash
# Cloner le dépôt
git clone https://github.com/votre-repo/agents-rs.git
cd agents-rs

# Installer les dépendances frontend
pnpm install

# Compiler le backend Rust
cargo build --manifest-path src-tauri/Cargo.toml

# Télécharger un modèle (exemple: Qwen3-1.7B)
mkdir -p models
# Placer votre fichier .gguf dans models/

# Tester le modèle
./test-quick.sh
```

## 🧪 Tests Rapides

```bash
# Test simple (1 prompt)
cargo run --manifest-path src-tauri/Cargo.toml --example simple

# Test complet (3 prompts + stats)
./test-quick.sh

# Mode interactif
./test-interactive.sh

# Tests unitaires
cargo test --manifest-path src-tauri/Cargo.toml --lib llm::tests
```

Voir [TEST-GUIDE.md](./TEST-GUIDE.md) pour plus de détails.

## 📦 Structure du projet

```
agents-rs/
├── src/                      # Frontend React
├── src-tauri/
│   └── src/
│       ├── backend/          # ⭐ Architecture backend
│       │   ├── llm_engine.rs
│       │   ├── context.rs
│       │   ├── mcp_server.rs
│       │   └── tools/
│       ├── lib.rs            # Commandes Tauri
│       └── main.rs
├── models/                   # Modèles GGUF à placer ici
├── tools/                    # Plugins MCP personnalisés
└── config.yaml               # Configuration
```

## 🧠 Utilisation

### Via le frontend (TypeScript)

```typescript
import { invoke } from '@tauri-apps/api/core';

// Créer une session
const sessionId = await invoke('create_session', { 
  title: 'Ma conversation' 
});

// Ajouter un message utilisateur
await invoke('add_message', { 
  sessionId, 
  role: 'user', 
  content: 'Bonjour !' 
});

// Générer une réponse (quand LLM sera intégré)
const response = await invoke('generate_response', { 
  prompt: 'Bonjour !' 
});

// Exécuter un outil
const result = await invoke('execute_tool', {
  toolName: 'file_reader',
  arguments: { path: './README.md' }
});
```

### Via HTTP (MCP)

```bash
# Health check
curl http://localhost:3000/

# Lister les outils
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/list",
    "id": 1
  }'

# Appeler un outil
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
      "name": "echo",
      "arguments": {"text": "Hello!"}
    },
    "id": 2
  }'
```

## 🔮 Roadmap

| Étape | Description | État |
|-------|-------------|------|
| 🧩 1 | Architecture backend Rust | ✅ **Terminé** |
| 🧠 2 | Serveur MCP minimal | ✅ **Terminé** |
| 🧰 3 | Système d'outils/plugins | ✅ **Terminé** |
| 🦙 4 | Intégration llama.cpp | 🚧 En cours |
| 💬 5 | Interface chat React | ⏳ À faire |
| 🔗 6 | Compatibilité Claude/LM Studio | 🔜 Futur |
| 📦 7 | Release bêta | 🔜 Futur |

## 🛠️ Développement

### Tests
```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

### Build de production
```bash
pnpm tauri build
```

### Ajouter un outil MCP

Voir [tools/README.md](./tools/README.md) pour créer vos propres outils.

## 📖 Documentation

- [Architecture Backend](./BACKEND_ARCHITECTURE.md)
- [Instructions Copilot](./.github/copilot-instructions.md)
- [Model Context Protocol](https://modelcontextprotocol.io)

## 🤝 Contribution

Les contributions sont les bienvenues ! Voir [CONTRIBUTING.md](./CONTRIBUTING.md) (à créer).
