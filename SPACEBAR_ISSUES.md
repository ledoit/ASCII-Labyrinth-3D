# Possible Spacebar Issues - Analysis

## Removed Previous Fix
- Removed `keyup` event handler
- Removed `isProcessingSpacebar` flag
- Removed 300ms delay
- Simplified to basic `keydown` handler

## Possible Root Causes

### 1. **Stale gameState Variable (Most Likely)**
   - **Issue**: The `gameState` variable is updated asynchronously in `gameLoop()` via `invoke()`
   - **Problem**: When spacebar is pressed, `gameState` might contain an old value where `has_won` is still `false`
   - **Why**: Race condition between gameLoop updates and keydown event
   - **Solution**: Read gameState directly from the most recent render/update, not the cached variable

### 2. **Focus Issues**
   - **Issue**: Viewport element might not have focus when spacebar is pressed
   - **Problem**: Window-level keydown might not fire if focus is elsewhere
   - **Why**: Tauri window or another element might have stolen focus
   - **Solution**: Ensure viewport has focus, or listen on document instead of window

### 3. **Event Not Reaching Handler**
   - **Issue**: Spacebar event might be captured/stopped by another handler
   - **Problem**: Another event listener might be calling `stopPropagation()` or `preventDefault()`
   - **Why**: Multiple listeners on the same event
   - **Solution**: Check for other spacebar handlers, use capture phase, or check event propagation

### 4. **has_won Flag Timing**
   - **Issue**: `has_won` is set in Rust, but JavaScript `gameState` hasn't been updated yet
   - **Problem**: The flag is set in `update_game()` but `gameState` variable update happens after render
   - **Why**: Async chain: update_game → render_frame → update gameState variable
   - **Solution**: Check `has_won` from the returned state immediately after update, not from cached variable

### 5. **JSON Parsing Failure**
   - **Issue**: `gameState` might be null, undefined, or invalid JSON
   - **Problem**: Silent failure in try/catch returns early
   - **Why**: If gameState is corrupted or not initialized
   - **Solution**: Add logging to see if parsing is failing

### 6. **Tauri Window Focus (Windows-specific)**
   - **Issue**: Windows Tauri windows might not receive keyboard events when not focused
   - **Problem**: Window might appear focused but not actually be receiving events
   - **Why**: Windows focus handling differences
   - **Solution**: Use Tauri's window focus events or check window focus state

### 7. **Event Target Mismatch**
   - **Issue**: Spacebar event might be firing on a different element
   - **Problem**: If viewport or another element captures the event
   - **Why**: Event bubbling/capturing might route it elsewhere
   - **Solution**: Check `e.target` and ensure it's the expected element

### 8. **has_won Flag Reset Too Quickly**
   - **Issue**: `has_won` might be set but immediately reset before spacebar is pressed
   - **Problem**: If `next_level()` is called automatically or `has_won` is cleared
   - **Why**: Race condition in state management
   - **Solution**: Check if `has_won` persists in the gameState

### 9. **Async Invoke Delay**
   - **Issue**: `invoke('next_level')` might be slow or failing silently
   - **Problem**: The call might not complete, or error might be swallowed
   - **Why**: Network/IPC delay or error handling
   - **Solution**: Add error logging and check if invoke completes

### 10. **Multiple Rapid Presses**
   - **Issue**: Multiple spacebar presses might interfere with each other
   - **Problem**: If first press triggers async operation, second press might read stale state
   - **Why**: No debouncing or state locking
   - **Solution**: Add a simple flag to prevent multiple simultaneous calls

## Recommended Debugging Steps

1. **Add console logging**:
   ```javascript
   console.log('Spacebar pressed, gameState:', gameState);
   console.log('has_won:', gameStateObj?.has_won);
   console.log('Focus element:', document.activeElement);
   ```

2. **Check gameState freshness**: Read from the most recent update, not cached variable

3. **Verify event firing**: Add log at the very start of keydown handler

4. **Check focus**: Ensure viewport has focus when spacebar is pressed

5. **Test with direct state check**: Temporarily hardcode `has_won: true` to see if handler fires
