import test, { expect } from "@playwright/test";

test.beforeEach(async ({ page }) => {
  await page.goto("https://www.globalsqa.com/demo-site/draganddrop/");
});

test("iframe", async ({ page }) => {
  const frame = page.frameLocator('[rel-title="Photo Manager"] iframe');

  await frame
    .locator("li", { hasText: "High Tatras 2" })
    .dragTo(frame.locator("#trash"));

  await frame.locator("li", { hasText: "High Tatras 3" }).hover();
  await page.mouse.down();
  await frame.locator("#trash").hover();
  await page.mouse.up();

  await expect(frame.locator("#trash li h5")).toHaveText([
    "High Tatras 2",
    "High Tatras 3",
  ]);
});