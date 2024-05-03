import { test, expect } from "playwright/test";

test.beforeEach(async ({ page }) => {
  await page.goto("http://localhost:4200");
});

test.describe("ui components", async () => {
  test.beforeEach(async ({ page }) => {
    await page.getByText("Forms").click();
    await page.getByText("Form Layouts").click();
  });

  test("input component", async ({ page }) => {
    const emailInput = page
      .locator("nb-card", { hasText: "Using the Grid" })
      .getByRole("textbox", { name: "Email" });

    await emailInput.fill("test@gmail.com");
    await emailInput.clear();
    await emailInput.pressSequentially("test@gmail.com", { delay: 500 });

    // Generic validation
    const emailValue = await emailInput.inputValue();
    expect(emailValue).toBe("test@gmail.com");

    // Locator validation
    await expect(emailInput).toHaveValue("test@gmail.com");
  });

  test("radio", async ({ page }) => {
    const gridForms = page.locator("nb-card", { hasText: "Using the Grid" });
    const radioOption1 = gridForms.getByRole("radio", { name: "Option 1" });
    const radioOption2 = gridForms.getByRole("radio", { name: "Option 2" });

    await radioOption1.check({ force: true });
    const isRadio1Checked = await radioOption1.isChecked();
    expect(isRadio1Checked).toBeTruthy();

    await radioOption2.check({ force: true });
    await expect(radioOption1).not.toBeChecked();
    await expect(radioOption2).toBeChecked();
  });
});

test("checkbox", async ({ page }) => {
  await page.getByText("Modal & Overlays").click();
  await page.getByText("Toastr").click();

  const checkBox1 = page.getByRole("checkbox", { name: "Hide on click" });
  const checkBox2 = page.getByRole("checkbox", {
    name: "Prevent arising of duplicate toast",
  });
  const checkBox3 = page.getByRole("checkbox", {
    name: "Show toast with icon",
  });

  await checkBox1.check({ force: true });
  expect(checkBox1).toBeChecked();

  const allCheckbox = page.getByRole("checkbox");

  for (const c of await allCheckbox.all()) {
    await c.check({ force: true });
    await expect(c).toBeChecked();
  }

  for (const c of await allCheckbox.all()) {
    await c.uncheck({ force: true });
    await expect(c).not.toBeChecked();
  }
});

test("dropdown", async ({ page }) => {
  const dropdown = page.locator("ngx-header nb-select");
  await dropdown.click();

  const options = page.locator("nb-option-list nb-option");

  await options.getByText("Dark").click();
  const header = page.locator("nb-layout-header");
  await expect(header).toHaveCSS("background-color", "rgb(34, 43, 69)");

  const colors = {
    Light: "rgb(255, 255, 255)",
    Dark: "rgb(34, 43, 69)",
    Cosmic: "rgb(50, 50, 89)",
    Corporate: "rgb(255, 255, 255)",
  };

  await dropdown.click();
  await expect(options).toHaveText(Object.keys(colors));

  for (let c in colors) {
    await options.getByText(c).click();
    await expect(header).toHaveCSS("background-color", colors[c]);
    await dropdown.click();
  }
});

test("dialogs", async ({ page }) => {
  await page.getByText("Tables & Data").click();
  await page.getByText("Smart Table").click();

  page.on("dialog", (dialog) => {
    expect(dialog.message()).toEqual("Are you sure you want to delete?");
    dialog.accept();
  });

  await page
    .locator("table")
    .locator("tr", { hasText: "mdo@gmail.com" })
    .locator(".nb-trash")
    .click();
  expect(page.locator("table")).not.toHaveText("mdo@gmail.com");
});
