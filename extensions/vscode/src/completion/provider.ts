import * as vscode from 'vscode';
import { getKeywordCompletions } from './keywords';
import { getTypeCompletions } from './types';
import { getFunctionCompletions } from './functions';
import { detectContext, CompletionContext } from '../utils/context';

/**
 * Completion provider for FerrisScript
 * Provides context-aware code completion for keywords, types, and functions
 */
export class FerrisScriptCompletionProvider implements vscode.CompletionItemProvider {
    /**
     * Provide completion items based on cursor position and context
     * @param document The text document
     * @param position The cursor position
     * @param token Cancellation token
     * @param context Completion context from VS Code
     * @returns Array of completion items or completion list
     */
    provideCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken,
        context: vscode.CompletionContext
    ): vscode.ProviderResult<vscode.CompletionItem[] | vscode.CompletionList> {
        // Detect completion context based on cursor position
        const ctx = detectContext(document, position);

        // Return appropriate completions based on context
        switch (ctx) {
            case CompletionContext.TypePosition: {
                // After ':' in type annotations - show only types
                return getTypeCompletions();
            }

            case CompletionContext.StatementStart: {
                // At statement start - show statement-level keywords only
                return getKeywordCompletions(true);
            }

            case CompletionContext.Expression: {
                // In expression context - show expression keywords and functions
                // Filter out statement-only keywords (fn, let, while, return)
                const statementOnlyKeywords = ['fn', 'let', 'while', 'return'];
                const allKeywords = getKeywordCompletions(false);
                const expressionKeywords = allKeywords.filter(item => {
                    const label = typeof item.label === 'string' ? item.label : item.label.label;
                    return !statementOnlyKeywords.includes(label);
                });
                return [
                    ...expressionKeywords,
                    ...getFunctionCompletions()
                ];
            }

            default: {
                // Unknown context - show everything
                return [
                    ...getKeywordCompletions(false),
                    ...getTypeCompletions(),
                    ...getFunctionCompletions()
                ];
            }
        }
    }
}
