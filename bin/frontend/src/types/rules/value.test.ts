import {compareDurations, TimeUnit, ValueDuration, ValueType} from "@types";

describe('compareDurations(left,right)', () => {

    test("should return 0 when durations are equal", () => {
        const left: ValueDuration = {type: ValueType.DURATION, value: 1, unit: TimeUnit.HOUR};
        const right: ValueDuration = {type: ValueType.DURATION, value: 60, unit: TimeUnit.MINUTE};
        expect(compareDurations(left, right)).toBe(0);
    });

    test("should return a positive number when left duration is greater", () => {
        const left: ValueDuration = {type: ValueType.DURATION, value: 2, unit: TimeUnit.HOUR};
        const right: ValueDuration = {type: ValueType.DURATION, value: 30, unit: TimeUnit.MINUTE};
        expect(compareDurations(left, right)).toBeGreaterThan(0);
    });

    test("should return a negative number when left duration is smaller", () => {
        const left: ValueDuration = {type: ValueType.DURATION, value: 10, unit: TimeUnit.SECOND};
        const right: ValueDuration = {type: ValueType.DURATION, value: 1, unit: TimeUnit.MINUTE};
        expect(compareDurations(left, right)).toBeLessThan(0);
    });

    test("should handle edge case of zero duration", () => {
        const left: ValueDuration = {type: ValueType.DURATION, value: 0, unit: TimeUnit.SECOND};
        const right: ValueDuration = {type: ValueType.DURATION, value: 0, unit: TimeUnit.HOUR};
        expect(compareDurations(left, right)).toBe(0);
    });

})