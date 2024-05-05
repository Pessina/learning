import { faker } from "@faker-js/faker";
import { test, expect, CDPSession } from "@playwright/test";
import crypto from "crypto";

test("test2", async ({ page }) => {
  const email = faker.internet.email();
  await page.goto("https://passkeys.io");
  const { client, authenticatorId } = await setupVirtualAuthenticator(page);

  await page.getByRole("textbox", { name: "Email" }).fill(email);
  await page.getByRole("button", { name: "Continue" }).click();
  await page.getByRole("button", { name: "Sign up" }).click();

  // Add one credential
  await createPasskey({
    client: client,
    email,
    authenticatorId: authenticatorId,
    rpId: "passkeys.io",
  });

  // Add one credential
  // TODO: Check how to re-use the credential created above for creation of passkey so we can have a fixed user for all testing purpose.
  // Maybe we can use a unique email and use recovery for testing, but not optimal coz we have to always call the recovery. Ideal it's have a hard-coded value on FE
  await page.getByRole("button", { name: "Create a passkey" }).click();

  await expect(
    page.getByText("Used for passcode authentication.")
  ).toBeVisible();

  const credentials = await client.send("WebAuthn.getCredentials", {
    authenticatorId,
  });

  // Prints two credential
  for (const c of credentials.credentials) {
    console.log(c);
  }
});

async function setupVirtualAuthenticator(page) {
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

  return { client, authenticatorId };
}

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
