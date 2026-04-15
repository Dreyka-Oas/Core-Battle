Voici le fichier **`SPECIFICATIONS_TECHNIQUES.md`** complet, consolidé et prêt à l'emploi. Il intègre la comparaison technologique, les choix architecturaux (ECS vs MVC) et la structure détaillée du projet.

Vous pouvez copier ce contenu directement dans un fichier à la racine de votre projet.

```markdown
# 🚀 Spécifications Techniques : Laboratoire de Simulation IA

> **Document de Référence** | Architecture Haute Performance pour Simulation de Combat 100% IA.
> Stack : Rust + Tauri 2 + WGPU + ECS.

---

## 1. 📊 Comparatif Technologique : Old vs Modern

Pour atteindre les objectifs de performance (simulation de milliers d'entités en temps réel), nous abandonnons les approches "web classiques" au profit de technologies "système modernes".

| Composant | Ancien Standard (Old) | 🚀 Choix Moderne (New) | Justification de la performance |
| :--- | :--- | :--- | :--- |
| **Architecture** | MVC (Objets) | **ECS (Data-Oriented)** | Alignement mémoire optimal. Traite 10 000+ entités en parallèle sans ralentissement. |
| **Sérialisation** | Serde + JSON/MessagePack | **rkyv (Zero-Copy)** | Les données sont lues directement depuis le disque/réseau sans décodage (latence quasi-nulle). |
| **Base de Données** | SQLite (via Rusqlite) | **Redb** ou **Limbo** | 100% Rust, asynchrone natif, zéro dépendance C++, écriture disque ultra-rapide. |
| **Rendu Graphique** | WebGL / Canvas | **WebGPU (WGPU)** | Accès direct au GPU pour le rendu ET le calcul (Compute Shaders pour l'IA). |
| **Interface UI** | HTML / CSS / DOM | **Slint** ou **Dioxus** | Rendu natif GPU. Pas de lourdeur du DOM navigateur, fluidité 60 FPS garantie. |
| **Réseau** | HTTP / TCP | **QUIC (Quinn)** | Protocole de nouvelle génération (HTTP/3). Rapide comme l'UDP, fiable comme le TCP. |
| **Parallélisme** | Threads Manuels | **Rayon** | Parallélisme de données automatique et sûr. |

---

## 2. 🏛️ Architecture Logicielle : Le Choix Stratégique

### Pourquoi pas de MVC ?
Le pattern **MVC (Modèle-Vue-Contrôleur)** est excellent pour les applications de gestion (CRUD), mais inadapté aux simulations temps réel.
*   **Problème MVC :** Les objets sont dispersés en mémoire (pointeurs). Le processeur perd du temps à "sauter" d'un objet à l'autre (Cache Miss).
*   **Solution ECS :** Nous utilisons le pattern **Entity-Component-System**.

### L'Architecture ECS (Entity-Component-System)
C'est le standard de l'industrie du jeu vidéo (utilisé par Unity DOTS, Unreal, Bevy).

1.  **Entity (Entité) :** Un simple identifiant numérique (ID). Ex: `Entité #42`.
2.  **Component (Composant) :** Données brutes stockées en tableaux continus.
    *   *Exemple :* Un tableau `Position` contient les coordonnées de *tous* les combattants côte à côte en RAM.
3.  **System (Système) :** Logique qui traite les tableaux.
    *   *Exemple :* Le système de physique lit le tableau `Position` et le met à jour en parallèle sur tous les cœurs du CPU.

**Résultat :** Le CPU accède à la mémoire de manière séquentielle et prévisible, multipliant la vitesse d'exécution par 10 à 100 par rapport à une approche Objet classique.

---

## 3. 📦 La Stack Technique Détaillée

Voici les "briques" précises à utiliser pour le développement.

### Backend (Cœur du Laboratoire)
*   **Langage :** Rust (Edition 2021+).
*   **Runtime Asynchrone :** `Tokio` (standard industriel).
*   **Moteur ECS :** `hecs` (léger et rapide) ou `bevy_ecs` (plus de fonctionnalités).
*   **Sérialisation :** `rkyv` pour la sauvegarde instantanée et le réseau.
*   **Calcul Parallèle :** `rayon` pour diviser le travail sur tous les cœurs CPU.
*   **Base de données :** `redb` (ACID, pure Rust) pour stocker les ADN des IA vainqueurs.

