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
    private readonly diagnosticCollection: vscode.DiagnosticCollection;
    private readonly compilerPath: string | undefined;

    constructor() {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('ferrisscript');
        this.compilerPath = this.findCompiler();
        
        // Notify user about compiler status asynchronously
        this.notifyCompilerStatus();
    }

    /**
     * Notify user about compiler status (async operation moved out of constructor)
     */
    private notifyCompilerStatus(): void {
        if (this.compilerPath) {
            console.log(`FerrisScript: Diagnostics enabled (compiler: ${this.compilerPath})`);
            void vscode.window.showInformationMessage(
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
        const configuredCompiler = this.tryConfiguredCompiler();
        if (configuredCompiler) {
            return configuredCompiler;
        }

        // 2. Try to find compiler in workspace
        const workspaceCompiler = this.tryWorkspaceCompiler();
        if (workspaceCompiler) {
            return workspaceCompiler;
        }

        // 3. Try to find in PATH
        const pathCompiler = this.tryPathCompiler();
        if (pathCompiler) {
            return pathCompiler;
        }

        console.warn('FerrisScript compiler not found. Diagnostics will be disabled.');
        return undefined;
    }

    /**
     * Try to use configured compiler path from settings
     */
    private tryConfiguredCompiler(): string | undefined {
        const config = vscode.workspace.getConfiguration('ferrisscript');
        const configuredPath = config.get<string>('compilerPath');
        
        if (!configuredPath || configuredPath.trim() === '') {
            return undefined;
        }

        try {
            const fs = require('fs');
            if (fs.existsSync(configuredPath)) {
                console.log(`Using configured FerrisScript compiler: ${configuredPath}`);
                return configuredPath;
            }
            console.warn(`Configured compiler path not found: ${configuredPath}`);
        } catch (error: unknown) {
            console.error('Error checking configured compiler path:', error instanceof Error ? error.message : String(error));
        }
        
        return undefined;
    }

    /**
     * Try to find compiler in workspace cargo target directories
     */
    private tryWorkspaceCompiler(): string | undefined {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            return undefined;
        }

        const workspacePath = workspaceFolders[0].uri.fsPath;
        const possiblePaths = [
            path.join(workspacePath, 'target', 'debug', 'ferrisscript.exe'),
            path.join(workspacePath, 'target', 'debug', 'ferrisscript'),
            path.join(workspacePath, 'target', 'release', 'ferrisscript.exe'),
            path.join(workspacePath, 'target', 'release', 'ferrisscript'),
        ];

        for (const compilerPath of possiblePaths) {
            if (this.fileExists(compilerPath)) {
                console.log(`Found FerrisScript compiler at: ${compilerPath}`);
                return compilerPath;
            }
        }

        return undefined;
    }

    /**
     * Check if file exists safely
     */
    private fileExists(filePath: string): boolean {
        try {
            const fs = require('fs');
            return fs.existsSync(filePath);
        } catch (error: unknown) {
            console.error(`Error checking file existence for ${filePath}:`, error instanceof Error ? error.message : String(error));
            return false;
        }
    }

    /**
     * Try to find compiler in system PATH
     * Security Note: Using PATH to find compiler is a legitimate use case for
     * CLI tool discovery. The risk is mitigated by:
     * 1. Using spawnSync with shell:false (no command injection)
     * 2. Only executing with validated arguments (--version, file paths)
     * 3. Timeout protection (prevents hanging)
     * 4. User notification when compiler is found (transparency)
     */
    private tryPathCompiler(): string | undefined {
        try {
            const result = cp.spawnSync('ferrisscript', ['--version'], { 
                encoding: 'utf-8',
                shell: false,  // No shell - prevents command injection
                timeout: 3000  // Prevent hanging if malicious binary
            });
            if (result.status === 0) {
                console.log('Found FerrisScript compiler in PATH');
                return 'ferrisscript';
            }
        } catch (error: unknown) {
            console.debug('Compiler not in PATH:', error instanceof Error ? error.message : String(error));
        }
        
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
        } catch (error: unknown) {
            // Log error and return undefined to gracefully handle compiler execution failures
            const errorMessage = error instanceof Error ? error.message : String(error);
            console.error('FerrisScript compiler execution error:', errorMessage);
            
            // Notify user if compiler execution failed (could be permissions, missing dependencies, etc.)
            void vscode.window.showWarningMessage(
                `FerrisScript: Failed to run compiler at ${this.compilerPath}. Check output for details.`
            );
            
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
