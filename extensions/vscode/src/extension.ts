import * as vscode from 'vscode';
import { FerrisScriptCompletionProvider } from './completion/provider';

/**
 * Extension activation entry point
 * Called when the extension is activated (on .ferris file open)
 */
export function activate(context: vscode.ExtensionContext): void {
    console.log('FerrisScript extension is now active');

    // Register completion provider
    const completionProvider = new FerrisScriptCompletionProvider();
    const completionDisposable = vscode.languages.registerCompletionItemProvider(
        { scheme: 'file', language: 'ferrisscript' },
        completionProvider,
        ':', // Trigger on colon for type hints
        '.' // Trigger on dot for member access (future)
    );

    context.subscriptions.push(completionDisposable);
}

/**
 * Extension deactivation cleanup
 * Called when the extension is deactivated
 */
export function deactivate(): void {
    console.log('FerrisScript extension is now deactivated');
}
