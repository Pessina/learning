import { assert } from "console";
import { test, expect } from "playwright/test";

test.beforeEach(async ({ page }) => {
  await page.goto("http://localhost:4200");
  await page.getByText("Forms").click();
  await page.getByText("Form Layouts").click();
});

test("test locators", async ({ page }) => {
  await page.locator("input").first().click();

  await page.locator("#inputEmail1").click();
});

test("test user-facing locators", async ({ page }) => {
  await page.getByRole("textbox", { name: "Email" }).first().click();
  await page.getByText("Using the Grid").click();
  await page.getByTestId("SignIn").click();
});

test("test getting child components", async ({ page }) => {
  await page
    .locator("nb-card")
    .locator("nb-radio")
    .getByText("Option 1")
    .click();
});

test("getting parent elements", async ({ page }) => {
  await page
    .locator("nb-card", { hasText: "Using the Grid" })
    .getByText("Sign in")
    .click();

  await page
    .locator("nb-card")
    .filter({ hasText: "Using the Grid" })
    .getByText("Sign in")
    .click();
});

test("reusing locators", async ({ page }) => {
  const baseForms = page.locator("nb-card", { hasText: "Basic form" });
  const emailField = baseForms.getByText("Email");

  await emailField.fill("test@gmail.com");
  await baseForms.getByText("Password").fill("123456");
  await baseForms.getByText("Submit").click();

  await expect(emailField).toHaveValue("test@gmail.com");
});

test("validating values", async ({ page }) => {
  const basicForms = page.locator("nb-card", { hasText: "Basic form" });
  const buttonText = await basicForms.locator("button").textContent();
  expect(buttonText).toBe("Submit");

  const gridForms = page.locator("nb-card", { hasText: "Using the Grid" });
  const optionsText = await gridForms.locator("nb-radio").allTextContents();
  expect(optionsText).toContain("Option 1");
  const emailField = basicForms.getByRole("textbox", { name: "Email" });

  await emailField.fill("test@gmail.com");
  const emailValue = await emailField.inputValue();
  expect(emailValue).toBe("test@gmail.com");

  const emailPlaceholder = await emailField.getAttribute("placeholder");
  expect(emailPlaceholder).toBe("Email");
});

test("auto waiting", async ({ page }) => {
  await page.goto("https://www.uitestingplayground.com/ajax");
  await page.getByText("Button Triggering AJAX Request").click();

  await page.waitForResponse("https://www.uitestingplayground.com/ajaxdata");
  // await page.waitForLoadState("networkidle");
  // await page.waitForTimeout(5000);

  await page.getByText("Data loaded with AJAX get request.").click();
});

test("timeouts", async ({ page }) => {
  test.setTimeout(10000);
  test.slow();

  await page.goto("https://www.uitestingplayground.com/ajax");
  await page.getByText("Button Triggering AJAX Request").click();
  await page.waitForResponse("https://www.uitestingplayground.com/ajaxdata");

  await page.getByText("Data loaded with AJAX get request.").click();
});
