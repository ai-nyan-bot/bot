import {describe, expect, test} from '@jest/globals';
import {injectId} from "./id.ts";


describe('injectId', () => {
    test('should add an id to a simple object', () => {
        const input = {name: 'Alice', age: 30};
        const output = injectId(input, () => "some-id");
        expect(output).toEqual({
            id: 'some-id',
            name: 'Alice',
            age: 30,
        });
    });

    test('should add an id to nested objects', () => {
        const input = {
            name: 'Parent',
            child: {name: 'Child'},
        };
        const output = injectId(input, () => "some-id");

        expect(output).toEqual({
            id: 'some-id',
            name: 'Parent',
            child: {
                id: 'some-id',
                name: 'Child',
            },
        });
    });

    test('should handle arrays correctly', () => {
        const input = {
            items: [{name: 'Item 1'}, {name: 'Item 2'}],
        };
        const output = injectId(input, () => "some-id");

        expect(output).toEqual({
            id: 'some-id',
            items: [
                {id: 'some-id', name: 'Item 1'},
                {id: 'some-id', name: 'Item 2'},
            ],
        });
    });
});