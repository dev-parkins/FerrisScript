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
exports.getKeywordCompletions = getKeywordCompletions;
const vscode = __importStar(require("vscode"));
/**
 * FerrisScript keyword definitions
 */
const KEYWORDS = [
    {
        label: 'fn',
        detail: 'function declaration',
        documentation: 'Declares a new function.\n\n**Example:**\n```ferrisscript\nfn add(a: i32, b: i32) -> i32 {\n    return a + b;\n}\n```',
        insertText: 'fn ${1:name}(${2:params}) {\n\t$0\n}',
        statementLevel: true
    },
    {
        label: 'let',
        detail: 'immutable variable declaration',
        documentation: 'Declares an immutable variable.\n\n**Example:**\n```ferrisscript\nlet x: i32 = 42;\nlet message: String = "Hello";\n```',
        insertText: 'let ${1:name}: ${2:i32} = $0;',
        statementLevel: true
    },
    {
        label: 'mut',
        detail: 'mutable variable modifier',
        documentation: 'Makes a variable mutable.\n\n**Example:**\n```ferrisscript\nlet mut counter: i32 = 0;\ncounter += 1;\n```',
        insertText: 'mut',
        statementLevel: false
    },
    {
        label: 'if',
        detail: 'conditional statement',
        documentation: 'Executes code conditionally.\n\n**Example:**\n```ferrisscript\nif x > 0 {\n    print("positive");\n}\n```',
        insertText: 'if ${1:condition} {\n\t$0\n}',
        statementLevel: true
    },
    {
        label: 'else',
        detail: 'else clause',
        documentation: 'Alternative branch for if statement.\n\n**Example:**\n```ferrisscript\nif x > 0 {\n    print("positive");\n} else {\n    print("not positive");\n}\n```',
        insertText: 'else {\n\t$0\n}',
        statementLevel: false
    },
    {
        label: 'while',
        detail: 'loop statement',
        documentation: 'Repeats code while condition is true.\n\n**Example:**\n```ferrisscript\nwhile counter < 10 {\n    counter += 1;\n}\n```',
        insertText: 'while ${1:condition} {\n\t$0\n}',
        statementLevel: true
    },
    {
        label: 'return',
        detail: 'return statement',
        documentation: 'Returns a value from a function.\n\n**Example:**\n```ferrisscript\nfn get_value() -> i32 {\n    return 42;\n}\n```',
        insertText: 'return $0;',
        statementLevel: true
    },
    {
        label: 'true',
        detail: 'boolean literal',
        documentation: 'Boolean true value.\n\n**Example:**\n```ferrisscript\nlet is_active: bool = true;\n```',
        insertText: 'true',
        statementLevel: false
    },
    {
        label: 'false',
        detail: 'boolean literal',
        documentation: 'Boolean false value.\n\n**Example:**\n```ferrisscript\nlet is_active: bool = false;\n```',
        insertText: 'false',
        statementLevel: false
    }
];
/**
 * Get keyword completion items
 * @param statementLevelOnly If true, only return keywords valid at statement start
 * @returns Array of completion items for keywords
 */
function getKeywordCompletions(statementLevelOnly) {
    const filtered = statementLevelOnly
        ? KEYWORDS.filter(k => k.statementLevel)
        : KEYWORDS;
    return filtered.map(kw => {
        const item = new vscode.CompletionItem(kw.label, vscode.CompletionItemKind.Keyword);
        item.detail = kw.detail;
        item.documentation = new vscode.MarkdownString(kw.documentation);
        item.insertText = new vscode.SnippetString(kw.insertText);
        return item;
    });
}
//# sourceMappingURL=keywords.js.map