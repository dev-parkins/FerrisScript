import * as vscode from 'vscode';
/**
 * Completion provider for FerrisScript
 * Provides context-aware code completion for keywords, types, and functions
 */
export declare class FerrisScriptCompletionProvider implements vscode.CompletionItemProvider {
    /**
     * Provide completion items based on cursor position and context
     * @param document The text document
     * @param position The cursor position
     * @param token Cancellation token
     * @param context Completion context from VS Code
     * @returns Array of completion items or completion list
     */
    provideCompletionItems(document: vscode.TextDocument, position: vscode.Position, token: vscode.CancellationToken, context: vscode.CompletionContext): vscode.ProviderResult<vscode.CompletionItem[] | vscode.CompletionList>;
}
