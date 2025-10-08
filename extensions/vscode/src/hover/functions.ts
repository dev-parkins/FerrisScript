import * as vscode from 'vscode';
import { getFunction } from '../utils/definitions';

/**
 * Get hover content for a function
 * @param word The word to get hover content for
 * @returns Markdown hover content, or undefined if not a function
 */
export function getFunctionHover(word: string): vscode.MarkdownString | undefined {
    const data = getFunction(word);
    if (!data) {
        return undefined;
    }

    const md = new vscode.MarkdownString();
    
    // Function signature
    const signature = `${data.name}${data.signature} -> ${data.returnType}`;
    md.appendMarkdown(`**\`${signature}\`**\n\n`);
    
    // Description
    md.appendMarkdown(`${data.description}\n\n`);
    
    // Syntax
    md.appendMarkdown(`**Syntax**: \`${data.syntax}\`\n\n`);
    
    // Example
    md.appendMarkdown(`**Example**:\n`);
    md.appendCodeblock(data.example, 'ferrisscript');
    md.isTrusted = true;

    return md;
}
