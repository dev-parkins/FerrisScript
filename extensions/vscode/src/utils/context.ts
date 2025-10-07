import * as vscode from 'vscode';

/**
 * Completion context types for FerrisScript
 */
export enum CompletionContext {
    /** After ':' in type annotations (let x: |, fn foo() -> |) */
    TypePosition,
    /** At statement start (beginning of line after whitespace) */
    StatementStart,
    /** Inside expression context (most places) */
    Expression,
    /** Unable to determine context */
    Unknown
}

/**
 * Detect completion context based on cursor position and surrounding text
 * @param document The text document
 * @param position The cursor position
 * @returns The detected completion context
 */
export function detectContext(
    document: vscode.TextDocument,
    position: vscode.Position
): CompletionContext {
    const line = document.lineAt(position.line).text;
    const beforeCursor = line.substring(0, position.character);

    // Type position: "let x: |" or "fn foo(param: |" or "fn bar() -> |"
    // Match colon followed by optional whitespace and word characters (for partial types)
    // This handles: "let x: " and "let x: V" and "let x: Vec" etc.
    if (/:\s*\w*$/.test(beforeCursor)) {
        return CompletionContext.TypePosition;
    }

    // Statement start: "    |" (only whitespace before cursor)
    // This catches the start of statements where we want keywords like let, fn, if, while
    if (/^\s*$/.test(beforeCursor)) {
        return CompletionContext.StatementStart;
    }

    // Default to expression context
    // This includes most other positions: inside function bodies, after operators, etc.
    return CompletionContext.Expression;
}
