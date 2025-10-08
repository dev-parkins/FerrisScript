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
     * Checks: user configuration, workspace, PATH, cargo target dir
     * 
     * Security: For maximum security, users should set an absolute path
     * via ferrisscript.compilerPath setting to avoid PATH-based attacks.
     */
    private findCompiler(): string | undefined {
        // 1. Check user configuration (most secure - absolute path)
        const config = vscode.workspace.getConfiguration('ferrisscript');
        const configuredPath = config.get<string>('compilerPath');
        if (configuredPath && configuredPath.trim() !== '') {
            try {
                const fs = require('fs');
                if (fs.existsSync(configuredPath)) {
                    console.log(`Using configured FerrisScript compiler: ${configuredPath}`);
                    return configuredPath;
                } else {
                    console.warn(`Configured compiler path not found: ${configuredPath}`);
                }
            } catch (e) {
                console.error('Error checking configured compiler path:', e);
            }
        }

        // 2. Try to find compiler in workspace
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
        // Security Note: Using PATH to find compiler is a legitimate use case for
        // CLI tool discovery. The risk is mitigated by:
        // 1. Using spawnSync with shell:false (no command injection)
        // 2. Only executing with validated arguments (--version, file paths)
        // 3. Timeout protection (prevents hanging)
        // 4. User notification when compiler is found (transparency)
        // Alternative: Require absolute path in settings, but reduces UX
        try {
            const result = cp.spawnSync('ferrisscript', ['--version'], { 
                encoding: 'utf-8',
                shell: false,  // No shell - prevents command injection
                timeout: 3000  // Prevent hanging if malicious binary
            });
            if (result.status === 0) {
                console.log('Found FerrisScript compiler in PATH');
                // Note: Returns 'ferrisscript' which will be resolved via PATH
                // when actually executed. This is standard practice for CLI tools.
                return 'ferrisscript';
            }
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
     * 
     * Security: Uses spawnSync without shell to prevent command injection.
     * The compiler path is validated during findCompiler() and file paths
     * come from VS Code's document URIs (trusted sources).
     */
    private runCompiler(filePath: string): string | undefined {
        if (!this.compilerPath) {
            return undefined;
        }

        try {
            // Security: Use spawnSync without shell to avoid command injection
            // Pass arguments as array instead of concatenating into command string
            const result = cp.spawnSync(this.compilerPath, [filePath], {
                encoding: 'utf-8',
                timeout: 5000,
                shell: false  // Don't spawn a shell - prevents command injection
            });
            
            // Combine stdout and stderr
            const stdout = result.stdout || '';
            const stderr = result.stderr || '';
            const output = stderr + stdout;
            
            // Log for debugging
            if (output.length > 0) {
                console.log('FerrisScript compiler output:', output);
            }
            
            // Check if output contains errors
            if (output.includes('Error[')) {
                return output;
            }
            
            return undefined;
        } catch (error: any) {
            console.error('FerrisScript compiler execution error:', error);
            return undefined;
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
