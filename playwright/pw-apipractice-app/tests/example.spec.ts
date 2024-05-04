import { test, expect } from "@playwright/test";
import tags from "../test-data/tags.json";

test.beforeEach("has title", async ({ page }) => {
  await page.route("*/**/api/tags", async (route) => {
    await route.fulfill({
      body: JSON.stringify(tags),
    });
  });

  await page.route("*/**/api/articles*", async (route) => {
    const response = await route.fetch();
    const data = await response.json();
    data.articles[0].title = "My new title";
    data.articles[0].description = "My new description";

    await route.fulfill({
      body: JSON.stringify(data),
    });
  });

  await page.goto("https://conduit.bondaracademy.com/");
});

test("check title", async ({ page }) => {
  await expect(page.locator(".navbar-brand")).toHaveText("conduit");
  // await page.getByText("Automation").click();
  await expect(
    page.locator("app-article-list").locator("h1").first()
  ).toHaveText("My new title");
  await expect(
    page.locator("app-article-list").locator("p").first()
  ).toHaveText("My new description");
});
