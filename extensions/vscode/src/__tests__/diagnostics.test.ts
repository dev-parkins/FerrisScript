/**
 * Tests for FerrisScript Diagnostics Provider
 */

import * as vscode from 'vscode';
import * as cp from 'child_process';
import { FerrisScriptDiagnosticProvider } from '../diagnostics/provider';
import { parseCompilerErrors } from '../diagnostics/parser';

// Mock child_process module
jest.mock('child_process');
const mockedCp = cp as jest.Mocked<typeof cp>;

// Mock fs module
jest.mock('fs');

describe('FerrisScriptDiagnosticProvider', () => {
    let provider: FerrisScriptDiagnosticProvider;
    let mockDocument: vscode.TextDocument;
    let mockDiagnosticCollection: vscode.DiagnosticCollection;

    beforeEach(() => {
        // Reset all mocks
        jest.clearAllMocks();

        // Mock diagnostic collection
        mockDiagnosticCollection = {
            clear: jest.fn(),
            delete: jest.fn(),
            dispose: jest.fn(),
            forEach: jest.fn(),
            get: jest.fn(),
            has: jest.fn(),
            set: jest.fn(),
            name: 'ferrisscript',
        } as unknown as vscode.DiagnosticCollection;

        jest.spyOn(vscode.languages, 'createDiagnosticCollection').mockReturnValue(mockDiagnosticCollection);

        // Mock document
        mockDocument = {
            languageId: 'ferrisscript',
            uri: vscode.Uri.file('/test/test.ferris'),
            lineAt: jest.fn((lineNum: number) => ({
                text: 'let velocty = 5;',
                range: new vscode.Range(lineNum, 0, lineNum, 16),
            })),
        } as unknown as vscode.TextDocument;
    });

    describe('Constructor and Compiler Discovery', () => {
        it('should create diagnostic collection', () => {
            provider = new FerrisScriptDiagnosticProvider();
            expect(vscode.languages.createDiagnosticCollection).toHaveBeenCalledWith('ferrisscript');
        });

        it('should find compiler in configured path', () => {
            // Mock configuration
            const mockConfig = {
                get: jest.fn((key: string) => {
                    if (key === 'compilerPath') {
                        return '/usr/bin/ferrisscript';
                    }
                    return undefined;
                }),
            };
            jest.spyOn(vscode.workspace, 'getConfiguration').mockReturnValue(mockConfig as any);

            // Mock fs.existsSync to return true
            const fs = require('fs');
            fs.existsSync = jest.fn().mockReturnValue(true);

            const consoleLogSpy = jest.spyOn(console, 'log').mockImplementation();

            provider = new FerrisScriptDiagnosticProvider();

            expect(consoleLogSpy).toHaveBeenCalledWith(
                expect.stringContaining('Using configured FerrisScript compiler')
            );

            consoleLogSpy.mockRestore();
        });

        it('should find compiler in workspace target directory', () => {
            // Mock configuration (no configured path)
            const mockConfig = {
                get: jest.fn(() => undefined),
            };
            jest.spyOn(vscode.workspace, 'getConfiguration').mockReturnValue(mockConfig as any);

            // Mock workspace folders
            const mockWorkspaceFolder = {
                uri: vscode.Uri.file('/workspace'),
                name: 'workspace',
                index: 0,
            };
            Object.defineProperty(vscode.workspace, 'workspaceFolders', {
                value: [mockWorkspaceFolder],
                configurable: true,
            });

            // Mock fs.existsSync to return true for workspace path (handle both Unix and Windows separators)
            const fs = require('fs');
            fs.existsSync = jest.fn((filePath: string) => {
                return filePath.includes('target') && (filePath.includes('debug') || filePath.includes('release'));
            });

            const consoleLogSpy = jest.spyOn(console, 'log').mockImplementation();

            provider = new FerrisScriptDiagnosticProvider();

            expect(consoleLogSpy).toHaveBeenCalledWith(
                expect.stringContaining('Found FerrisScript compiler at')
            );

            consoleLogSpy.mockRestore();
        });

        it('should find compiler in PATH', () => {
            // Mock configuration (no configured path)
            const mockConfig = {
                get: jest.fn(() => undefined),
            };
            jest.spyOn(vscode.workspace, 'getConfiguration').mockReturnValue(mockConfig as any);

            // Mock no workspace folders
            Object.defineProperty(vscode.workspace, 'workspaceFolders', {
                value: undefined,
                configurable: true,
            });

            // Mock spawnSync for PATH check
            mockedCp.spawnSync.mockReturnValue({
                status: 0,
                stdout: 'ferrisscript 0.0.3',
                stderr: '',
            } as any);

            const consoleLogSpy = jest.spyOn(console, 'log').mockImplementation();

            provider = new FerrisScriptDiagnosticProvider();

            expect(mockedCp.spawnSync).toHaveBeenCalledWith(
                'ferrisscript',
                ['--version'],
                expect.objectContaining({ shell: false, timeout: 3000 })
            );

            expect(consoleLogSpy).toHaveBeenCalledWith(
                expect.stringContaining('Found FerrisScript compiler in PATH')
            );

            consoleLogSpy.mockRestore();
        });

        it('should handle compiler not found', () => {
            // Mock configuration (no configured path)
            const mockConfig = {
                get: jest.fn(() => undefined),
            };
            jest.spyOn(vscode.workspace, 'getConfiguration').mockReturnValue(mockConfig as any);

            // Mock no workspace folders
            Object.defineProperty(vscode.workspace, 'workspaceFolders', {
                value: undefined,
                configurable: true,
            });

            // Mock spawnSync failure
            mockedCp.spawnSync.mockReturnValue({
                status: 1,
                error: new Error('Command not found'),
            } as any);

            const consoleWarnSpy = jest.spyOn(console, 'warn').mockImplementation();

            provider = new FerrisScriptDiagnosticProvider();

            expect(consoleWarnSpy).toHaveBeenCalledWith(
                expect.stringContaining('Diagnostics disabled')
            );

            consoleWarnSpy.mockRestore();
        });

        it('should handle fs.existsSync errors gracefully', () => {
            // Mock configuration
            const mockConfig = {
                get: jest.fn((key: string) => {
                    if (key === 'compilerPath') {
                        return '/usr/bin/ferrisscript';
                    }
                    return undefined;
                }),
            };
            jest.spyOn(vscode.workspace, 'getConfiguration').mockReturnValue(mockConfig as any);

            // Mock fs.existsSync to throw error
            const fs = require('fs');
            fs.existsSync = jest.fn().mockImplementation(() => {
                throw new Error('Permission denied');
            });

            const consoleErrorSpy = jest.spyOn(console, 'error').mockImplementation();

            provider = new FerrisScriptDiagnosticProvider();

            expect(consoleErrorSpy).toHaveBeenCalledWith(
                expect.stringContaining('Error checking configured compiler path'),
                expect.any(String)
            );

            consoleErrorSpy.mockRestore();
        });

        it('should handle configured path that does not exist', () => {
            // Mock configuration with invalid path
            const mockConfig = {
                get: jest.fn((key: string) => {
                    if (key === 'compilerPath') {
                        return '/nonexistent/ferrisscript';
                    }
                    return undefined;
                }),
            };
            jest.spyOn(vscode.workspace, 'getConfiguration').mockReturnValue(mockConfig as any);

            // Mock fs.existsSync to return false
            const fs = require('fs');
            fs.existsSync = jest.fn().mockReturnValue(false);

            const consoleWarnSpy = jest.spyOn(console, 'warn').mockImplementation();

            provider = new FerrisScriptDiagnosticProvider();

            expect(consoleWarnSpy).toHaveBeenCalledWith(
                expect.stringContaining('Configured compiler path not found')
            );

            consoleWarnSpy.mockRestore();
        });

        it('should handle empty configured path', () => {
            // Mock configuration with empty string
            const mockConfig = {
                get: jest.fn((key: string) => {
                    if (key === 'compilerPath') {
                        return '   '; // Whitespace only
                    }
                    return undefined;
                }),
            };
            jest.spyOn(vscode.workspace, 'getConfiguration').mockReturnValue(mockConfig as any);

            // Mock no workspace folders
            Object.defineProperty(vscode.workspace, 'workspaceFolders', {
                value: undefined,
                configurable: true,
            });

            // Mock spawnSync failure
            mockedCp.spawnSync.mockReturnValue({
                status: 1,
                error: new Error('Command not found'),
            } as any);

            const consoleWarnSpy = jest.spyOn(console, 'warn').mockImplementation();

            provider = new FerrisScriptDiagnosticProvider();

            expect(consoleWarnSpy).toHaveBeenCalledWith(
                expect.stringContaining('Diagnostics disabled')
            );

            consoleWarnSpy.mockRestore();
        });

        it('should handle workspace folder fileExists error', () => {
            // Mock configuration (no configured path)
            const mockConfig = {
                get: jest.fn(() => undefined),
            };
            jest.spyOn(vscode.workspace, 'getConfiguration').mockReturnValue(mockConfig as any);

            // Mock workspace folders
            const mockWorkspaceFolder = {
                uri: vscode.Uri.file('/workspace'),
                name: 'workspace',
                index: 0,
            };
            Object.defineProperty(vscode.workspace, 'workspaceFolders', {
                value: [mockWorkspaceFolder],
                configurable: true,
            });

            // Mock fs.existsSync to throw error (simulate I/O error)
            const fs = require('fs');
            fs.existsSync = jest.fn().mockImplementation(() => {
                throw new Error('I/O error');
            });

            const consoleErrorSpy = jest.spyOn(console, 'error').mockImplementation();

            provider = new FerrisScriptDiagnosticProvider();

            expect(consoleErrorSpy).toHaveBeenCalledWith(
                expect.stringContaining('Error checking file existence'),
                expect.any(String)
            );

            consoleErrorSpy.mockRestore();
        });

        it('should handle PATH compiler check with spawnSync exception', () => {
            // Mock configuration (no configured path)
            const mockConfig = {
                get: jest.fn(() => undefined),
            };
            jest.spyOn(vscode.workspace, 'getConfiguration').mockReturnValue(mockConfig as any);

            // Mock no workspace folders
            Object.defineProperty(vscode.workspace, 'workspaceFolders', {
                value: undefined,
                configurable: true,
            });

            // Mock spawnSync to throw exception
            mockedCp.spawnSync.mockImplementation(() => {
                throw new Error('ENOENT: command not found');
            });

            const consoleDebugSpy = jest.spyOn(console, 'debug').mockImplementation();

            provider = new FerrisScriptDiagnosticProvider();

            expect(consoleDebugSpy).toHaveBeenCalledWith(
                expect.stringContaining('Compiler not in PATH'),
                expect.any(String)
            );

            consoleDebugSpy.mockRestore();
        });
    });

    describe('updateDiagnostics', () => {
        beforeEach(() => {
            // Setup provider with compiler found in PATH
            const mockConfig = { get: jest.fn(() => undefined) };
            jest.spyOn(vscode.workspace, 'getConfiguration').mockReturnValue(mockConfig as any);
            Object.defineProperty(vscode.workspace, 'workspaceFolders', {
                value: undefined,
                configurable: true,
            });
            mockedCp.spawnSync.mockReturnValue({
                status: 0,
                stdout: 'ferrisscript 0.0.3',
                stderr: '',
            } as any);

            provider = new FerrisScriptDiagnosticProvider();
            jest.clearAllMocks(); // Clear setup calls
        });

        it('should skip non-ferrisscript documents', () => {
            const nonFerrisDoc = {
                languageId: 'typescript',
                uri: vscode.Uri.file('/test/test.ts'),
            } as vscode.TextDocument;

            provider.updateDiagnostics(nonFerrisDoc);

            expect(mockDiagnosticCollection.delete).not.toHaveBeenCalled();
        });

        it('should clear existing diagnostics for document', () => {
            provider.updateDiagnostics(mockDocument);

            expect(mockDiagnosticCollection.delete).toHaveBeenCalledWith(mockDocument.uri);
        });

        it('should run compiler and set diagnostics on error', () => {
            // Mock compiler output with error
            mockedCp.spawnSync.mockReturnValue({
                status: 1,
                stdout: '',
                stderr: `Error[E201]: Undefined variable 'velocty'
  --> test.ferris:5:10
help: a variable with a similar name exists
   | Did you mean 'velocity'?`,
            } as any);

            provider.updateDiagnostics(mockDocument);

            expect(mockedCp.spawnSync).toHaveBeenCalledWith(
                'ferrisscript',
                ['/test/test.ferris'],
                expect.objectContaining({ shell: false, timeout: 5000 })
            );

            expect(mockDiagnosticCollection.set).toHaveBeenCalledWith(
                mockDocument.uri,
                expect.arrayContaining([
                    expect.objectContaining({
                        message: expect.stringContaining('[E201]'),
                        severity: vscode.DiagnosticSeverity.Error,
                    }),
                ])
            );
        });

        it('should not set diagnostics when no errors', () => {
            // Mock compiler output with no errors
            mockedCp.spawnSync.mockReturnValue({
                status: 0,
                stdout: 'Compilation successful',
                stderr: '',
            } as any);

            provider.updateDiagnostics(mockDocument);

            expect(mockDiagnosticCollection.set).not.toHaveBeenCalled();
        });

        it('should handle compiler execution errors gracefully', () => {
            // Mock compiler execution error
            mockedCp.spawnSync.mockImplementation(() => {
                throw new Error('ENOENT: no such file or directory');
            });

            const consoleErrorSpy = jest.spyOn(console, 'error').mockImplementation();

            provider.updateDiagnostics(mockDocument);

            expect(consoleErrorSpy).toHaveBeenCalledWith(
                expect.stringContaining('compiler execution error'),
                expect.any(String)
            );

            consoleErrorSpy.mockRestore();
        });

        it('should handle compiler with warnings', () => {
            // Mock compiler output with warnings that includes "Error[" marker
            mockedCp.spawnSync.mockReturnValue({
                status: 0,
                stdout: '',
                stderr: `Warning[W101]: Unused variable 'x'
  --> test.ferris:2:9
help: consider removing this variable
Error[E999]: Placeholder to trigger diagnostic set`,
            } as any);

            // Mock document with warning location
            const warnDoc = {
                languageId: 'ferrisscript',
                uri: vscode.Uri.file('/test/warn.ferris'),
                lineAt: jest.fn((lineNum: number) => ({
                    text: '    let x = 5;',
                    range: new vscode.Range(lineNum, 0, lineNum, 14),
                })),
            } as unknown as vscode.TextDocument;

            provider.updateDiagnostics(warnDoc);

            // Should set diagnostics (includes warnings)
            expect(mockDiagnosticCollection.set).toHaveBeenCalled();
        });

        it('should handle compiler output with both stdout and stderr', () => {
            // Mock compiler output with both stdout and stderr
            mockedCp.spawnSync.mockReturnValue({
                status: 1,
                stdout: 'Some compilation output',
                stderr: `Error[E201]: Undefined variable 'velocty'
  --> test.ferris:5:10`,
            } as any);

            provider.updateDiagnostics(mockDocument);

            expect(mockDiagnosticCollection.set).toHaveBeenCalled();
        });

        it('should handle non-error compiler output', () => {
            // Mock compiler output with no Error[ markers
            mockedCp.spawnSync.mockReturnValue({
                status: 0,
                stdout: 'Compiled successfully',
                stderr: '',
            } as any);

            provider.updateDiagnostics(mockDocument);

            // Should not set diagnostics
            expect(mockDiagnosticCollection.set).not.toHaveBeenCalled();
        });

        it('should handle compiler output logging', () => {
            // Mock compiler output
            mockedCp.spawnSync.mockReturnValue({
                status: 1,
                stdout: 'Some output',
                stderr: `Error[E201]: Test error
  --> test.ferris:1:1`,
            } as any);

            const consoleLogSpy = jest.spyOn(console, 'log').mockImplementation();

            provider.updateDiagnostics(mockDocument);

            expect(consoleLogSpy).toHaveBeenCalledWith(
                expect.stringContaining('FerrisScript compiler output'),
                expect.any(String)
            );

            consoleLogSpy.mockRestore();
        });
    });

    describe('Resource Management', () => {
        it('should clear all diagnostics', () => {
            // Setup provider
            const mockConfig = { get: jest.fn(() => undefined) };
            jest.spyOn(vscode.workspace, 'getConfiguration').mockReturnValue(mockConfig as any);
            Object.defineProperty(vscode.workspace, 'workspaceFolders', {
                value: undefined,
                configurable: true,
            });
            mockedCp.spawnSync.mockReturnValue({ status: 1 } as any);

            provider = new FerrisScriptDiagnosticProvider();
            jest.clearAllMocks();

            provider.clearAll();

            expect(mockDiagnosticCollection.clear).toHaveBeenCalled();
        });

        it('should dispose of diagnostic collection', () => {
            // Setup provider
            const mockConfig = { get: jest.fn(() => undefined) };
            jest.spyOn(vscode.workspace, 'getConfiguration').mockReturnValue(mockConfig as any);
            Object.defineProperty(vscode.workspace, 'workspaceFolders', {
                value: undefined,
                configurable: true,
            });
            mockedCp.spawnSync.mockReturnValue({ status: 1 } as any);

            provider = new FerrisScriptDiagnosticProvider();
            jest.clearAllMocks();

            provider.dispose();

            expect(mockDiagnosticCollection.dispose).toHaveBeenCalled();
        });
    });
});

