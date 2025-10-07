import * as vscode from 'vscode';
/**
 * Completion context types for FerrisScript
 */
export declare enum CompletionContext {
    /** After ':' in type annotations (let x: |, fn foo() -> |) */
    TypePosition = 0,
    /** At statement start (beginning of line after whitespace) */
    StatementStart = 1,
    /** Inside expression context (most places) */
    Expression = 2,
    /** Unable to determine context */
    Unknown = 3
}
/**
 * Detect completion context based on cursor position and surrounding text
 * @param document The text document
 * @param position The cursor position
 * @returns The detected completion context
 */
export declare function detectContext(document: vscode.TextDocument, position: vscode.Position): CompletionContext;
