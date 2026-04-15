# Core Battle Project Index

## Root Structure

```
Core Battle/
├── .ia/                    # Project documentation and index
├── client/                 # Desktop UI application
├── server/                 # Headless simulation engine
└── shared/                 # Common ECS components
```

## Directory Descriptions

### `.ia/`
| File | Description |
|------|-------------|
| `index.md` | This file - project structure index |
| `RULES.md` | Coding rules and conventions |
| `SPECIFICATIONS.md` | Technical specifications document |

### `client/`
| File/Dir | Description |
|----------|-------------|
| `Cargo.toml` | Client dependencies and package config |
| `tauri.conf.json` | Tauri desktop app configuration |
| `src/main.rs` | Client entry point |
| `src/lib.rs` | Library root with App component |
| `src/styles/` | CSS styles module |

### `server/`
| File/Dir | Description |
|----------|-------------|
| `src/` | Server source directory |

### `shared/`
| File/Dir | Description |
|----------|-------------|
| `src/` | Shared ECS components directory |
