/** @type {import('ts-jest').JestConfigWithTsJest} **/
export default {
    testEnvironment: "jsdom",
    transform: {
        "^.+.tsx?$": ["ts-jest", {}],
    },
    setupFilesAfterEnv: ["<rootDir>/jest.setup.ts"],
    moduleNameMapper: {
        "^@app/types$": "<rootDir>/src/types",
        "^@app/(.*)$": "<rootDir>/src/app/$1",
        "^@components/(.*)$": "<rootDir>/src/components/$1",
        "^@types$": "<rootDir>/src/types/index.ts",
        "^@utils$": "<rootDir>/src/utils/index.ts",
        "^@utils/(.*)$": "<rootDir>/src/utils/$1",
    },
};