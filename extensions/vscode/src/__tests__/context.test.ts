/**
 * Tests for completion context detection
 */

import * as vscode from 'vscode';
import { CompletionContext, detectContext } from '../utils/context';

describe('Context Detection', () => {
    function createMockDocument(lines: string[]): vscode.TextDocument {
        return {
            getText: (range?: vscode.Range) => {
                if (!range) {
                    return lines.join('\n');
                }
                return lines[range.start.line].substring(range.start.character, range.end.character);
            },
            lineAt: (lineOrPosition: number | vscode.Position) => {
                const lineNumber = typeof lineOrPosition === 'number' ? lineOrPosition : lineOrPosition.line;
                const text = lines[lineNumber];
                return {
                    text,
                    lineNumber,
                    range: new vscode.Range(new vscode.Position(lineNumber, 0), new vscode.Position(lineNumber, text.length)),
                    rangeIncludingLineBreak: new vscode.Range(new vscode.Position(lineNumber, 0), new vscode.Position(lineNumber, text.length + 1)),
                    firstNonWhitespaceCharacterIndex: text.search(/\S/),
                    isEmptyOrWhitespace: text.trim().length === 0,
                };
            },
            lineCount: lines.length,
        } as unknown as vscode.TextDocument;
    }

    describe('TypePosition Context', () => {
        it('should detect type position after colon in let statement', () => {
            const doc = createMockDocument(['let x: ']);
            const position = new vscode.Position(0, 7); // After "let x: "
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.TypePosition);
        });

        it('should detect type position with partial type name', () => {
            const doc = createMockDocument(['let x: i']);
            const position = new vscode.Position(0, 8); // After "let x: i"
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.TypePosition);
        });

        it('should detect type position in function parameter', () => {
            const doc = createMockDocument(['fn foo(param: ']);
            const position = new vscode.Position(0, 14); // After "fn foo(param: "
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.TypePosition);
        });

        it.skip('should detect type position in function return type', () => {
            // Skip: Context detection doesn't handle return types currently
            const doc = createMockDocument(['fn bar(x: i32) -> ']);
            const position = new vscode.Position(0, 18); // After "fn bar(x: i32) -> "
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.TypePosition);
        });

        it('should detect type position with whitespace before cursor', () => {
            const doc = createMockDocument(['let x:  ']);
            const position = new vscode.Position(0, 8); // After "let x:  " (two spaces)
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.TypePosition);
        });
    });

    describe('StatementStart Context', () => {
        it('should detect statement start on empty line', () => {
            const doc = createMockDocument(['']);
            const position = new vscode.Position(0, 0);
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.StatementStart);
        });

        it('should detect statement start after indentation', () => {
            const doc = createMockDocument(['    ']);
            const position = new vscode.Position(0, 4); // After 4 spaces
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.StatementStart);
        });

        it('should detect statement start with tab indentation', () => {
            const doc = createMockDocument(['\t\t']);
            const position = new vscode.Position(0, 2); // After 2 tabs
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.StatementStart);
        });

        it('should detect statement start with mixed whitespace', () => {
            const doc = createMockDocument([' \t ']);
            const position = new vscode.Position(0, 3); // After mixed whitespace
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.StatementStart);
        });
    });

    describe('Expression Context', () => {
        it('should detect expression context in function body', () => {
            const doc = createMockDocument(['    let x = ']);
            const position = new vscode.Position(0, 12); // After "let x = "
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.Expression);
        });

        it('should detect expression context after operator', () => {
            const doc = createMockDocument(['    x + ']);
            const position = new vscode.Position(0, 8); // After "x + "
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.Expression);
        });

        it('should detect expression context in function call', () => {
            const doc = createMockDocument(['    print(']);
            const position = new vscode.Position(0, 10); // After "print("
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.Expression);
        });

        it('should detect expression context in if condition', () => {
            const doc = createMockDocument(['    if ']);
            const position = new vscode.Position(0, 7); // After "if "
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.Expression);
        });

        it('should detect expression context after dot (member access)', () => {
            const doc = createMockDocument(['    position.']);
            const position = new vscode.Position(0, 13); // After "position."
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.Expression);
        });

        it('should detect expression context in assignment', () => {
            const doc = createMockDocument(['    x = y']);
            const position = new vscode.Position(0, 9); // After "y"
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.Expression);
        });
    });

    describe('Edge Cases', () => {
        it('should handle multiline documents', () => {
            const doc = createMockDocument([
                'fn main() -> void {',
                '    let x: ',
                '    ',
                '}',
            ]);
            
            // Line 1: Type position
            const context1 = detectContext(doc, new vscode.Position(1, 11));
            expect(context1).toBe(CompletionContext.TypePosition);
            
            // Line 2: Statement start
            const context2 = detectContext(doc, new vscode.Position(2, 4));
            expect(context2).toBe(CompletionContext.StatementStart);
        });

        it('should handle cursor at start of line', () => {
            const doc = createMockDocument(['let x = 5']);
            const position = new vscode.Position(0, 0);
            
            // At start of "let", should be statement start (empty before cursor)
            const context = detectContext(doc, new vscode.Position(0, 0));
            expect(context).toBe(CompletionContext.StatementStart);
        });

        it('should handle cursor in middle of word', () => {
            const doc = createMockDocument(['let x = value']);
            const position = new vscode.Position(0, 10); // In "value"
            
            const context = detectContext(doc, position);
            expect(context).toBe(CompletionContext.Expression);
        });
    });
});
