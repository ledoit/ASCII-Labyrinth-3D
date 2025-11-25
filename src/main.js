import { invoke } from '@tauri-apps/api/core';

let gameState = null;
let keys = {
    w: false,
    s: false,
    a: false,
    d: false,
    q: false,
    e: false,
};

const viewport = document.getElementById('viewport');
let viewportWidth = 120;
let viewportHeight = 40;

// Initialize game
async function init() {
    try {
        const stateJson = await invoke('init_game');
        gameState = stateJson;
        resizeViewport();
        gameLoop();
    } catch (error) {
        console.error('Failed to initialize game:', error);
    }
}

function resizeViewport() {
    const container = document.getElementById('app');
    const availableWidth = container.clientWidth - 40;
    const availableHeight = container.clientHeight - 100;
    
    // Calculate optimal viewport size based on available space
    // Using a monospace font, we can estimate character size
    const charWidth = 8; // Approximate pixel width of a character
    const charHeight = 16; // Approximate pixel height of a character
    
    viewportWidth = Math.floor(availableWidth / charWidth);
    viewportHeight = Math.floor(availableHeight / charHeight);
    
    // Ensure minimum size
    viewportWidth = Math.max(80, Math.min(viewportWidth, 200));
    viewportHeight = Math.max(30, Math.min(viewportHeight, 80));
    
    viewport.style.width = `${viewportWidth * charWidth}px`;
    viewport.style.height = `${viewportHeight * charHeight}px`;
    viewport.style.fontSize = '12px';
    viewport.style.lineHeight = '16px';
}

async function gameLoop() {
    if (!gameState) return;
    
    // Get input
    const input = {
        forward: keys.w,
        backward: keys.s,
        left: keys.a,
        right: keys.d,
        turn_left: keys.q,
        turn_right: keys.e,
    };
    
    // Update game state
    try {
        gameState = await invoke('update_game', {
            stateJson: gameState,
            input: input,
        });
        
        // Render frame
        const frame = await invoke('render_frame', {
            stateJson: gameState,
            width: viewportWidth,
            height: viewportHeight,
        });
        
        // Display frame
        displayFrame(frame);
    } catch (error) {
        console.error('Game loop error:', error);
    }
    
    requestAnimationFrame(gameLoop);
}

function displayFrame(frame) {
    // Split frame into lines and render
    const lines = frame.split('\n');
    let html = '';
    
    for (let i = 0; i < lines.length && i < viewportHeight; i++) {
        const line = lines[i] || '';
        // Escape HTML and preserve spaces
        const escaped = line
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;')
            .replace(/ /g, '&nbsp;');
        html += escaped + '<br>';
    }
    
    viewport.innerHTML = html;
}

// Keyboard event handlers
document.addEventListener('keydown', (e) => {
    switch (e.key.toLowerCase()) {
        case 'w':
            keys.w = true;
            e.preventDefault();
            break;
        case 's':
            keys.s = true;
            e.preventDefault();
            break;
        case 'a':
            keys.a = true;
            e.preventDefault();
            break;
        case 'd':
            keys.d = true;
            e.preventDefault();
            break;
        case 'q':
            keys.q = true;
            e.preventDefault();
            break;
        case 'e':
            keys.e = true;
            e.preventDefault();
            break;
        case 'escape':
            window.close();
            break;
    }
});

document.addEventListener('keyup', (e) => {
    switch (e.key.toLowerCase()) {
        case 'w':
            keys.w = false;
            break;
        case 's':
            keys.s = false;
            break;
        case 'a':
            keys.a = false;
            break;
        case 'd':
            keys.d = false;
            break;
        case 'q':
            keys.q = false;
            break;
        case 'e':
            keys.e = false;
            break;
    }
});

// Handle window resize
window.addEventListener('resize', () => {
    resizeViewport();
});

// Start the game
init();

