import * as vscode from 'vscode';

/**
 * Function definition for FerrisScript
 */
interface FunctionData {
    label: string;
    detail: string;
    documentation: string;
    insertText: string;
}

/**
 * FerrisScript built-in function definitions
 */
const FUNCTIONS: FunctionData[] = [
    {
        label: 'print',
        detail: 'print(message: String) -> void',
        documentation: 'Prints a message to the Godot console.\n\n**Parameters:**\n- `message: String` - The message to print\n\n**Returns:** `void`\n\n**Example:**\n```ferrisscript\nprint("Hello, World!");\nprint("Player position: " + position.x);\n```',
        insertText: 'print($0)'
    }
];

/**
 * Get function completion items
 * @returns Array of completion items for functions
 */
export function getFunctionCompletions(): vscode.CompletionItem[] {
    return FUNCTIONS.map(func => {
        const item = new vscode.CompletionItem(func.label, vscode.CompletionItemKind.Function);
        item.detail = func.detail;
        item.documentation = new vscode.MarkdownString(func.documentation);
        item.insertText = new vscode.SnippetString(func.insertText);
        return item;
    });
}
