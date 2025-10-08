import * as vscode from 'vscode';

/**
 * Hover data for FerrisScript keywords
 */
interface KeywordHoverData {
    keyword: string;
    description: string;
    syntax: string;
    example: string;
}

const KEYWORD_HOVERS: KeywordHoverData[] = [
    {
        keyword: 'let',
        description: 'Declares an immutable variable',
        syntax: 'let name: type = value;',
        example: 'let speed: f32 = 100.0;\nlet name: String = "Player";'
    },
    {
        keyword: 'mut',
        description: 'Declares a mutable variable (used with let)',
        syntax: 'let mut name: type = value;',
        example: 'let mut position: Vector2 = Vector2(0.0, 0.0);\nposition.x += 10.0;'
    },
    {
        keyword: 'fn',
        description: 'Declares a function',
        syntax: 'fn name(param: type) -> return_type { ... }',
        example: 'fn update(delta: f32) -> void {\n    self.position.x += 50.0 * delta;\n}'
    },
    {
        keyword: 'if',
        description: 'Conditional statement - executes code if condition is true',
        syntax: 'if condition { ... }',
        example: 'if speed > 100.0 {\n    print("Going fast!");\n}'
    },
    {
        keyword: 'else',
        description: 'Alternative branch for if statement',
        syntax: 'if condition { ... } else { ... }',
        example: 'if speed > 100.0 {\n    print("Fast");\n} else {\n    print("Slow");\n}'
    },
    {
        keyword: 'while',
        description: 'Loop that executes while condition is true',
        syntax: 'while condition { ... }',
        example: 'let mut count: i32 = 0;\nwhile count < 10 {\n    count += 1;\n}'
    },
    {
        keyword: 'return',
        description: 'Returns a value from a function',
        syntax: 'return value;',
        example: 'fn get_speed() -> f32 {\n    return 100.0;\n}'
    },
    {
        keyword: 'true',
        description: 'Boolean literal representing true',
        syntax: 'true',
        example: 'let is_active: bool = true;\nif is_active {\n    print("Active!");\n}'
    },
    {
        keyword: 'false',
        description: 'Boolean literal representing false',
        syntax: 'false',
        example: 'let is_paused: bool = false;\nif !is_paused {\n    update(delta);\n}'
    }
];

/**
 * Get hover content for a keyword
 * @param word The word to get hover content for
 * @returns Markdown hover content, or undefined if not a keyword
 */
export function getKeywordHover(word: string): vscode.MarkdownString | undefined {
    const data = KEYWORD_HOVERS.find(kw => kw.keyword === word);
    if (!data) {
        return undefined;
    }

    const md = new vscode.MarkdownString();
    md.appendMarkdown(`**\`${data.keyword}\`** - ${data.description}\n\n`);
    md.appendMarkdown(`**Syntax**: \`${data.syntax}\`\n\n`);
    md.appendMarkdown(`**Example**:\n`);
    md.appendCodeblock(data.example, 'ferrisscript');
    md.isTrusted = true;

    return md;
}
