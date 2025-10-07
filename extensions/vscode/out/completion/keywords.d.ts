import * as vscode from 'vscode';
/**
 * Get keyword completion items
 * @param statementLevelOnly If true, only return keywords valid at statement start
 * @returns Array of completion items for keywords
 */
export declare function getKeywordCompletions(statementLevelOnly: boolean): vscode.CompletionItem[];
