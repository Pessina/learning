import { Locator, Page } from "@playwright/test";

export class NavigationPage {
  readonly page: Page;
  readonly formsLayoutMenuItem: Locator;

  constructor(page: Page) {
    this.page = page;
    this.formsLayoutMenuItem = page.getByText("Form Layouts");
  }

  async navigateToLayouts() {
    await this.page.getByText("Forms").click();
    await this.formsLayoutMenuItem.click();
  }
}
