import React from "react";
import {fireEvent, render, screen} from "@testing-library/react";
import {LoadingButton} from "./button-loading";
import "@testing-library/jest-dom";

describe("LoadingButton", () => {

    test("default state", () => {
        render(<LoadingButton text="Update" loadingText="Updating..." loading={false} onClick={() => {
        }}/>);

        const button = screen.getByRole("button", {name: "Update"});
        expect(button).toBeInTheDocument();
        expect(button).toHaveTextContent("Update");
    });

    test("loading state", () => {
        render(<LoadingButton text="Update" loadingText="Updating..." loading={true} onClick={() => {
        }}/>);

        const button = screen.getByRole("button", {name: "⏳ Updating..."});
        expect(button).toBeInTheDocument();
        expect(button).toHaveTextContent("Updating...");
        expect(button).toBeDisabled();
    });

    test("clickable when not loading", () => {
        const handleClick = jest.fn();
        render(<LoadingButton text="Update" loadingText="Updating..." loading={false} onClick={handleClick}/>);

        const button = screen.getByRole("button", {name: "Update"});
        fireEvent.click(button);

        expect(handleClick).toHaveBeenCalledTimes(1);
    });


    test("not clickable when already loading", () => {
        const handleClick = jest.fn();
        render(<LoadingButton text="Update" loadingText="Updating..." loading={true} onClick={handleClick}/>);

        const button = screen.getByRole("button", {name: "⏳ Updating..."});
        fireEvent.click(button);

        expect(handleClick).not.toHaveBeenCalled();
    });

});