describe('parseCompilerErrors', () => {
    let mockDocument: vscode.TextDocument;

    beforeEach(() => {
        mockDocument = {
            lineAt: jest.fn((lineNum: number) => {
                const lines = [
                    'fn main() {',
                    '    let x = 5;',
                    '    let y = 10;',
                    '    // This line has an error',
                    '    let velocty = 5;',
                    '    print(velocity);',
                    '}',
                ];
                return {
                    text: lines[lineNum] || '',
                    range: new vscode.Range(lineNum, 0, lineNum, lines[lineNum]?.length || 0),
                };
            }),
        } as unknown as vscode.TextDocument;
    });

    it('should parse single error', () => {
        const output = `Error[E201]: Undefined variable 'velocty'
  --> test.ferris:5:10
help: a variable with a similar name exists`;

        const diagnostics = parseCompilerErrors(output, mockDocument);

        expect(diagnostics).toHaveLength(1);
        expect(diagnostics[0]).toMatchObject({
            message: "[E201] Undefined variable 'velocty'",
            severity: vscode.DiagnosticSeverity.Error,
            code: 'E201',
            source: 'ferrisscript',
        });

        // Verify range
        expect(diagnostics[0].range.start.line).toBe(4); // 0-indexed
        expect(diagnostics[0].range.start.character).toBe(9); // 0-indexed
    });

    it('should parse multiple errors', () => {
        const output = `Error[E201]: Undefined variable 'velocty'
  --> test.ferris:5:10
help: a variable with a similar name exists

Error[E302]: Type mismatch
  --> test.ferris:6:5
help: expected i32, found String`;

        const diagnostics = parseCompilerErrors(output, mockDocument);

        expect(diagnostics).toHaveLength(2);
        expect(diagnostics[0].message).toContain('E201');
        expect(diagnostics[1].message).toContain('E302');
    });

    it('should parse warnings', () => {
        const output = `Warning[W101]: Unused variable 'x'
  --> test.ferris:2:9
help: consider removing this variable`;

        const diagnostics = parseCompilerErrors(output, mockDocument);

        expect(diagnostics).toHaveLength(1);
        expect(diagnostics[0]).toMatchObject({
            message: "[W101] Unused variable 'x'",
            severity: vscode.DiagnosticSeverity.Warning,
            code: 'W101',
            source: 'ferrisscript',
        });
    });

    it('should parse both errors and warnings', () => {
        const output = `Error[E201]: Undefined variable 'velocty'
  --> test.ferris:5:10

Warning[W101]: Unused variable 'x'
  --> test.ferris:2:9`;

        const diagnostics = parseCompilerErrors(output, mockDocument);

        expect(diagnostics).toHaveLength(2);
        expect(diagnostics[0].severity).toBe(vscode.DiagnosticSeverity.Error);
        expect(diagnostics[1].severity).toBe(vscode.DiagnosticSeverity.Warning);
    });

    it('should calculate error length based on word at position', () => {
        const output = `Error[E201]: Undefined variable 'velocty'
  --> test.ferris:5:10`;

        const diagnostics = parseCompilerErrors(output, mockDocument);

        expect(diagnostics[0].range.start.character).toBe(9); // Start of 'velocty'
        expect(diagnostics[0].range.end.character).toBeGreaterThan(9); // End after 'velocty'
    });

    it('should handle empty output', () => {
        const output = '';

        const diagnostics = parseCompilerErrors(output, mockDocument);

        expect(diagnostics).toHaveLength(0);
    });

    it('should handle output with no errors or warnings', () => {
        const output = 'Compilation successful\nNo errors found';

        const diagnostics = parseCompilerErrors(output, mockDocument);

        expect(diagnostics).toHaveLength(0);
    });
});
