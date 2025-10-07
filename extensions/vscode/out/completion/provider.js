"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FerrisScriptCompletionProvider = void 0;
const keywords_1 = require("./keywords");
const types_1 = require("./types");
const functions_1 = require("./functions");
const context_1 = require("../utils/context");
/**
 * Completion provider for FerrisScript
 * Provides context-aware code completion for keywords, types, and functions
 */
class FerrisScriptCompletionProvider {
    /**
     * Provide completion items based on cursor position and context
     * @param document The text document
     * @param position The cursor position
     * @param token Cancellation token
     * @param context Completion context from VS Code
     * @returns Array of completion items or completion list
     */
    provideCompletionItems(document, position, token, context) {
        // Detect completion context based on cursor position
        const ctx = (0, context_1.detectContext)(document, position);
        // Return appropriate completions based on context
        switch (ctx) {
            case context_1.CompletionContext.TypePosition:
                // After ':' in type annotations - show only types
                return (0, types_1.getTypeCompletions)();
            case context_1.CompletionContext.StatementStart:
                // At statement start - show statement-level keywords only
                return (0, keywords_1.getKeywordCompletions)(true);
            case context_1.CompletionContext.Expression:
                // In expression context - show all keywords and functions
                return [
                    ...(0, keywords_1.getKeywordCompletions)(false),
                    ...(0, functions_1.getFunctionCompletions)()
                ];
            default:
                // Unknown context - show everything
                return [
                    ...(0, keywords_1.getKeywordCompletions)(false),
                    ...(0, types_1.getTypeCompletions)(),
                    ...(0, functions_1.getFunctionCompletions)()
                ];
        }
    }
}
exports.FerrisScriptCompletionProvider = FerrisScriptCompletionProvider;
//# sourceMappingURL=provider.js.map