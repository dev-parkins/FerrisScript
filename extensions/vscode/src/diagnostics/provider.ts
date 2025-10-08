import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';
import { parseCompilerErrors } from './parser';

/**
 * FerrisScript Diagnostic Provider
 * 
 * Integrates with the FerrisScript compiler to show errors and warnings
 * in VS Code's problem panel and as inline squiggles.
 */
export class FerrisScriptDiagnosticProvider {
    private diagnosticCollection: vscode.DiagnosticCollection;
    private compilerPath: string | undefined;

    constructor() {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('ferrisscript');
        this.compilerPath = this.findCompiler();
        
        // Notify user about compiler status
        if (this.compilerPath) {
            console.log(`FerrisScript: Diagnostics enabled (compiler: ${this.compilerPath})`);
            vscode.window.showInformationMessage(
                `FerrisScript: Diagnostics enabled using compiler at ${this.compilerPath}`
            );
        } else {
            console.warn('FerrisScript: Diagnostics disabled (compiler not found)');
            // Note: Diagnostic provider infrastructure is in place,
            // but requires a standalone CLI executable to function.
            // This will be added in a future version.
        }
    }

    /**
     * Find the FerrisScript compiler executable
     * Checks: workspace, PATH, cargo target dir
     */
    private findCompiler(): string | undefined {
        // Try to find compiler in workspace
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (workspaceFolders) {
            const workspacePath = workspaceFolders[0].uri.fsPath;
            
            // Check common locations
            const possiblePaths = [
                path.join(workspacePath, 'target', 'debug', 'ferrisscript.exe'),
                path.join(workspacePath, 'target', 'debug', 'ferrisscript'),
                path.join(workspacePath, 'target', 'release', 'ferrisscript.exe'),
                path.join(workspacePath, 'target', 'release', 'ferrisscript'),
            ];

            for (const compilerPath of possiblePaths) {
                try {
                    const fs = require('fs');
                    if (fs.existsSync(compilerPath)) {
                        console.log(`Found FerrisScript compiler at: ${compilerPath}`);
                        return compilerPath;
                    }
                } catch (e) {
                    // Continue searching
                }
            }
        }

        // Try to find in PATH
        try {
            cp.execSync('ferrisscript --version', { encoding: 'utf-8' });
            console.log('Found FerrisScript compiler in PATH');
            return 'ferrisscript';
        } catch (e) {
            // Not in PATH
        }

        console.warn('FerrisScript compiler not found. Diagnostics will be disabled.');
        return undefined;
    }

    /**
     * Update diagnostics for a document
     * Runs the FerrisScript compiler and parses errors
     */
    public updateDiagnostics(document: vscode.TextDocument): void {
        if (document.languageId !== 'ferrisscript') {
            return;
        }

        // Clear existing diagnostics for this document
        this.diagnosticCollection.delete(document.uri);

        if (!this.compilerPath) {
            // Show informational message that compiler is not found
            // (only do this once per session to avoid spam)
            return;
        }

        // Run compiler
        const errors = this.runCompiler(document.uri.fsPath);
        if (errors) {
            const diagnostics = parseCompilerErrors(errors, document);
            this.diagnosticCollection.set(document.uri, diagnostics);
        }
    }

    /**
     * Run the FerrisScript compiler and capture output
     */
    private runCompiler(filePath: string): string | undefined {
        if (!this.compilerPath) {
            return undefined;
        }

        try {
            // Run compiler - it will throw if there are compilation errors
            const result = cp.execSync(`"${this.compilerPath}" "${filePath}"`, {
                encoding: 'utf-8',
                timeout: 5000
            });
            
            // No errors if we get here - but check output anyway
            // Some compilers write errors to stdout
            if (result && result.includes('Error[')) {
                return result;
            }
            return undefined;
        } catch (error: any) {
            // Compiler errors are in stderr or stdout
            const stderr = error.stderr ? error.stderr.toString() : '';
            const stdout = error.stdout ? error.stdout.toString() : '';
            const output = stderr + stdout;
            
            // Log for debugging
            console.log('FerrisScript compiler output:', output);
            
            return output.length > 0 ? output : undefined;
        }
    }

    /**
     * Clear all diagnostics
     */
    public clearAll(): void {
        this.diagnosticCollection.clear();
    }

    /**
     * Dispose of resources
     */
    public dispose(): void {
        this.diagnosticCollection.dispose();
    }
}
