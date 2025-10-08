import * as vscode from 'vscode';

/**
 * Hover data for FerrisScript types
 */
interface TypeHoverData {
    type: string;
    description: string;
    category: string;
    example: string;
}

const TYPE_HOVERS: TypeHoverData[] = [
    {
        type: 'i32',
        description: '32-bit signed integer',
        category: 'Primitive Type',
        example: 'let count: i32 = 42;\nlet score: i32 = -100;'
    },
    {
        type: 'f32',
        description: '32-bit floating point number',
        category: 'Primitive Type',
        example: 'let speed: f32 = 100.5;\nlet delta: f32 = 0.016;'
    },
    {
        type: 'bool',
        description: 'Boolean value (true or false)',
        category: 'Primitive Type',
        example: 'let is_active: bool = true;\nlet can_move: bool = false;'
    },
    {
        type: 'String',
        description: 'Text string',
        category: 'Primitive Type',
        example: 'let name: String = "Player";\nlet message: String = "Hello, World!";'
    },
    {
        type: 'Vector2',
        description: '2D vector (x, y coordinates)',
        category: 'Godot Type',
        example: 'let position: Vector2 = Vector2(10.0, 20.0);\nlet velocity: Vector2 = Vector2(0.0, 0.0);'
    },
    {
        type: 'Node',
        description: 'Base Godot scene node',
        category: 'Godot Type',
        example: 'fn _ready() -> void {\n    let parent: Node = self.get_parent();\n}'
    },
    {
        type: 'void',
        description: 'No return value (used for functions)',
        category: 'Special Type',
        example: 'fn update(delta: f32) -> void {\n    // No return statement needed\n}'
    }
];

/**
 * Get hover content for a type
 * @param word The word to get hover content for
 * @returns Markdown hover content, or undefined if not a type
 */
export function getTypeHover(word: string): vscode.MarkdownString | undefined {
    const data = TYPE_HOVERS.find(t => t.type === word);
    if (!data) {
        return undefined;
    }

    const md = new vscode.MarkdownString();
    md.appendMarkdown(`**\`${data.type}\`** (${data.category})\n\n`);
    md.appendMarkdown(`${data.description}\n\n`);
    md.appendMarkdown(`**Example**:\n`);
    md.appendCodeblock(data.example, 'ferrisscript');
    md.isTrusted = true;

    return md;
}
