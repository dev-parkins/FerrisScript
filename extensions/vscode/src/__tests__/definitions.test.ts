/**
 * Tests for shared language feature definitions
 */

import {
    KEYWORDS,
    TYPES,
    FUNCTIONS,
    getKeyword,
    getType,
    getFunction,
    KeywordFeature,
    TypeFeature,
    FunctionFeature,
} from '../utils/definitions';

describe('Language Feature Definitions', () => {
    describe('KEYWORDS', () => {
        it('should contain all expected keywords', () => {
            const keywordNames = KEYWORDS.map(k => k.name);
            expect(keywordNames).toContain('fn');
            expect(keywordNames).toContain('let');
            expect(keywordNames).toContain('mut');
            expect(keywordNames).toContain('if');
            expect(keywordNames).toContain('else');
            expect(keywordNames).toContain('while');
            expect(keywordNames).toContain('return');
            expect(keywordNames).toContain('true');
            expect(keywordNames).toContain('false');
        });

        it('should have correct statement level flags', () => {
            const fn = KEYWORDS.find(k => k.name === 'fn');
            expect(fn?.statementLevel).toBe(true);

            const mut = KEYWORDS.find(k => k.name === 'mut');
            expect(mut?.statementLevel).toBe(false);

            const ifKw = KEYWORDS.find(k => k.name === 'if');
            expect(ifKw?.statementLevel).toBe(true);
        });

        it('should have all required fields', () => {
            KEYWORDS.forEach(keyword => {
                expect(keyword.name).toBeDefined();
                expect(keyword.description).toBeDefined();
                expect(keyword.example).toBeDefined();
                expect(keyword.category).toBe('keyword');
                expect(keyword.insertText).toBeDefined();
                expect(typeof keyword.statementLevel).toBe('boolean');
            });
        });

        it('should have non-empty documentation', () => {
            KEYWORDS.forEach(keyword => {
                expect(keyword.description.length).toBeGreaterThan(0);
                expect(keyword.example.length).toBeGreaterThan(0);
            });
        });
    });

    describe('TYPES', () => {
        it('should contain all expected types', () => {
            const typeNames = TYPES.map(t => t.name);
            expect(typeNames).toContain('i32');
            expect(typeNames).toContain('f32');
            expect(typeNames).toContain('bool');
            expect(typeNames).toContain('String');
            expect(typeNames).toContain('void');
            expect(typeNames).toContain('Vector2');
            expect(typeNames).toContain('Node');
        });

        it('should correctly mark builtin types', () => {
            const i32 = TYPES.find(t => t.name === 'i32');
            expect(i32?.isBuiltin).toBe(true);

            const Vector2 = TYPES.find(t => t.name === 'Vector2');
            expect(Vector2?.isBuiltin).toBe(false);

            const Node = TYPES.find(t => t.name === 'Node');
            expect(Node?.isBuiltin).toBe(false);
        });

        it('should have all required fields', () => {
            TYPES.forEach(type => {
                expect(type.name).toBeDefined();
                expect(type.description).toBeDefined();
                expect(type.example).toBeDefined();
                expect(type.category).toBe('type');
                expect(typeof type.isBuiltin).toBe('boolean');
            });
        });
    });

    describe('FUNCTIONS', () => {
        it('should contain expected functions', () => {
            const functionNames = FUNCTIONS.map(f => f.name);
            expect(functionNames).toContain('print');
        });

        it('should have all required fields', () => {
            FUNCTIONS.forEach(func => {
                expect(func.name).toBeDefined();
                expect(func.description).toBeDefined();
                expect(func.example).toBeDefined();
                expect(func.category).toBe('function');
                expect(func.signature).toBeDefined();
                expect(func.returnType).toBeDefined();
            });
        });

        it('should have correct print function signature', () => {
            const print = FUNCTIONS.find(f => f.name === 'print');
            expect(print?.signature).toBe('print(message: String)');
            expect(print?.returnType).toBe('void');
        });
    });

    describe('getKeyword', () => {
        it('should return keyword by name', () => {
            const fn = getKeyword('fn');
            expect(fn).toBeDefined();
            expect(fn?.name).toBe('fn');
        });

        it('should return undefined for non-existent keyword', () => {
            const result = getKeyword('nonexistent');
            expect(result).toBeUndefined();
        });

        it('should be case-sensitive', () => {
            const result = getKeyword('FN');
            expect(result).toBeUndefined();
        });
    });

    describe('getType', () => {
        it('should return type by name', () => {
            const i32 = getType('i32');
            expect(i32).toBeDefined();
            expect(i32?.name).toBe('i32');
        });

        it('should return undefined for non-existent type', () => {
            const result = getType('nonexistent');
            expect(result).toBeUndefined();
        });

        it('should find both builtin and Godot types', () => {
            const i32 = getType('i32');
            expect(i32?.isBuiltin).toBe(true);

            const Vector2 = getType('Vector2');
            expect(Vector2?.isBuiltin).toBe(false);
        });
    });

    describe('getFunction', () => {
        it('should return function by name', () => {
            const print = getFunction('print');
            expect(print).toBeDefined();
            expect(print?.name).toBe('print');
        });

        it('should return undefined for non-existent function', () => {
            const result = getFunction('nonexistent');
            expect(result).toBeUndefined();
        });
    });

    describe('Data Consistency', () => {
        it('should have unique keyword names', () => {
            const names = KEYWORDS.map(k => k.name);
            const uniqueNames = new Set(names);
            expect(names.length).toBe(uniqueNames.size);
        });

        it('should have unique type names', () => {
            const names = TYPES.map(t => t.name);
            const uniqueNames = new Set(names);
            expect(names.length).toBe(uniqueNames.size);
        });

        it('should have unique function names', () => {
            const names = FUNCTIONS.map(f => f.name);
            const uniqueNames = new Set(names);
            expect(names.length).toBe(uniqueNames.size);
        });
    });
});
