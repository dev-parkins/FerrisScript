/**
 * Comprehensive tests for completion providers
 */

import * as vscode from 'vscode';
import { FerrisScriptCompletionProvider } from '../completion/provider';
import { getKeywordCompletions } from '../completion/keywords';
import { getTypeCompletions } from '../completion/types';
import { getFunctionCompletions } from '../completion/functions';

// Mock the utils module
jest.mock('../utils/context', () => ({
    detectContext: jest.fn(),
    CompletionContext: {
        TypePosition: 0,
        StatementStart: 1,
        Expression: 2,
        Unknown: 3,
    },
}));

import { detectContext, CompletionContext } from '../utils/context';

describe('FerrisScriptCompletionProvider', () => {
    let provider: FerrisScriptCompletionProvider;
    let mockDocument: vscode.TextDocument;
    let mockPosition: vscode.Position;
    let mockToken: vscode.CancellationToken;
    let mockContext: vscode.CompletionContext;

    beforeEach(() => {
        provider = new FerrisScriptCompletionProvider();
        mockDocument = {} as vscode.TextDocument;
        mockPosition = new vscode.Position(0, 0);
        mockToken = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose: () => {} }) };
        mockContext = { triggerKind: vscode.CompletionTriggerKind.Invoke, triggerCharacter: undefined };
    });

    describe('provideCompletionItems', () => {
        it('should provide type completions in type position', () => {
            (detectContext as jest.Mock).mockReturnValue(CompletionContext.TypePosition);

            const result = provider.provideCompletionItems(mockDocument, mockPosition, mockToken, mockContext);

            expect(result).toBeDefined();
            expect(Array.isArray(result)).toBe(true);
            // Should contain types like i32, f32, bool, String, etc.
            if (Array.isArray(result)) {
                const labels = result.map(item => typeof item.label === 'string' ? item.label : item.label.label);
                expect(labels).toContain('i32');
                expect(labels).toContain('f32');
                expect(labels).toContain('bool');
            }
        });

        it('should provide statement-level keywords at statement start', () => {
            (detectContext as jest.Mock).mockReturnValue(CompletionContext.StatementStart);

            const result = provider.provideCompletionItems(mockDocument, mockPosition, mockToken, mockContext);

            expect(result).toBeDefined();
            expect(Array.isArray(result)).toBe(true);
            // Should contain statement keywords: fn, let, if, while, return
            if (Array.isArray(result)) {
                const labels = result.map(item => typeof item.label === 'string' ? item.label : item.label.label);
                expect(labels).toContain('fn');
                expect(labels).toContain('let');
                expect(labels).toContain('if');
                expect(labels).toContain('while');
            }
        });

        it('should provide expression keywords and functions in expression context', () => {
            (detectContext as jest.Mock).mockReturnValue(CompletionContext.Expression);

            const result = provider.provideCompletionItems(mockDocument, mockPosition, mockToken, mockContext);

            expect(result).toBeDefined();
            expect(Array.isArray(result)).toBe(true);
            if (Array.isArray(result)) {
                const labels = result.map(item => typeof item.label === 'string' ? item.label : item.label.label);
                // Should NOT contain statement-only keywords
                expect(labels).not.toContain('fn');
                expect(labels).not.toContain('let');
                // Should contain expression keywords
                expect(labels).toContain('true');
                expect(labels).toContain('false');
                // Should contain functions
                expect(labels).toContain('print');
            }
        });

        it('should handle unknown context', () => {
            (detectContext as jest.Mock).mockReturnValue(CompletionContext.Unknown);

            const result = provider.provideCompletionItems(mockDocument, mockPosition, mockToken, mockContext);

            expect(result).toBeDefined();
            expect(Array.isArray(result)).toBe(true);
        });
    });
});

describe('getKeywordCompletions', () => {
    it('should return all keywords when statementLevelOnly is false', () => {
        const result = getKeywordCompletions(false);

        expect(result.length).toBeGreaterThan(0);
        const labels = result.map(item => typeof item.label === 'string' ? item.label : item.label.label);
        expect(labels).toContain('fn');
        expect(labels).toContain('let');
        expect(labels).toContain('true');
        expect(labels).toContain('false');
    });

    it('should return only statement-level keywords when statementLevelOnly is true', () => {
        const result = getKeywordCompletions(true);

        expect(result.length).toBeGreaterThan(0);
        const labels = result.map(item => typeof item.label === 'string' ? item.label : item.label.label);
        expect(labels).toContain('fn');
        expect(labels).toContain('let');
        expect(labels).not.toContain('true');
        expect(labels).not.toContain('false');
    });

    it('should return CompletionItems with correct kind', () => {
        const result = getKeywordCompletions(false);

        result.forEach(item => {
            expect(item.kind).toBe(vscode.CompletionItemKind.Keyword);
        });
    });

    it('should have documentation as MarkdownString', () => {
        const result = getKeywordCompletions(false);

        result.forEach(item => {
            expect(item.documentation).toBeInstanceOf(vscode.MarkdownString);
        });
    });
});

describe('getTypeCompletions', () => {
    it('should return all type completions', () => {
        const result = getTypeCompletions();

        expect(result.length).toBeGreaterThan(0);
        const labels = result.map(item => typeof item.label === 'string' ? item.label : item.label.label);
        expect(labels).toContain('i32');
        expect(labels).toContain('f32');
        expect(labels).toContain('bool');
        expect(labels).toContain('String');
        expect(labels).toContain('Vector2');
    });

    it('should return CompletionItems with correct kind', () => {
        const result = getTypeCompletions();

        result.forEach(item => {
            expect(item.kind).toBe(vscode.CompletionItemKind.Class);
        });
    });

    it('should have documentation', () => {
        const result = getTypeCompletions();

        result.forEach(item => {
            expect(item.documentation).toBeDefined();
        });
    });
});

describe('getFunctionCompletions', () => {
    it('should return all function completions', () => {
        const result = getFunctionCompletions();

        expect(result.length).toBeGreaterThan(0);
        const labels = result.map(item => typeof item.label === 'string' ? item.label : item.label.label);
        expect(labels).toContain('print');
    });

    it('should return CompletionItems with correct kind', () => {
        const result = getFunctionCompletions();

        result.forEach(item => {
            expect(item.kind).toBe(vscode.CompletionItemKind.Function);
        });
    });

    it('should have snippets for function calls', () => {
        const result = getFunctionCompletions();

        result.forEach(item => {
            expect(item.insertText).toBeDefined();
        });
    });
});
