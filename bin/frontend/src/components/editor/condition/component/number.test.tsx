import {render, screen} from "@testing-library/react";
import "@testing-library/jest-dom";
import {ValueType} from "@types";
import {NumberText} from "@components/editor/condition/component/number.tsx";

describe("NumberText", () => {
    it("renders SOL", () => {
        render(<NumberText type={ValueType.SOL} value={23}/>);
        expect(screen.getByText("23 SOL")).toBeInTheDocument();
    });
    it("renders USD", () => {
        render(<NumberText type={ValueType.USD} value={23}/>);
        expect(screen.getByText("$23")).toBeInTheDocument();
    });
    it("throws an error for an unsupported type", () => {
        const consoleErrorSpy = jest.spyOn(console, "error").mockImplementation(() => {
        });
        expect(() => render(<NumberText value={10} type={ValueType.STRING}/>))
            .toThrow("Unsupported type: STRING");
        consoleErrorSpy.mockRestore();
    });
});
