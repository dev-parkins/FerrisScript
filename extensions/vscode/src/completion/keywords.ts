import * as vscode from 'vscode';
import { KEYWORDS } from '../utils/definitions';

/**
 * Get keyword completion items
 * @param statementLevelOnly If true, only return keywords valid at statement start
 * @returns Array of completion items for keywords
 */
export function getKeywordCompletions(statementLevelOnly: boolean): vscode.CompletionItem[] {
    const filtered = statementLevelOnly 
        ? KEYWORDS.filter(k => k.statementLevel)
        : KEYWORDS;
    
    return filtered.map(kw => {
        const item = new vscode.CompletionItem(kw.name, vscode.CompletionItemKind.Keyword);
        item.detail = kw.category;
        const doc = `${kw.description}\n\n**Syntax:**\n\`\`\`ferrisscript\n${kw.syntax}\n\`\`\`\n\n**Example:**\n\`\`\`ferrisscript\n${kw.example}\n\`\`\``;
        item.documentation = new vscode.MarkdownString(doc);
        item.insertText = new vscode.SnippetString(kw.insertText);
        return item;
    });
}
