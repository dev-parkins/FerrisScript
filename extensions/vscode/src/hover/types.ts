import * as vscode from 'vscode';
import { getType } from '../utils/definitions';

/**
 * Get hover content for a type
 * @param word The word to get hover content for
 * @returns Markdown hover content, or undefined if not a type
 */
export function getTypeHover(word: string): vscode.MarkdownString | undefined {
    const data = getType(word);
    if (!data) {
        return undefined;
    }

    const md = new vscode.MarkdownString();
    md.appendMarkdown(`**\`${data.name}\`** (${data.category})\n\n`);
    md.appendMarkdown(`${data.description}\n\n`);
    md.appendMarkdown(`**Example**:\n`);
    md.appendCodeblock(data.example, 'ferrisscript');
    md.isTrusted = true;

    return md;
}
