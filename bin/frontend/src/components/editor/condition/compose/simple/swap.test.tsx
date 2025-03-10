import {render, screen} from "@testing-library/react";
import "@testing-library/jest-dom";
import {RenderText} from ".";
import {Timeframe} from "@types";

describe("RenderText", () => {
    test("returns null when minValue and maxValue are missing", () => {
        const { container } = render(<RenderText minTimeframe={Timeframe.H1} maxTimeframe={Timeframe.H1} />);
        expect(container.firstChild).toBeNull();
    });

    // test("renders correctly with default props", () => {
    //     // render(<h1>Test</h1>);
    //     render(<RenderText
    //         minTimeframe={Timeframe.H1}
    //         maxTimeframe={Timeframe.H1}
    //     />);
    //
    //     expect(screen.getByText(/at least 10 txn in the last 1 hour/i)).toBeInTheDocument();
    //     // expect(screen.getByText(/but not more than 20 in the last hour/i)).toBeInTheDocument();
    // });

    // test("indicates an error if minValue > maxValue for the same timeframe", () => {
    //     const minValue: ValueNumber = 30;
    //     const maxValue: ValueNumber = 20;
    //
    //     render(<RenderText minValue={minValue} minTimeframe="1h" maxValue={maxValue} maxTimeframe="1h" />);
    //
    //     expect(screen.getByText(/error: minValue should not exceed maxValue/i)).toBeInTheDocument();
    // });
    //
    // test("renders different timeframes correctly", () => {
    //     render(<RenderText minValue={5} minTimeframe="15m" maxValue={50} maxTimeframe="1h" />);
    //
    //     expect(screen.getByText(/at least 5 txn in the last 15 minutes/i)).toBeInTheDocument();
    //     expect(screen.getByText(/but not more than 50 in the last hour/i)).toBeInTheDocument();
    // });
    //
    // test("renders correctly when no minValue or maxValue is provided", () => {
    //     render(<RenderText minTimeframe="30m" maxTimeframe="1d" />);
    //
    //     expect(screen.getByText(/at least 10 txn in the last 30 minutes/i)).toBeInTheDocument();
    //     expect(screen.getByText(/but not more than 20 in the last day/i)).toBeInTheDocument();
    // });
});