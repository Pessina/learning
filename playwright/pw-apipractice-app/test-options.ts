import { test as base } from "@playwright/test";

export type TestOptions = {
  baseURL: string;
  baseAPIURL: string;
};

export const test = base.extend<TestOptions>({
  baseAPIURL: ["", { option: true }],
});
