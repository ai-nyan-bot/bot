import {render} from "@testing-library/react";
import {RenderText} from "./age";
import {TimeUnit, ValueType} from "@types";

describe("RenderText", () => {

    test("null when minValue and maxValue are missing", () => {
        const {container} = render(<RenderText/>);
        expect(container.firstChild).toBeNull();
    });

    test("warning if minValue is greater than maxValue", () => {
        const {container} = render(<RenderText
            minValue={{
                type: ValueType.DURATION,
                value: 600,
                unit: TimeUnit.SECOND
            }}
            maxValue={{
                type: ValueType.DURATION,
                value: 5,
                unit: TimeUnit.MINUTE
            }}
        />);

        const warningDiv = container.querySelector("div");
        expect(warningDiv).not.toBeNull();
        expect(warningDiv!!.className).toContain("text-yellow-700");
        expect(warningDiv!!.className).toContain("font-bold");

        expect(warningDiv?.querySelector("p:nth-child(1)")?.textContent).toBe("⚠️ The rule will never execute ⚠️");
        expect(warningDiv?.querySelector("p:nth-child(2)")?.textContent).toBe("Minimum age is greater than the maximum age");
    })

    test("minValue == maxValue", () => {
        const {container} = render(<RenderText
            minValue={{
                type: ValueType.DURATION,
                value: 300,
                unit: TimeUnit.SECOND
            }}
            maxValue={{
                type: ValueType.DURATION,
                value: 5,
                unit: TimeUnit.MINUTE
            }}
        />);

        const warningDiv = container.querySelector("div");
        expect(warningDiv).not.toBeNull();
        expect(warningDiv!!.className).toContain("text-yellow-700");
        expect(warningDiv!!.className).toContain("font-bold");

        expect(warningDiv?.querySelector("p:nth-child(1)")?.textContent).toBe("⚠️ The rule will most likely not execute ⚠️");
        expect(warningDiv?.querySelector("p:nth-child(2)")?.textContent).toBe("The token is exactly 5 minutes old");
    })

});