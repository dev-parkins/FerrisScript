import * as vscode from 'vscode';

/**
 * Hover data for FerrisScript built-in functions
 */
interface FunctionHoverData {
    name: string;
    signature: string;
    description: string;
    parameters: Array<{ name: string; type: string; description: string }>;
    returnType: string;
    example: string;
}

const FUNCTION_HOVERS: FunctionHoverData[] = [
    {
        name: 'print',
        signature: 'print(message: String) -> void',
        description: 'Prints a message to the console',
        parameters: [
            {
                name: 'message',
                type: 'String',
                description: 'The message to print'
            }
        ],
        returnType: 'void',
        example: 'print("Hello, World!");\nlet name: String = "Player";\nprint("Name: " + name);'
    }
];

/**
 * Get hover content for a function
 * @param word The word to get hover content for
 * @returns Markdown hover content, or undefined if not a function
 */
export function getFunctionHover(word: string): vscode.MarkdownString | undefined {
    const data = FUNCTION_HOVERS.find(f => f.name === word);
    if (!data) {
        return undefined;
    }

    const md = new vscode.MarkdownString();
    
    // Function signature
    md.appendMarkdown(`**\`${data.signature}\`**\n\n`);
    
    // Description
    md.appendMarkdown(`${data.description}\n\n`);
    
    // Parameters
    if (data.parameters.length > 0) {
        md.appendMarkdown(`**Parameters**:\n`);
        for (const param of data.parameters) {
            md.appendMarkdown(`- \`${param.name}\` (\`${param.type}\`): ${param.description}\n`);
        }
        md.appendMarkdown(`\n`);
    }
    
    // Return type
    md.appendMarkdown(`**Returns**: \`${data.returnType}\`\n\n`);
    
    // Example
    md.appendMarkdown(`**Example**:\n`);
    md.appendCodeblock(data.example, 'ferrisscript');
    md.isTrusted = true;

    return md;
}
