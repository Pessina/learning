use anchor_lang::prelude::*;

use base64ct::{Base64UrlUnpadded, Encoding};
use rsa::{pkcs1v15::VerifyingKey, signature::Verifier, BigUint, RsaPublicKey};
use serde_json;
use sha2::Sha256;

pub fn verify_oidc_signature_impl() -> Result<bool> {
    let mut google_keys = vec![];
    google_keys.push(PublicKey {
        e: "AQAB".to_string(),
        kid: "89ce3598c473af1bda4bff95e6c8736450206fba".to_string(),
        use_: "sig".to_string(),
        kty: "RSA".to_string(),
        n: "wvLUmyAlRhJkFgok97rojtg0xkqsQ6CPPoqRUSXDIYcjfVWMy1Z4hk_-90Y554KTuADfT_0FA46FWb-pr4Scm00gB3CnM8wGLZiaUeDUOu84_Zjh-YPVAua6hz6VFa7cpOUOQ5ZCxCkEQMjtrmei21a6ijy5LS1n9fdiUsjOuYWZSoIQCUj5ow5j2asqYYLRfp0OeymYf6vnttYwz3jS54Xe7tYHW2ZJ_DLCja6mz-9HzIcJH5Tmv5tQRhAUs3aoPKoCQ8ceDHMblDXNV2hBpkv9B6Pk5QVkoDTyEs7lbPagWQ1uz6bdkxM-DnjcMUJ2nh80R_DcbhyqkK4crNrM1w".to_string(),
        alg: "RS256".to_string(),
    });
    google_keys.push(PublicKey {
        e: "AQAB".to_string(),
        kid: "dd125d5f462fbc6014aedab81ddf3bcedab70847".to_string(),
        use_: "sig".to_string(),
        kty: "RSA".to_string(),
        n: "jwstqI4w2drqbTTVRDriFqepwVVI1y05D5TZCmGvgMK5hyOsVW0tBRiY9Jk9HKDRue3vdXiMgarwqZEDOyOA0rpWh-M76eauFhRl9lTXd5gkX0opwh2-dU1j6UsdWmMa5OpVmPtqXl4orYr2_3iAxMOhHZ_vuTeD0KGeAgbeab7_4ijyLeJ-a8UmWPVkglnNb5JmG8To77tSXGcPpBcAFpdI_jftCWr65eL1vmAkPNJgUTgI4sGunzaybf98LSv_w4IEBc3-nY5GfL-mjPRqVCRLUtbhHO_5AYDpqGj6zkKreJ9-KsoQUP6RrAVxkNuOHV9g1G-CHihKsyAifxNN2Q".to_string(),
        alg: "RS256".to_string(),
    });

    let oidc_data =  OIDCValidationData{
        token: "eyJhbGciOiJSUzI1NiIsImtpZCI6Ijg5Y2UzNTk4YzQ3M2FmMWJkYTRiZmY5NWU2Yzg3MzY0NTAyMDZmYmEiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiI3Mzk5MTEwNjk3OTctaWRwMDYyODY2OTY0Z2JuZG82NjkzaDMydGdhNWN2bDEuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiI3Mzk5MTEwNjk3OTctaWRwMDYyODY2OTY0Z2JuZG82NjkzaDMydGdhNWN2bDEuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTc5MDI4NTUzNzMxNTc0MTAzMzAiLCJlbWFpbCI6ImZzLnBlc3NpbmFAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsIm5vbmNlIjoidGVzdF8xMjNfZmVsaXBlIiwibmJmIjoxNzM2NTIzMjM2LCJuYW1lIjoiRmVsaXBlIFBlc3NpbmEiLCJwaWN0dXJlIjoiaHR0cHM6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL2EvQUNnOG9jSktKYlV5QlZxQ0J2NHFWR09EU25WVGdMSFBLTjB0Vk9NSU1YVml1a2dyZC0wdGZlZFU9czk2LWMiLCJnaXZlbl9uYW1lIjoiRmVsaXBlIiwiZmFtaWx5X25hbWUiOiJQZXNzaW5hIiwiaWF0IjoxNzM2NTIzNTM2LCJleHAiOjE3MzY1MjcxMzYsImp0aSI6ImY3MjdlZjg1MGFhNzNmMDQ3ZmQwNjY5OWIwNjk3YTIwMDIzYWViYWMifQ.nlRKhlzBhHVpYejoSkH_S9ZOeAejlhvnL5u-94AzsREIhzuKroJbPp9jEHuvvki5dJozc-FzXx9lfpjT17X6PT0hJOM86QUE05RkmV9WkrVSr8trr1zbHY6dieii9tzj7c01pXsLJTa2FvTonmJAxDteVt_vsZFl7-pRWmyXKLMk4CFv9AZx20-uj5pDLuj-F5IkAk_cpXBuMJYh5PQeNBDk22d5svDTQkuwUAH5N9sssXRzDNdv92snGu4AykpmoPIJeSmc3EY-RW0TB5bAnwXH0E3keAjv84yrNYjnovYn2FRqKbTKxNxN4XUgWU_P0oRYCzckJznwz4tStaYZ2A".to_string(),
        message: "test_123_felipe".to_string()
    };

    let oidc_identity = OIDCAuthenticator {
        client_id: "739911069797-idp062866964gbndo6693h32tga5cvl1.apps.googleusercontent.com"
            .to_string(),
        issuer: "https://accounts.google.com".to_string(),
        email: Some("fs.pessina@gmail.com".to_string()),
        sub: None,
    };

    let parts: Vec<&str> = oidc_data.token.split('.').collect();
    if parts.len() != 3 {
        panic!("Invalid JWT format - token must have 3 parts");
    }
    let (header_b64, payload_b64, sig_b64) = (parts[0], parts[1], parts[2]);

    let payload_json =
        Base64UrlUnpadded::decode_vec(payload_b64).expect("Failed to decode JWT payload");
    let payload: serde_json::Value =
        serde_json::from_slice(&payload_json).expect("Failed to parse JWT payload as JSON");

    let token_issuer = payload["iss"].as_str().unwrap_or_default();
    if token_issuer != oidc_identity.issuer {
        panic!("Token issuer does not match expected issuer");
    }
    let token_client_id = payload["aud"].as_str().unwrap_or_default();
    if token_client_id != oidc_identity.client_id {
        panic!("Token audience does not match expected client ID");
    }

    let token_email = payload["email"].as_str();
    let token_sub = payload["sub"].as_str();

    match (
        token_email,
        token_sub,
        &oidc_identity.email,
        &oidc_identity.sub,
    ) {
        (Some(email), _, Some(expected_email), _) if email == expected_email => {}
        (_, Some(sub), _, Some(expected_sub)) if sub == expected_sub => {}
        _ => panic!("Token email/subject does not match expected values"),
    }

    let token_nonce = payload["nonce"].as_str().unwrap_or_default();
    if token_nonce != oidc_data.message {
        panic!("Token nonce does not match expected message");
    }

    let header_json =
        Base64UrlUnpadded::decode_vec(header_b64).expect("Failed to decode JWT header");
    let header: serde_json::Value =
        serde_json::from_slice(&header_json).expect("Failed to parse JWT header as JSON");

    let kid = header["kid"].as_str().unwrap_or_default();
    let public_key = google_keys
        .iter()
        .find(|pk| pk.kid == kid)
        .expect("Key ID not found in issuer's key set");

    let n =
        Base64UrlUnpadded::decode_vec(&public_key.n).expect("Failed to decode public key modulus");
    let e =
        Base64UrlUnpadded::decode_vec(&public_key.e).expect("Failed to decode public key exponent");

    let rsa_pubkey = RsaPublicKey::new(BigUint::from_bytes_be(&n), BigUint::from_bytes_be(&e))
        .expect("Failed to construct RSA public key");

    let verifying_key = VerifyingKey::<Sha256>::new(rsa_pubkey);
    let message = format!("{}.{}", header_b64, payload_b64);
    let signature = Base64UrlUnpadded::decode_vec(sig_b64).expect("Failed to decode JWT signature");

    msg!("Verifying signature");

    let result = verifying_key
        .verify(
            message.as_bytes(),
            &rsa::pkcs1v15::Signature::try_from(signature.as_slice())
                .expect("Failed to parse signature"),
        )
        .is_ok();

    Ok(result)
}

pub struct PublicKey {
    pub kid: String,
    pub n: String,
    pub e: String,
    pub alg: String,
    pub kty: String,
    pub use_: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct OIDCAuthenticator {
    pub client_id: String,
    pub issuer: String,
    pub email: Option<String>,
    pub sub: Option<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct OIDCValidationData {
    pub token: String,
    pub message: String,
}

#[derive(Accounts)]
pub struct VerifyOIDCSignature<'info> {
    pub payer: Signer<'info>,
}
