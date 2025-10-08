import * as vscode from 'vscode';

/**
 * Parse FerrisScript compiler error output into VS Code diagnostics
 * 
 * Expected error format:
 * Error[E201]: Undefined variable 'velocty'
 *   --> move.ferris:5:10
 * help: a variable with a similar name exists
 *    | Did you mean 'velocity'?
 */
export function parseCompilerErrors(
    output: string,
    document: vscode.TextDocument
): vscode.Diagnostic[] {
    const diagnostics: vscode.Diagnostic[] = [];
    
    // Parse FerrisScript error format
    // Regex captures: Error code, message, line, column
    const errorRegex = /Error\[(\w+)\]: ([^\n]+)\n\s*--> [^:]+:(\d+):(\d+)/g;
    let match;

    while ((match = errorRegex.exec(output)) !== null) {
        const [, code, message, line, column] = match;
        
        const lineNum = parseInt(line, 10) - 1; // VS Code is 0-indexed
        const colNum = parseInt(column, 10) - 1;
        
        // Try to determine error length based on the word at that position
        const lineText = document.lineAt(lineNum).text;
        let errorLength = 1;
        
        // Find the end of the current word/token
        let endCol = colNum;
        while (endCol < lineText.length && /\w/.test(lineText[endCol])) {
            endCol++;
        }
        errorLength = endCol - colNum;
        
        const range = new vscode.Range(
            lineNum,
            colNum,
            lineNum,
            colNum + Math.max(errorLength, 1)
        );

        const diagnostic = new vscode.Diagnostic(
            range,
            `[${code}] ${message}`,
            vscode.DiagnosticSeverity.Error
        );
        
        diagnostic.code = code;
        diagnostic.source = 'ferrisscript';
        
        diagnostics.push(diagnostic);
    }

    // Also parse warnings (if compiler produces them)
    const warningRegex = /Warning\[(\w+)\]: ([^\n]+)\n\s*--> [^:]+:(\d+):(\d+)/g;
    
    while ((match = warningRegex.exec(output)) !== null) {
        const [, code, message, line, column] = match;
        
        const lineNum = parseInt(line, 10) - 1;
        const colNum = parseInt(column, 10) - 1;
        
        const lineText = document.lineAt(lineNum).text;
        let errorLength = 1;
        let endCol = colNum;
        while (endCol < lineText.length && /\w/.test(lineText[endCol])) {
            endCol++;
        }
        errorLength = endCol - colNum;
        
        const range = new vscode.Range(
            lineNum,
            colNum,
            lineNum,
            colNum + Math.max(errorLength, 1)
        );

        const diagnostic = new vscode.Diagnostic(
            range,
            `[${code}] ${message}`,
            vscode.DiagnosticSeverity.Warning
        );
        
        diagnostic.code = code;
        diagnostic.source = 'ferrisscript';
        
        diagnostics.push(diagnostic);
    }

    return diagnostics;
}
