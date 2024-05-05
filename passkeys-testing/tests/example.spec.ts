import { test, expect, CDPSession } from "@playwright/test";
import crypto from "crypto";

let client: CDPSession;
let authenticatorId: string;

test.beforeEach(async ({ page }) => {
  await page.goto("https://passkeys.eu");

  client = await page.context().newCDPSession(page);

  // Enable WebAuthn environment in this session
  await client.send("WebAuthn.enable", { enableUI: true });

  // Attach a virtual authenticator with specific options
  const result = await client.send("WebAuthn.addVirtualAuthenticator", {
    options: {
      protocol: "ctap2",
      transport: "internal",
      hasResidentKey: true,
      hasUserVerification: true,
      isUserVerified: true,
      automaticPresenceSimulation: true,
    },
  });
  authenticatorId = result.authenticatorId;
});

test("has correct title", async ({ page }) => {
  await expect(page.locator("h1")).toHaveText("Passkeys demo");
});

test("login works", async ({ page }) => {
  await page
    .getByRole("textbox", { name: "Email address" })
    .fill("sssdsfds@gmail.com");

  await page.getByRole("button", { name: "Passkey signup" }).click();
  await page.getByRole("button", { name: "Create your account" }).click();
  await expect(page.getByText("You're logged in.")).toBeVisible({
    timeout: 30000,
  });
});

test("credential creation and retrieval", async ({ page }) => {
  // Generate an ECDSA key pair with P-256 curve
  const keyPair = await crypto.subtle.generateKey(
    {
      name: "ECDSA",
      namedCurve: "P-256",
    },
    true,
    ["sign", "verify"]
  );

  // Export the private key in PKCS#8 format
  const pkcs8PrivateKey = await crypto.subtle.exportKey(
    "pkcs8",
    keyPair.privateKey
  );

  // Convert the ArrayBuffer to a base64 string
  const base64PrivateKey = btoa(
    String.fromCharCode.apply(null, new Uint8Array(pkcs8PrivateKey))
  );

  const credentialId = crypto.randomUUID();

  // Assuming `base64PrivateKey` contains a valid ECDSA P-256 private key in PKCS#8 format

  const userHandle = btoa("some-user-identifier");

  const credential = {
    credentialId: btoa(credentialId),
    isResidentCredential: true,
    privateKey: base64PrivateKey,
    signCount: 0,
    userHandle: userHandle,
    rpId: "example.com",
  };

  console.log({
    authenticatorId,
    credential,
  });

  await client.send("WebAuthn.addCredential", {
    authenticatorId,
    credential,
  });

  // Retrieve the credential just added
  const retrievedCredential = await client.send("WebAuthn.getCredential", {
    authenticatorId,
    credentialId: btoa(credentialId),
  });

  console.log("Retrieved Credential:", retrievedCredential);

  // Clear all credentials from the authenticator
  await client.send("WebAuthn.clearCredentials", { authenticatorId });

  // Remove the virtual authenticator
  await client.send("WebAuthn.removeVirtualAuthenticator", { authenticatorId });
});
