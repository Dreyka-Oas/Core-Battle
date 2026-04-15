# Project Rules

## File Structure
- Maximum 5 files per directory
- Maximum 100 lines per file
- Single responsibility per file
- English naming only (files, folders, variables, functions)

## Code Style
- No comments in code
- No French variables or identifiers
- All names in English
- Clear, descriptive naming

## Architecture
- ECS pattern for game logic
- Clean separation of concerns
- Components do one thing

## Folders
- `src/` for source code
- `components/` for UI components
- `styles/` for CSS
- `systems/` for ECS systems
- `components/` for ECS components
- `.ia/` at root required for project documentation

## Index File (`.ia/index.md`)
- List complete project tree structure
- Describe each file/directory in 5 words max
- One table per major directory
- Keep descriptions concise and in English

## UI Theme
- Light professional theme with off-white backgrounds
- Primary background: `#f5f5f5`
- Left panel: `#ececec`
- Right panel: `#ffffff`
- Text: `#1a1a2e`
- Border thickness: 4px solid for all separators

## UI Behavior
- Toggle button visible only on left-panel hover
- Collapse reduces width (200px → 0px), does not minimize window
- Same border thickness for left-right separator and top-bottom separator
