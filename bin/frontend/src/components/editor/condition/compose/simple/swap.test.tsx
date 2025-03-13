import {render} from "@testing-library/react";
import {RenderText, SwapType} from "./swap";
import {Timeframe, ValueType} from "@types";

describe("RenderText", () => {

    test("null when minValue and maxValue are missing", () => {
        const {container} = render(<RenderText minTimeframe={Timeframe.H1} maxTimeframe={Timeframe.H1}
                                               type={SwapType.Total}/>);
        expect(container.firstChild).toBeNull();
    });

    test("warning if minValue is greater than maxValue at the same timeframe", () => {
        const {container} = render(<RenderText
            minValue={{
                type: ValueType.COUNT,
                value: 2
            }}
            minTimeframe={Timeframe.H1}
            maxValue={{
                type: ValueType.COUNT,
                value: 1
            }}
            maxTimeframe={Timeframe.H1}
            type={SwapType.Total}
        />);

        const warningDiv = container.querySelector("div");
        expect(warningDiv).not.toBeNull();
        expect(warningDiv!!.className).toContain("text-yellow-700");
        expect(warningDiv!!.className).toContain("font-bold");

        expect(warningDiv?.querySelector("p:nth-child(1)")?.textContent).toBe("⚠️ The rule will never execute ⚠️");
        expect(warningDiv?.querySelector("p:nth-child(2)")?.textContent).toBe("Minimum txn count is greater than the maximum txn count for the same timeframe");
    });

    test("min value === max value - same timeframe", () => {
        const {container} = render(<RenderText
            minValue={{
                type: ValueType.COUNT,
                value: 2
            }}
            minTimeframe={Timeframe.H1}
            maxValue={{
                type: ValueType.COUNT,
                value: 2
            }}
            maxTimeframe={Timeframe.H1}
            type={SwapType.Total}
        />);

        const div = container.querySelector("div")!!;
        expect(div.querySelector("p:nth-child(1)")?.textContent).toBe("Exactly 2 txn occurred in the last 1 hour.");
    });

    test("min value === max value - different timeframe", () => {
        const {container} = render(<RenderText
            minValue={{
                type: ValueType.COUNT,
                value: 2
            }}
            minTimeframe={Timeframe.H1}
            maxValue={{
                type: ValueType.COUNT,
                value: 2
            }}
            maxTimeframe={Timeframe.H6}
            type={SwapType.Total}
        />);

        const div = container.querySelector("div")!!;
        expect(div.querySelector("p:nth-child(1)")?.textContent).toBe("At least 2 txn occurred in the last 1 hour.");
        expect(div.querySelector("p:nth-child(2)")?.textContent).toBe("However, no more than 2 txn should occur in the last 6 hours.");
    });

    test("min value < max value - same timeframe", () => {
        const {container} = render(<RenderText
            minValue={{
                type: ValueType.COUNT,
                value: 1
            }}
            minTimeframe={Timeframe.H1}
            maxValue={{
                type: ValueType.COUNT,
                value: 2
            }}
            maxTimeframe={Timeframe.H1}
            type={SwapType.Total}
        />);

        const div = container.querySelector("div")!!;
        expect(div.querySelector("p:nth-child(1)")?.textContent).toBe("At least 1 txn occurred in the last 1 hour.");
        expect(div.querySelector("p:nth-child(2)")?.textContent).toBe("However, the count should not exceed 2 in the same timeframe.");
    });

    test("min value", () => {
        const {container} = render(<RenderText
            minValue={{
                type: ValueType.COUNT,
                value: 1
            }}
            minTimeframe={Timeframe.M1}
            maxTimeframe={Timeframe.H1}
            type={SwapType.Total}
        />);

        const div = container.querySelector("div")!!;
        expect(div.querySelector("p:nth-child(1)")?.textContent).toBe("At least 1 txn occurred in the last 1 minute.");
    });

    test("max value", () => {
        const {container} = render(<RenderText
            minTimeframe={Timeframe.M1}
            maxValue={{
                type: ValueType.COUNT,
                value: 99
            }}
            maxTimeframe={Timeframe.H1}
            type={SwapType.Total}
        />);

        const div = container.querySelector("div")!!;
        expect(div.querySelector("p:nth-child(1)")?.textContent).toBe("No more than 99 txn should occur in the last 1 hour.");
    });

    describe("text", () => {

        test("total: 1", () => {
            const {container} = render(<RenderText
                minValue={{
                    type: ValueType.COUNT,
                    value: 1
                }}
                minTimeframe={Timeframe.M1}
                maxTimeframe={Timeframe.H1}
                type={SwapType.Total}
            />);

            const div = container.querySelector("div")!!;
            expect(div.querySelector("p:nth-child(1)")?.textContent).toBe("At least 1 txn occurred in the last 1 minute.");
        });

        test("total: 21", () => {
            const {container} = render(<RenderText
                minValue={{
                    type: ValueType.COUNT,
                    value: 21
                }}
                minTimeframe={Timeframe.M1}
                maxTimeframe={Timeframe.H1}
                type={SwapType.Total}
            />);

            const div = container.querySelector("div")!!;
            expect(div.querySelector("p:nth-child(1)")?.textContent).toBe("At least 21 txn occurred in the last 1 minute.");
        });

        test("buy: 1", () => {
            const {container} = render(<RenderText
                minValue={{
                    type: ValueType.COUNT,
                    value: 1
                }}
                minTimeframe={Timeframe.M1}
                maxTimeframe={Timeframe.H1}
                type={SwapType.Buy}
            />);

            const div = container.querySelector("div")!!;
            expect(div.querySelector("p:nth-child(1)")?.textContent).toBe("At least 1 buy txn occurred in the last 1 minute.");
        });

        test("buy: 21", () => {
            const {container} = render(<RenderText
                minValue={{
                    type: ValueType.COUNT,
                    value: 21
                }}
                minTimeframe={Timeframe.M1}
                maxTimeframe={Timeframe.H1}
                type={SwapType.Buy}
            />);

            const div = container.querySelector("div")!!;
            expect(div.querySelector("p:nth-child(1)")?.textContent).toBe("At least 21 buy txn occurred in the last 1 minute.");
        });

        test("sell: 1", () => {
            const {container} = render(<RenderText
                minValue={{
                    type: ValueType.COUNT,
                    value: 1
                }}
                minTimeframe={Timeframe.M1}
                maxTimeframe={Timeframe.H1}
                type={SwapType.Sell}
            />);

            const div = container.querySelector("div")!!;
            expect(div.querySelector("p:nth-child(1)")?.textContent).toBe("At least 1 sell txn occurred in the last 1 minute.");
        });

        test("sell: 21", () => {
            const {container} = render(<RenderText
                minValue={{
                    type: ValueType.COUNT,
                    value: 21
                }}
                minTimeframe={Timeframe.M1}
                maxTimeframe={Timeframe.H1}
                type={SwapType.Sell}
            />);

            const div = container.querySelector("div")!!;
            expect(div.querySelector("p:nth-child(1)")?.textContent).toBe("At least 21 sell txn occurred in the last 1 minute.");
        });
    })
});