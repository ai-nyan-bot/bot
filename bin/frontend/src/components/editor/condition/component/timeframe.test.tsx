import {render, screen} from "@testing-library/react";
import "@testing-library/jest-dom";
import {TimeframeText} from "@components/editor/condition/component/timeframe.tsx";
import {Timeframe} from "@types";

describe("TimeframeText", () => {
    it("renders M1", () => {
        render(<TimeframeText value={Timeframe.M1}/>);
        expect(screen.getByText("1 minute")).toBeInTheDocument();
    });

    it("renders M5", () => {
        render(<TimeframeText value={Timeframe.M5}/>);
        expect(screen.getByText("5 minutes")).toBeInTheDocument();
    });

    it("renders M15", () => {
        render(<TimeframeText value={Timeframe.M15}/>);
        expect(screen.getByText("15 minutes")).toBeInTheDocument();
    });

    it("renders H1", () => {
        render(<TimeframeText value={Timeframe.H1}/>);
        expect(screen.getByText("1 hour")).toBeInTheDocument();
    });

    it("renders H6", () => {
        render(<TimeframeText value={Timeframe.H6}/>);
        expect(screen.getByText("6 hours")).toBeInTheDocument();
    });

    it("renders D1", () => {
        render(<TimeframeText value={Timeframe.D1}/>);
        expect(screen.getByText("24 hours")).toBeInTheDocument();
    });

    it("throws an error for an unsupported timeframe", () => {
        const consoleErrorSpy = jest.spyOn(console, "error").mockImplementation(() => {
        });
        expect(() => render(<TimeframeText value={"INVALID" as Timeframe}/>)).toThrow("Unsupported timeframe: INVALID");
        consoleErrorSpy.mockRestore();
    });
});
