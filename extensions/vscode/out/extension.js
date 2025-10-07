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
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
const provider_1 = require("./completion/provider");
/**
 * Extension activation entry point
 * Called when the extension is activated (on .ferris file open)
 */
function activate(context) {
    console.log('FerrisScript extension is now active');
    // Register completion provider
    const completionProvider = new provider_1.FerrisScriptCompletionProvider();
    const completionDisposable = vscode.languages.registerCompletionItemProvider({ scheme: 'file', language: 'ferrisscript' }, completionProvider, ':', // Trigger on colon for type hints
    '.' // Trigger on dot for member access (future)
    );
    context.subscriptions.push(completionDisposable);
}
/**
 * Extension deactivation cleanup
 * Called when the extension is deactivated
 */
function deactivate() {
    console.log('FerrisScript extension is now deactivated');
}
//# sourceMappingURL=extension.js.map