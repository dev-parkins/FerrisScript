import * as vscode from 'vscode';
import { FUNCTIONS } from '../utils/definitions';

/**
 * Get function completion items
 * @returns Array of completion items for functions
 */
export function getFunctionCompletions(): vscode.CompletionItem[] {
    return FUNCTIONS.map(func => {
        const item = new vscode.CompletionItem(func.name, vscode.CompletionItemKind.Function);
        item.detail = `${func.name}${func.signature} -> ${func.returnType}`;
        const doc = `${func.description}\n\n**Syntax:**\n\`\`\`ferrisscript\n${func.syntax}\n\`\`\`\n\n**Example:**\n\`\`\`ferrisscript\n${func.example}\n\`\`\``;
        item.documentation = new vscode.MarkdownString(doc);
        item.insertText = new vscode.SnippetString(`${func.name}($0)`);
        return item;
    });
}
