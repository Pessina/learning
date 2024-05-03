import test from "@playwright/test";
import { NavigationPage } from "../test-pages/NavigationPage";

test.beforeEach(async ({ page }) => {
  await page.goto("http://localhost:4200");
});

test("navigate to layouts", async ({ page }) => {
  const navigationPage = new NavigationPage(page);
  await navigationPage.navigateToLayouts();
});
