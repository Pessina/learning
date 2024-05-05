import { test as setup } from "../test-options";
import user from "../.auth/user.json";
import fs from "fs";

const authFile = ".auth/user.json";

setup("authentication", async ({ request, baseAPIURL }) => {
  // await page.goto("https://conduit.bondaracademy.com/");
  // await page.getByText("Sign in").click();
  // await page.getByRole("textbox", { name: "Email" }).fill("fs@gmail.com");
  // await page.getByRole("textbox", { name: "Password" }).fill("123123");
  // await page.getByRole("button").click();

  // await page.waitForTimeout(1000);

  // await page.context().storageState({ path: authFile });

  const authRes = await request.post(`${baseAPIURL}/api/users/login`, {
    data: { user: { email: "fs@gmail.com", password: "123123" } },
  });

  const token = (await authRes.json()).user.token;
  user.origins[0].localStorage[0].value = token;
  fs.writeFileSync(authFile, JSON.stringify(user));

  process.env["AUTH_TOKEN"] = token;
});
