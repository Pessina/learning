import { ZkProgram, Struct, createForeignCurve, Crypto, createEcdsa, Bytes, Bool } from "o1js";

class Secp256k1  extends createForeignCurve(Crypto.CurveParams.Secp256k1){}
class ECDSA extends createEcdsa(Secp256k1){}
class Bytes32 extends Bytes(32){}

export class ProofInput  extends Struct ({
    message: Bytes32, 
    publicKey: Secp256k1
}) {}

export const EthSignatureProgram = ZkProgram({
    name: "EthSignatureProgram",
    publicInput: ProofInput, 
    publicOutput: Bool, 
    methods: {
        verifySignature: {
            privateInputs: [ECDSA], 
            async method(
                proofInput: ProofInput, 
                signature: ECDSA
            ) {
                return {
                    publicOutput: signature.verifyEthers(
                        proofInput.message, 
                        proofInput.publicKey
                    )
                }
            }
        }
    }
})