import * as vscode from 'vscode';
import { FerrisScriptCompletionProvider } from './completion/provider';
import { FerrisScriptHoverProvider } from './hover/provider';
import { FerrisScriptDiagnosticProvider } from './diagnostics/provider';

let diagnosticProvider: FerrisScriptDiagnosticProvider | undefined;

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

    // Register hover provider (Phase 5)
    const hoverProvider = new FerrisScriptHoverProvider();
    const hoverDisposable = vscode.languages.registerHoverProvider(
        { scheme: 'file', language: 'ferrisscript' },
        hoverProvider
    );

    context.subscriptions.push(hoverDisposable);

    // Register diagnostic provider (Phase 5)
    diagnosticProvider = new FerrisScriptDiagnosticProvider();
    context.subscriptions.push(diagnosticProvider);

    // Update diagnostics on file save
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument((document) => {
            if (diagnosticProvider) {
                diagnosticProvider.updateDiagnostics(document);
            }
        })
    );

    // Update diagnostics on file open
    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument((document) => {
            if (diagnosticProvider) {
                diagnosticProvider.updateDiagnostics(document);
            }
        })
    );

    // Update diagnostics for currently active editor
    const activeEditor = vscode.window.activeTextEditor;
    if (activeEditor && diagnosticProvider) {
        diagnosticProvider.updateDiagnostics(activeEditor.document);
    }
}

/**
 * Extension deactivation cleanup
 * Called when the extension is deactivated
 */
export function deactivate(): void {
    if (diagnosticProvider) {
        diagnosticProvider.dispose();
        diagnosticProvider = undefined;
    }
    console.log('FerrisScript extension is now deactivated');
}
