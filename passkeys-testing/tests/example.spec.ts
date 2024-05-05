import { faker } from "@faker-js/faker";
import { test, expect, CDPSession } from "@playwright/test";
import crypto from "crypto";

let client: CDPSession;
let authenticatorId: string;

// test.beforeEach(async ({ page }) => {
//   client = await page.context().newCDPSession(page);
//   await client.send("WebAuthn.enable", { enableUI: true });

//   const result = await client.send("WebAuthn.addVirtualAuthenticator", {
//     options: {
//       protocol: "ctap2",
//       transport: "internal",
//       hasResidentKey: true,
//       hasUserVerification: false,
//       isUserVerified: true,
//       automaticPresenceSimulation: false,
//     },
//   });
//   authenticatorId = result.authenticatorId;

//   await page.goto("https://passkeys.io");
// });

// test.afterEach(async () => {
//   await client.send("WebAuthn.clearCredentials", { authenticatorId });

//   await client.send("WebAuthn.removeVirtualAuthenticator", { authenticatorId });
// });

test("credential creation and retrieval", async ({ page }) => {
  const credentialId = await createPasskey({
    client: client,
    email: "test@gmail.com",
    authenticatorId: authenticatorId,
    rpId: "www.passkeys.io",
  });

  await page.getByRole("button", { name: "Sign in with a passkey" }).click();

  const retrievedCredential = await client.send("WebAuthn.getCredential", {
    authenticatorId,
    credentialId,
  });

  console.log("Retrieved Credential:", retrievedCredential);
});

test("test2", async ({ page }) => {
  const email = faker.internet.email();
  await page.goto("https://passkeys.io");

  const client = await page.context().newCDPSession(page);
  // Disable UI for automated testing
  await client.send("WebAuthn.enable", { enableUI: false });

  const result = await client.send("WebAuthn.addVirtualAuthenticator", {
    options: {
      protocol: "ctap2",
      transport: "internal",
      hasResidentKey: true,
      hasUserVerification: true,
      isUserVerified: true,
      // Necessary to trigger passkey automatically.
      // Not clear what I have to do in case I set it to false, I have to basically manually submit the passkeys
      automaticPresenceSimulation: true,
    },
  });
  const authenticatorId = result.authenticatorId;

  client.on("WebAuthn.credentialAdded", () => {
    console.log("WebAuthn.credentialAdded");
  });
  client.on("WebAuthn.credentialAsserted", () => {
    console.log("WebAuthn.credentialAsserted");
  });

  await page.getByRole("textbox", { name: "Email" }).fill(email);
  await page.getByRole("button", { name: "Continue" }).click();
  await page.getByRole("button", { name: "Sign up" }).click();

  await createPasskey({
    client: client,
    email,
    authenticatorId: authenticatorId,
    rpId: "passkeys.io",
  });

  await page.getByRole("button", { name: "Create a passkey" }).click();

  await expect(
    page.getByText("Used for passcode authentication.")
  ).toBeVisible();

  const credentials = await client.send("WebAuthn.getCredentials", {
    authenticatorId,
  });

  for (const c of credentials.credentials) {
    console.log(c);
  }
});

async function createPasskey({
  client,
  email,
  authenticatorId,
  rpId,
}: {
  client: CDPSession;
  email: string;
  authenticatorId: string;
  rpId: string;
}): Promise<string> {
  const keyPair = (await crypto.subtle.generateKey(
    {
      name: "ECDSA",
      namedCurve: "P-256",
    },
    true,
    ["sign", "verify"]
  )) as CryptoKeyPair;

  const pkcs8PrivateKey = (await crypto.subtle.exportKey(
    "pkcs8",
    keyPair.privateKey
  )) as ArrayBuffer;

  const base64PrivateKey = btoa(
    String.fromCharCode.apply(null, new Uint8Array(pkcs8PrivateKey))
  );

  const credentialId = crypto.randomUUID();

  const userHandle = btoa(email);

  const credential = {
    credentialId: btoa(credentialId),
    isResidentCredential: true,
    privateKey: base64PrivateKey,
    signCount: 0,
    userHandle: userHandle,
    rpId,
  };

  await client.send("WebAuthn.addCredential", {
    authenticatorId,
    credential,
  });

  return btoa(credentialId);
}
