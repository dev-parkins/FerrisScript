import * as vscode from 'vscode';
import { getKeyword } from '../utils/definitions';

/**
 * Get hover content for a keyword
 * @param word The word to get hover content for
 * @returns Markdown hover content, or undefined if not a keyword
 */
export function getKeywordHover(word: string): vscode.MarkdownString | undefined {
    const data = getKeyword(word);
    if (!data) {
        return undefined;
    }

    const md = new vscode.MarkdownString();
    md.appendMarkdown(`**\`${data.name}\`** - ${data.description}\n\n`);
    md.appendMarkdown(`**Syntax**: \`${data.syntax}\`\n\n`);
    md.appendMarkdown(`**Example**:\n`);
    md.appendCodeblock(data.example, 'ferrisscript');
    md.isTrusted = true;

    return md;
}
