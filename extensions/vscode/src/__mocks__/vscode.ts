/**
 * Mock implementation of VS Code API for unit testing
 * Provides minimal implementation of VS Code types and functions
 */

export enum CompletionItemKind {
  Text = 0,
  Method = 1,
  Function = 2,
  Constructor = 3,
  Field = 4,
  Variable = 5,
  Class = 6,
  Interface = 7,
  Module = 8,
  Property = 9,
  Unit = 10,
  Value = 11,
  Enum = 12,
  Keyword = 13,
  Snippet = 14,
  Color = 15,
  File = 16,
  Reference = 17,
  Folder = 18,
  EnumMember = 19,
  Constant = 20,
  Struct = 21,
  Event = 22,
  Operator = 23,
  TypeParameter = 24,
}

export enum CompletionItemTag {
  Deprecated = 1,
}

export enum DiagnosticSeverity {
  Error = 0,
  Warning = 1,
  Information = 2,
  Hint = 3,
}

export class Position {
  constructor(public line: number, public character: number) {}
  
  isAfter(other: Position): boolean {
    return this.line > other.line || (this.line === other.line && this.character > other.character);
  }
  
  isBefore(other: Position): boolean {
    return this.line < other.line || (this.line === other.line && this.character < other.character);
  }
  
  isEqual(other: Position): boolean {
    return this.line === other.line && this.character === other.character;
  }
}

export class Range {
  public start: Position;
  public end: Position;
  
  constructor(startLine: number | Position, startChar: number | Position, endLine?: number, endChar?: number) {
    // Support both Range(Position, Position) and Range(line, char, line, char)
    if (typeof startLine === 'number' && typeof startChar === 'number' && 
        typeof endLine === 'number' && typeof endChar === 'number') {
      this.start = new Position(startLine, startChar);
      this.end = new Position(endLine, endChar);
    } else if (startLine instanceof Position && startChar instanceof Position) {
      this.start = startLine;
      this.end = startChar;
    } else {
      throw new Error('Invalid Range constructor arguments');
    }
  }
  
  contains(positionOrRange: Position | Range): boolean {
    if (positionOrRange instanceof Position) {
      return !positionOrRange.isBefore(this.start) && !positionOrRange.isAfter(this.end);
    }
    return this.contains(positionOrRange.start) && this.contains(positionOrRange.end);
  }
}

export class Uri {
  public fsPath: string;
  
  private constructor(public scheme: string, public path: string) {
    this.fsPath = path;
  }
  
  static file(path: string): Uri {
    return new Uri('file', path);
  }
  
  static parse(value: string): Uri {
    const match = value.match(/^(\w+):(.+)$/);
    return new Uri(match?.[1] || 'file', match?.[2] || value);
  }
  
  toString(): string {
    return `${this.scheme}:${this.path}`;
  }
}

export class Diagnostic {
  constructor(
    public range: Range,
    public message: string,
    public severity: DiagnosticSeverity = DiagnosticSeverity.Error
  ) {}
}

export class CompletionItem {
  constructor(public label: string, public kind?: CompletionItemKind) {}
  detail?: string;
  documentation?: string | MarkdownString;
  insertText?: string;
  filterText?: string;
  sortText?: string;
  tags?: CompletionItemTag[];
}

export class CompletionList {
  constructor(public items: CompletionItem[], public isIncomplete?: boolean) {}
}

export class MarkdownString {
  constructor(public value: string = '', public supportThemeIcons?: boolean) {}
  
  appendMarkdown(value: string): MarkdownString {
    this.value += value;
    return this;
  }
  
  appendCodeblock(value: string, language?: string): MarkdownString {
    this.value += `\n\`\`\`${language || ''}\n${value}\n\`\`\`\n`;
    return this;
  }
}

export class SnippetString {
  constructor(public value: string = '') {}
  
  appendText(value: string): SnippetString {
    this.value += value;
    return this;
  }
  
  appendTabstop(number: number = 0): SnippetString {
    this.value += `$${number}`;
    return this;
  }
  
  appendPlaceholder(value: string, number: number = 0): SnippetString {
    this.value += `\${${number}:${value}}`;
    return this;
  }
}

export class Hover {
  constructor(public contents: MarkdownString | MarkdownString[], public range?: Range) {}
}

export interface TextDocument {
  uri: Uri;
  fileName: string;
  languageId: string;
  version: number;
  getText(range?: Range): string;
  lineAt(lineOrPosition: number | Position): TextLine;
  offsetAt(position: Position): number;
  positionAt(offset: number): Position;
  lineCount: number;
}

export interface TextLine {
  lineNumber: number;
  text: string;
  range: Range;
  rangeIncludingLineBreak: Range;
  firstNonWhitespaceCharacterIndex: number;
  isEmptyOrWhitespace: boolean;
}

export class CancellationTokenSource {
  token: CancellationToken = {
    isCancellationRequested: false,
    onCancellationRequested: () => ({ dispose: () => {} }),
  };
  
  cancel(): void {
    this.token.isCancellationRequested = true;
  }
  
