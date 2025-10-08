/**
 * Tests for hover providers
 */

import * as vscode from 'vscode';
import { FerrisScriptHoverProvider } from '../hover/provider';
import { getKeywordHover } from '../hover/keywords';
import { getTypeHover } from '../hover/types';
import { getFunctionHover } from '../hover/functions';

describe('FerrisScriptHoverProvider', () => {
    let provider: FerrisScriptHoverProvider;
    let mockDocument: vscode.TextDocument;
    let mockPosition: vscode.Position;
    let mockToken: vscode.CancellationToken;

    beforeEach(() => {
        provider = new FerrisScriptHoverProvider();
        mockPosition = new vscode.Position(0, 5);
        mockToken = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose: () => {} }) };
    });

    function createMockDocument(lines: string[]): vscode.TextDocument {
        return {
            lineAt: (lineOrPosition: number | vscode.Position) => {
                const lineNumber = typeof lineOrPosition === 'number' ? lineOrPosition : lineOrPosition.line;
                const text = lines[lineNumber];
                return {
                    text,
                    lineNumber,
                    range: new vscode.Range(new vscode.Position(lineNumber, 0), new vscode.Position(lineNumber, text.length)),
                };
            },
            getWordRangeAtPosition: (position: vscode.Position, regex?: RegExp) => {
                const line = lines[position.line];
                const match = line.match(/\w+/g);
                if (match) {
                    let start = 0;
                    for (const word of match) {
                        const wordStart = line.indexOf(word, start);
                        const wordEnd = wordStart + word.length;
                        if (position.character >= wordStart && position.character <= wordEnd) {
                            return new vscode.Range(
                                new vscode.Position(position.line, wordStart),
                                new vscode.Position(position.line, wordEnd)
                            );
                        }
                        start = wordEnd;
                    }
                }
                return undefined;
            },
            getText: (range?: vscode.Range) => {
                if (!range) {
                    return lines.join('\n');
                }
                const line = lines[range.start.line];
                return line.substring(range.start.character, range.end.character);
            },
        } as unknown as vscode.TextDocument;
    }

    describe('provideHover', () => {
        it('should provide hover for keyword', () => {
            mockDocument = createMockDocument(['let x = 5;']);
            mockPosition = new vscode.Position(0, 1); // On "let"

            const result = provider.provideHover(mockDocument, mockPosition, mockToken);

            expect(result).toBeDefined();
        });

        it('should provide hover for type', () => {
            mockDocument = createMockDocument(['let x: i32 = 5;']);
            mockPosition = new vscode.Position(0, 7); // On "i32"

            const result = provider.provideHover(mockDocument, mockPosition, mockToken);

            expect(result).toBeDefined();
        });

        it('should provide hover for function', () => {
            mockDocument = createMockDocument(['print("hello");']);
            mockPosition = new vscode.Position(0, 2); // On "print"

            const result = provider.provideHover(mockDocument, mockPosition, mockToken);

            expect(result).toBeDefined();
        });

        it('should return undefined for unknown word', () => {
            mockDocument = createMockDocument(['unknown_word']);
            mockPosition = new vscode.Position(0, 2);

            const result = provider.provideHover(mockDocument, mockPosition, mockToken);

            expect(result).toBeUndefined();
        });

        it('should return undefined when no word at position', () => {
            mockDocument = createMockDocument(['   ']);
            mockPosition = new vscode.Position(0, 1);

            const result = provider.provideHover(mockDocument, mockPosition, mockToken);

            expect(result).toBeUndefined();
        });
    });
});

describe('getKeywordHover', () => {
    it('should return hover for valid keywords', () => {
        const result = getKeywordHover('let');
        expect(result).toBeInstanceOf(vscode.MarkdownString);
        expect(result?.value).toContain('immutable variable');
    });

    it('should return undefined for invalid keyword', () => {
        const result = getKeywordHover('notakeyword');
        expect(result).toBeUndefined();
    });

    it('should handle all defined keywords', () => {
        const keywords = ['let', 'fn', 'if', 'else', 'while', 'return', 'true', 'false', 'mut'];
        keywords.forEach(kw => {
            const result = getKeywordHover(kw);
            expect(result).toBeInstanceOf(vscode.MarkdownString);
        });
    });
});

describe('getTypeHover', () => {
    it('should return hover for valid types', () => {
        const result = getTypeHover('i32');
        expect(result).toBeInstanceOf(vscode.MarkdownString);
        expect(result?.value).toContain('32-bit');
    });

    it('should return undefined for invalid type', () => {
        const result = getTypeHover('notatype');
        expect(result).toBeUndefined();
    });

    it('should handle all defined types', () => {
        const types = ['i32', 'f32', 'bool', 'String', 'void', 'Vector2', 'Node'];
        types.forEach(type => {
            const result = getTypeHover(type);
            expect(result).toBeInstanceOf(vscode.MarkdownString);
        });
    });
});

describe('getFunctionHover', () => {
    it('should return hover for valid functions', () => {
        const result = getFunctionHover('print');
        expect(result).toBeInstanceOf(vscode.MarkdownString);
        expect(result?.value).toContain('print');
    });

    it('should return undefined for invalid function', () => {
        const result = getFunctionHover('notafunction');
        expect(result).toBeUndefined();
    });
});
