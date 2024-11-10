import { EthSignatureProgram, ProofInput } from "./EthSignatureProgram.js";
import {Wallet} from 'ethers'
import { createForeignCurve, Crypto, createEcdsa, Bytes, Bool, Provable, VerificationKey, verify } from "o1js";

class Secp256k1  extends createForeignCurve(Crypto.CurveParams.Secp256k1){}
class ECDSA extends createEcdsa(Secp256k1){}
class Bytes32 extends Bytes(32){}

describe("Signature verification Happy Path", () => {
    let vk: VerificationKey
    beforeAll(async () => {
        console.log("Compiling...")
        const compileArtifact = await EthSignatureProgram.compile()
        vk = compileArtifact.verificationKey
        console.timeEnd("Compiling...")
    })


    it("Verifies the signature", async () => {
        const message = "Hello Bangkok".padEnd(32, '0')
        const wallet = Wallet.createRandom(); 

        const signature = await wallet.signMessage(message)
        const publicKey = wallet.signingKey.compressedPublicKey;

        const o1jsPublicKey = Secp256k1.fromEthers(publicKey)
        const o1jsMessage = Bytes32.fromString(message)
        const o1jsSignature = ECDSA.fromHex(signature)

        const isSignatureValid = await EthSignatureProgram.verifySignature(
            new ProofInput({
                message: o1jsMessage, 
                publicKey: o1jsPublicKey
            }), 
            o1jsSignature
        )

        expect(isSignatureValid.proof.publicOutput.toBoolean()).toBeTruthy(); 

        console.log("Message:", isSignatureValid.proof.publicInput.message.toString());
        console.log("Public Key:", isSignatureValid.proof.publicInput.publicKey.toString());

        expect(await verify(isSignatureValid.proof, vk)).toBe(true)
    })
})