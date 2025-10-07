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
exports.getTypeCompletions = getTypeCompletions;
const vscode = __importStar(require("vscode"));
/**
 * FerrisScript type definitions
 */
const TYPES = [
    {
        label: 'i32',
        detail: '32-bit signed integer',
        documentation: '32-bit signed integer type.\n\n**Range:** -2,147,483,648 to 2,147,483,647\n\n**Example:**\n```ferrisscript\nlet count: i32 = 42;\nlet negative: i32 = -100;\n```'
    },
    {
        label: 'f32',
        detail: '32-bit floating point',
        documentation: '32-bit floating point type.\n\n**Example:**\n```ferrisscript\nlet speed: f32 = 3.14;\nlet delta: f32 = 0.016;\n```'
    },
    {
        label: 'bool',
        detail: 'boolean type',
        documentation: 'Boolean type (true or false).\n\n**Example:**\n```ferrisscript\nlet is_ready: bool = true;\nlet is_jumping: bool = false;\n```'
    },
    {
        label: 'String',
        detail: 'text string type',
        documentation: 'Text string type.\n\n**Example:**\n```ferrisscript\nlet name: String = "Player";\nlet message: String = "Hello, World!";\n```'
    },
    {
        label: 'Vector2',
        detail: 'Godot 2D vector type',
        documentation: 'Godot Vector2 type for 2D coordinates.\n\n**Fields:** `x: f32`, `y: f32`\n\n**Example:**\n```ferrisscript\nlet position: Vector2 = Vector2(10.0, 20.0);\nlet velocity: Vector2 = Vector2(5.0, 0.0);\n```'
    },
    {
        label: 'Node',
        detail: 'Godot node type',
        documentation: 'Godot Node base type.\n\n**Example:**\n```ferrisscript\nlet parent: Node = self.get_parent();\n```'
    },
    {
        label: 'void',
        detail: 'no return value',
        documentation: 'Void type for functions that do not return a value.\n\n**Example:**\n```ferrisscript\nfn initialize() -> void {\n    print("Initialized");\n}\n```'
    }
];
/**
 * Get type completion items
 * @returns Array of completion items for types
 */
function getTypeCompletions() {
    return TYPES.map(type => {
        const item = new vscode.CompletionItem(type.label, vscode.CompletionItemKind.Class);
        item.detail = type.detail;
        item.documentation = new vscode.MarkdownString(type.documentation);
        item.insertText = type.label;
        return item;
    });
}
//# sourceMappingURL=types.js.map