### Graphisme & Calcul (GPU)
*   **API Graphique :** `wgpu` (Vulkan, Metal, DirectX 12).
*   **Calcul IA :** Compute Shaders (WGSL). Les décisions des IA sont calculées par milliers simultanément sur la carte graphique.

### Frontend (Interface Desktop)
*   **Framework :** `Tauri 2`. (Léger, sécurité intégrée).
*   **UI Interface :** `Slint` (recommandé pour le style natif) ou `Dioxus` (si vous préférez une syntaxe type React).
*   **Communication :** IPC (Inter-Process Communication) optimisé par binaires.

---

## 4. 📂 Structure des Dossiers (Cargo Workspace)

Nous divisons le projet en 3 "Crates" (modules Rust indépendants) pour compiler uniquement ce qui est nécessaire.

```text
mon_laboratoire_ia/
├── Cargo.toml              # Configuration du Workspace Maître
├── 📁 shared/               # CRATE 1 : Données Communes
│   ├── Cargo.toml
│   └── src/
│       ├── components.rs   # Définition des composants ECS (Position, PV, ADN)
│       ├── packets.rs      # Structures de données pour le réseau (rkyv)
│       └── lib.rs
├── 📁 server/               # CRATE 2 : Moteur de Simulation (Headless)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs         # Point d'entrée du simulateur
│       ├── simulation/     
│       │   ├── systems/    # Logique CPU (Physique, Collisions)
│       │   └── ai_kernels/ # Shaders WGSL pour l'IA (exécutés sur GPU)
│       ├── persistence/    # Sauvegarde Redb
│       └── network/        # Serveur QUIC (Quinn)
└── 📁 client/               # CRATE 3 : Observatoire (Tauri)
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs         # Lancement Tauri
    │   ├── rendering/      # Rendu WebGPU (caméra, particules)
    │   └── ui_logic/       # Gestion des boutons/stats (Slint/Dioxus)
    └── ui/                 # Fichiers d'interface (Slint ou HTML)
```

---

## 5. 🔄 Flux de Données & Cycle de Vie

Voici comment les données circulent une fois l'application lancée :

1.  **Initialisation :**
    *   Le `server` charge les meilleures IA depuis `redb` (disque).
    *   Le `server` initialise le monde ECS en RAM.

2.  **Boucle de Simulation (Tick) :**
    *   **Phase CPU (Rayon) :** Les systèmes ECS calculent la physique et les déplacements en parallèle.
    *   **Phase GPU (WGPU) :** Les états des IA sont envoyés à la carte graphique. Le *Compute Shader* calcule les décisions (attaquer, fuir) en quelques millisecondes pour toutes les entités.
    *   **Mise à jour :** Les décisions reviennent dans l'ECS.

3.  **Transmission au Client (Observateur) :**
    *   Toutes les 16ms (60 FPS), le `server` capture un instantané de l'état du monde.
    *   `rkyv` sérialise cet instantané en binaire brut (Zero-Copy).
    *   L'IPC Tauri ou le réseau QUIC envoie le paquet au `client`.

4.  **Affichage (Client) :**
    *   Le `client` reçoit le paquet binaire.
    *   WebGPU dessine les entités à l'écran sans créer d'objets JavaScript intermédiaires.

---

## 6. ⚡ Optimisations Clés

*   **Déterminisme :** Le moteur est 100% déterministe. Si vous redémarrez avec la même "graine" (seed), le combat sera identique. Cela permet de "rejouer" des combats pour analyser les bugs.
*   **Mode Turbo :** Le simulateur peut tourner à 1000 ticks/seconde (vitesse accélérée) pour entraîner les IA rapidement, tandis que le client n'affiche que 60 images/seconde (vitesse temps réel).
*   **Mémoire :** Utilisation de `parking_lot` pour les verrous (mutex) plus rapides que les verrous standards de Rust.

---

## 7. 🛠️ Commandes de Démarrage

Pour initialiser le projet avec cette architecture :

```bash
# Créer le workspace
mkdir mon_laboratoire_ia && cd mon_laboratoire_ia

# Créer les dossiers
cargo new shared --lib
cargo new server
cargo new client

# Configurer le Cargo.toml racine pour lier les crates
echo '[workspace]
members = ["shared", "server", "client"]' > Cargo.toml
```

---
*Document prêt pour le développement. Version 1.0.*
```