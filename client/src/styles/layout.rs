pub const LAYOUT_CSS: &str = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: system-ui, -apple-system, sans-serif;
    overflow: hidden;
}

.app-container {
    display: flex;
    flex-direction: row;
    width: 100vw;
    height: 100vh;
    background: #f5f5f5;
}

.left-panel {
    display: flex;
    flex-direction: column;
    background: #ececec;
    border-right: 4px solid #d0d0d0;
    transition: width 0.3s ease;
    position: relative;
    overflow: visible;
}

.left-top {
    flex: 0 0 auto;
    min-height: 100px;
    padding: 16px;
    border-bottom: 4px solid #d0d0d0;
    color: #1a1a2e;
}

.left-bottom {
    flex: 1;
    padding: 16px;
    color: #1a1a2e;
    overflow-y: auto;
}

.toggle-btn {
    position: absolute;
    right: -20px;
    top: 50%;
    transform: translateY(-50%);
    width: 20px;
    height: 48px;
    background: #d0d0d0;
    border: none;
    border-radius: 0 4px 4px 0;
    color: #1a1a2e;
    cursor: pointer;
    z-index: 10;
    opacity: 0;
    transition: opacity 0.2s ease, background 0.2s ease;
}

.left-panel:hover .toggle-btn {
    opacity: 1;
}

.toggle-btn:hover {
    background: #c0c0c0;
}

.right-panel {
    flex: 1;
    background: #ffffff;
    color: #1a1a2e;
    padding: 24px;
    overflow-y: auto;
}
"#;
