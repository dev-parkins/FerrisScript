"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.getFunctionCompletions = getFunctionCompletions;
const vscode = __importStar(require("vscode"));
/**
 * FerrisScript built-in function definitions
 */
const FUNCTIONS = [
    {
        label: 'print',
        detail: 'print(message: String) -> void',
        documentation: 'Prints a message to the Godot console.\n\n**Parameters:**\n- `message: String` - The message to print\n\n**Returns:** `void`\n\n**Example:**\n```ferrisscript\nprint("Hello, World!");\nprint("Player position: " + position.x);\n```',
        insertText: 'print($0)'
    }
];
/**
 * Get function completion items
 * @returns Array of completion items for functions
 */
function getFunctionCompletions() {
    return FUNCTIONS.map(func => {
        const item = new vscode.CompletionItem(func.label, vscode.CompletionItemKind.Function);
        item.detail = func.detail;
        item.documentation = new vscode.MarkdownString(func.documentation);
        item.insertText = new vscode.SnippetString(func.insertText);
        return item;
    });
}
//# sourceMappingURL=functions.js.map