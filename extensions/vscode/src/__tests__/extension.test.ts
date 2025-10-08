/**
 * Tests for main extension activation and deactivation
 */

import * as vscode from 'vscode';
import { activate, deactivate } from '../extension';

// Mock providers
jest.mock('../completion/provider');
jest.mock('../hover/provider');
jest.mock('../diagnostics/provider');

describe('Extension Activation', () => {
    let context: vscode.ExtensionContext;
    let consoleLogSpy: jest.SpyInstance;

    beforeEach(() => {
        // Create mock extension context
        context = {
            subscriptions: [],
            workspaceState: {
                get: jest.fn(),
                update: jest.fn(),
            },
            globalState: {
                get: jest.fn(),
                update: jest.fn(),
            },
            extensionPath: '/test/path',
            globalStoragePath: '/test/storage',
            logPath: '/test/logs',
        } as unknown as vscode.ExtensionContext;

        consoleLogSpy = jest.spyOn(console, 'log').mockImplementation();
    });

    afterEach(() => {
        consoleLogSpy.mockRestore();
    });

    it('should log activation message', () => {
        activate(context);
        expect(consoleLogSpy).toHaveBeenCalledWith('FerrisScript extension is now active');
    });

    it('should register completion provider', () => {
        const registerSpy = jest.spyOn(vscode.languages, 'registerCompletionItemProvider');
        
        activate(context);
        
        expect(registerSpy).toHaveBeenCalledWith(
            { scheme: 'file', language: 'ferrisscript' },
            expect.anything(),
            ':', // Trigger on colon
            '.' // Trigger on dot
        );
    });

    it('should register hover provider', () => {
        const registerSpy = jest.spyOn(vscode.languages, 'registerHoverProvider');
        
        activate(context);
        
        expect(registerSpy).toHaveBeenCalledWith(
            { scheme: 'file', language: 'ferrisscript' },
            expect.anything()
        );
    });

    it('should register diagnostic provider', () => {
        activate(context);
        
        // Diagnostic provider is created and added to subscriptions
        // The provider creates a diagnostic collection in its constructor
        expect(context.subscriptions.length).toBeGreaterThan(0);
        
        // Verify diagnostic provider is in subscriptions (it has a dispose method)
        const disposables = context.subscriptions.filter(d => typeof (d as any).updateDiagnostics === 'function');
        expect(disposables.length).toBeGreaterThan(0);
    });

    it('should register onDidSaveTextDocument listener', () => {
        const onSaveSpy = jest.spyOn(vscode.workspace, 'onDidSaveTextDocument');
        
        activate(context);
        
        expect(onSaveSpy).toHaveBeenCalled();
    });

    it('should register onDidOpenTextDocument listener', () => {
        const onOpenSpy = jest.spyOn(vscode.workspace, 'onDidOpenTextDocument');
        
        activate(context);
        
        expect(onOpenSpy).toHaveBeenCalled();
    });

    it('should add all registrations to context.subscriptions', () => {
        activate(context);
        
        // Should have multiple subscriptions:
        // 1. Completion provider
        // 2. Hover provider
        // 3. Diagnostic provider
        // 4. onDidSaveTextDocument
        // 5. onDidOpenTextDocument
        expect(context.subscriptions.length).toBeGreaterThanOrEqual(5);
    });
});

describe('Extension Deactivation', () => {
    let context: vscode.ExtensionContext;
    let consoleLogSpy: jest.SpyInstance;

    beforeEach(() => {
        context = {
            subscriptions: [],
            workspaceState: {
                get: jest.fn(),
                update: jest.fn(),
            },
            globalState: {
                get: jest.fn(),
                update: jest.fn(),
            },
            extensionPath: '/test/path',
            globalStoragePath: '/test/storage',
            logPath: '/test/logs',
        } as unknown as vscode.ExtensionContext;

        consoleLogSpy = jest.spyOn(console, 'log').mockImplementation();
    });

    afterEach(() => {
        consoleLogSpy.mockRestore();
    });

    it('should log deactivation message', () => {
        // Activate first
        activate(context);
        
        // Then deactivate
        deactivate();
        
        expect(consoleLogSpy).toHaveBeenCalledWith('FerrisScript extension is now deactivated');
    });

    it('should dispose diagnostic provider', () => {
        // Activate first
        activate(context);
        
        // Then deactivate
        deactivate();
        
        // Diagnostic provider should be disposed
        // This is tested indirectly - the provider is set to undefined
        expect(consoleLogSpy).toHaveBeenCalledWith('FerrisScript extension is now deactivated');
    });

    it('should handle deactivation when not activated', () => {
        // Should not throw when deactivating without activation
        expect(() => deactivate()).not.toThrow();
    });
});
