import { test, expect } from "@playwright/test";

test.beforeEach("has title", async ({ page }) => {
  await page.goto("https://conduit.bondaracademy.com/");
});

test("check title", async ({ page }) => {
  await expect(page.locator(".navbar-brand")).toHaveText("conduit");
});
