import * as vscode from 'vscode';
import { getKeywordHover } from './keywords';
import { getTypeHover } from './types';
import { getFunctionHover } from './functions';

/**
 * FerrisScript Hover Provider
 * 
 * Provides hover tooltips for keywords, types, and built-in functions.
 * Shows description, syntax, and examples for each language element.
 */
export class FerrisScriptHoverProvider implements vscode.HoverProvider {
    provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken
    ): vscode.ProviderResult<vscode.Hover> {
        // Get the word at the current position
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) {
            return undefined;
        }

        const word = document.getText(wordRange);

        // Try to find hover content for this word
        let hoverContent: vscode.MarkdownString | undefined;

        // Check keywords first (let, fn, if, etc.)
        hoverContent = getKeywordHover(word);
        if (hoverContent) {
            return new vscode.Hover(hoverContent, wordRange);
        }

        // Check types (i32, Vector2, etc.)
        hoverContent = getTypeHover(word);
        if (hoverContent) {
            return new vscode.Hover(hoverContent, wordRange);
        }

        // Check built-in functions (print, etc.)
        hoverContent = getFunctionHover(word);
        if (hoverContent) {
            return new vscode.Hover(hoverContent, wordRange);
        }

        // No hover content found
        return undefined;
    }
}
