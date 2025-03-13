import {render} from "@testing-library/react";
import {RenderText} from "@components/editor/condition/compose/simple/market-cap";
import {ValueType} from "@types";

describe("RenderText", () => {

    test("null when minValue and maxValue are missing", () => {
        const {container} = render(<RenderText/>);
        expect(container.firstChild).toBeNull();
    });

    test("min value < max value", () => {
        const {container} = render(<RenderText
            minValue={{
                type: ValueType.SOL,
                value: 2
            }}
            maxValue={{
                type: ValueType.SOL,
                value: 1
            }}
        />);

        const warningDiv = container.querySelector("div");
        expect(warningDiv).not.toBeNull();
        expect(warningDiv!!.className).toContain("text-yellow-700");
        expect(warningDiv!!.className).toContain("font-bold");

        expect(warningDiv?.querySelector("p:nth-child(1)")?.textContent).toBe("⚠️ The rule will never execute ⚠️");
        expect(warningDiv?.querySelector("p:nth-child(2)")?.textContent).toBe("Minimum market cap is greater than the maximum market cap");
    });

    test("min value", () => {
        const {container} = render(<RenderText
            minValue={{
                type: ValueType.SOL,
                value: 1
            }}
        />);

        const div = container.querySelector("div")!!;
        expect(div.querySelector("p:nth-child(1)")?.textContent).toBe("The token must have a market cap of at least 1 SOL");
    })

    test("max value", () => {
        const {container} = render(<RenderText
            maxValue={{
                type: ValueType.USD,
                value: 1000
            }}
        />);

        const div = container.querySelector("div")!!;
        expect(div.querySelector("p:nth-child(1)")?.textContent).toBe("The token must have a market cap no higher than $1000");
    })


});