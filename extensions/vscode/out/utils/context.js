"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.CompletionContext = void 0;
exports.detectContext = detectContext;
/**
 * Completion context types for FerrisScript
 */
var CompletionContext;
(function (CompletionContext) {
    /** After ':' in type annotations (let x: |, fn foo() -> |) */
    CompletionContext[CompletionContext["TypePosition"] = 0] = "TypePosition";
    /** At statement start (beginning of line after whitespace) */
    CompletionContext[CompletionContext["StatementStart"] = 1] = "StatementStart";
    /** Inside expression context (most places) */
    CompletionContext[CompletionContext["Expression"] = 2] = "Expression";
    /** Unable to determine context */
    CompletionContext[CompletionContext["Unknown"] = 3] = "Unknown";
})(CompletionContext || (exports.CompletionContext = CompletionContext = {}));
/**
 * Detect completion context based on cursor position and surrounding text
 * @param document The text document
 * @param position The cursor position
 * @returns The detected completion context
 */
function detectContext(document, position) {
    const line = document.lineAt(position.line).text;
    const beforeCursor = line.substring(0, position.character);
    // Type position: "let x: |" or "fn foo(param: |" or "fn bar() -> |"
    // Match colon followed by optional whitespace at end of string
    if (/:\s*$/.test(beforeCursor)) {
        return CompletionContext.TypePosition;
    }
    // Statement start: "    |" (only whitespace before cursor)
    // This catches the start of statements where we want keywords like let, fn, if, while
    if (/^\s*$/.test(beforeCursor)) {
        return CompletionContext.StatementStart;
    }
    // Default to expression context
    // This includes most other positions: inside function bodies, after operators, etc.
    return CompletionContext.Expression;
}
//# sourceMappingURL=context.js.map