  dispose(): void {
    // No-op for mock
  }
}

export interface CancellationToken {
  isCancellationRequested: boolean;
  onCancellationRequested: (listener: () => void) => { dispose(): void };
}

export interface CompletionContext {
  triggerKind: CompletionTriggerKind;
  triggerCharacter?: string;
}

export enum CompletionTriggerKind {
  Invoke = 0,
  TriggerCharacter = 1,
  TriggerForIncompleteCompletions = 2,
}

export interface ExtensionContext {
  subscriptions: { dispose(): void }[];
  workspaceState: { get<T>(key: string): T | undefined; update(key: string, value: unknown): Thenable<void> };
  globalState: { get<T>(key: string): T | undefined; update(key: string, value: unknown): Thenable<void> };
  extensionPath: string;
  storagePath?: string;
  globalStoragePath: string;
  logPath: string;
}

export namespace languages {
  export function registerCompletionItemProvider(
    selector: string | { scheme: string; language: string },
    provider: any,
    ...triggerCharacters: string[]
  ): { dispose(): void } {
    return { dispose: () => {} };
  }
  
  export function registerHoverProvider(
    selector: string | { scheme: string; language: string },
    provider: any
  ): { dispose(): void } {
    return { dispose: () => {} };
  }
  
  export function createDiagnosticCollection(name?: string): DiagnosticCollection {
    return new MockDiagnosticCollection();
  }
}

export interface DiagnosticCollection {
  name: string;
  set(uri: Uri, diagnostics: Diagnostic[] | undefined): void;
  set(entries: [Uri, Diagnostic[] | undefined][]): void;
  delete(uri: Uri): void;
  clear(): void;
  forEach(callback: (uri: Uri, diagnostics: Diagnostic[], collection: DiagnosticCollection) => void): void;
  get(uri: Uri): Diagnostic[] | undefined;
  has(uri: Uri): boolean;
  dispose(): void;
}

class MockDiagnosticCollection implements DiagnosticCollection {
  name = 'test';
  private diagnostics = new Map<string, Diagnostic[]>();
  
  set(uriOrEntries: Uri | [Uri, Diagnostic[] | undefined][], diagnostics?: Diagnostic[]): void {
    if (Array.isArray(uriOrEntries)) {
      uriOrEntries.forEach(([uri, diags]) => {
        if (diags) {
          this.diagnostics.set(uri.toString(), diags);
        } else {
          this.diagnostics.delete(uri.toString());
        }
      });
    } else if (diagnostics) {
      this.diagnostics.set(uriOrEntries.toString(), diagnostics);
    } else {
      this.diagnostics.delete(uriOrEntries.toString());
    }
  }
  
  delete(uri: Uri): void {
    this.diagnostics.delete(uri.toString());
  }
  
  clear(): void {
    this.diagnostics.clear();
  }
  
  forEach(callback: (uri: Uri, diagnostics: Diagnostic[], collection: DiagnosticCollection) => void): void {
    this.diagnostics.forEach((diags, uriStr) => {
      callback(Uri.parse(uriStr), diags, this);
    });
  }
  
  get(uri: Uri): Diagnostic[] | undefined {
    return this.diagnostics.get(uri.toString());
  }
  
  has(uri: Uri): boolean {
    return this.diagnostics.has(uri.toString());
  }
  
  dispose(): void {
    this.diagnostics.clear();
  }
}

export namespace workspace {
  export function getConfiguration(section?: string): WorkspaceConfiguration {
    return new MockWorkspaceConfiguration();
  }
  
  export const onDidSaveTextDocument = (listener: (e: TextDocument) => void) => ({
    dispose: () => {},
  });
  
  export const onDidOpenTextDocument = (listener: (e: TextDocument) => void) => ({
    dispose: () => {},
  });
  
  export const workspaceFolders: { uri: Uri; name: string }[] | undefined = undefined;
}

class MockWorkspaceConfiguration {
  private config = new Map<string, unknown>();
  
  get<T>(section: string, defaultValue?: T): T | undefined {
    return (this.config.get(section) as T) || defaultValue;
  }
  
  has(section: string): boolean {
    return this.config.has(section);
  }
  
  update(section: string, value: unknown): Thenable<void> {
    this.config.set(section, value);
    return Promise.resolve();
  }
}

export interface WorkspaceConfiguration {
  get<T>(section: string, defaultValue?: T): T | undefined;
  has(section: string): boolean;
  update(section: string, value: unknown): Thenable<void>;
}

export namespace window {
  export const activeTextEditor: { document: TextDocument } | undefined = undefined;
  
  export function showInformationMessage(message: string, ...items: string[]): Thenable<string | undefined> {
    return Promise.resolve(undefined);
  }
  
  export function showWarningMessage(message: string, ...items: string[]): Thenable<string | undefined> {
    return Promise.resolve(undefined);
  }
  
  export function showErrorMessage(message: string, ...items: string[]): Thenable<string | undefined> {
    return Promise.resolve(undefined);
  }
}
