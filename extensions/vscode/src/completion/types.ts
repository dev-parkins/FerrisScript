import * as vscode from 'vscode';
import { TYPES } from '../utils/definitions';

/**
 * Get type completion items
 * @returns Array of completion items for types
 */
export function getTypeCompletions(): vscode.CompletionItem[] {
    return TYPES.map(type => {
        const item = new vscode.CompletionItem(type.name, vscode.CompletionItemKind.Class);
        item.detail = type.category;
        const doc = `${type.description}\n\n**Syntax:**\n\`\`\`ferrisscript\n${type.syntax}\n\`\`\`\n\n**Example:**\n\`\`\`ferrisscript\n${type.example}\n\`\`\``;
        item.documentation = new vscode.MarkdownString(doc);
        item.insertText = type.name;
        return item;
    });
}
