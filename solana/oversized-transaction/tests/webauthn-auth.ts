import * as anchor from "@coral-xyz/anchor";
import { sha256 } from "js-sha256";
import { Program } from "@coral-xyz/anchor";
import {
  ComputeBudgetProgram,
  PublicKey,
  TransactionInstruction,
} from "@solana/web3.js";
import { OversizedTransaction } from "../target/types/oversized_transaction";
import { assert } from "chai";

const SECP256R1_PROGRAM_ID = new PublicKey(
  "Secp256r1SigVerify1111111111111111111111111"
);

describe.only("Webauthn Auth", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .oversizedTransaction as Program<OversizedTransaction>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;

  const compressedPublicKey =
    "0x0220fb23e028391b72c517850b3cc83ba529ef4db766098a29bf3c8d06be957878";

  function createSecp256r1VerificationInstruction(
    signature: Uint8Array,
    publicKey: Uint8Array,
    message: Uint8Array
  ): TransactionInstruction {
    const data = Buffer.alloc(
      2 + 14 + publicKey.length + signature.length + message.length
    );

    data.writeUInt8(1, 0);
    data.writeUInt8(0, 1);

    const offsets = {
      signature_offset: 2 + 14,
      signature_instruction_index: 0xffff,
      public_key_offset: 2 + 14 + signature.length,
      public_key_instruction_index: 0xffff,
      message_data_offset: 2 + 14 + signature.length + publicKey.length,
      message_data_size: message.length,
      message_instruction_index: 0xffff,
    };

    data.writeUInt16LE(offsets.signature_offset, 2);
    data.writeUInt16LE(offsets.signature_instruction_index, 4);
    data.writeUInt16LE(offsets.public_key_offset, 6);
    data.writeUInt16LE(offsets.public_key_instruction_index, 8);
    data.writeUInt16LE(offsets.message_data_offset, 10);
    data.writeUInt16LE(offsets.message_data_size, 12);
    data.writeUInt16LE(offsets.message_instruction_index, 14);

    data.set(signature, offsets.signature_offset);
    data.set(publicKey, offsets.public_key_offset);
    data.set(message, offsets.message_data_offset);

    return new TransactionInstruction({
      keys: [],
      programId: SECP256R1_PROGRAM_ID,
      data: data,
    });
  }

  it.only("should validate WebAuthN signature correctly", async () => {
    let webauthn_data = {
      signature:
        "0xf77969b7eaeaaed4b9a5cc5636b3755259d29d1406d8e852a8ce43dc74644da11453962702ea21a9efdd4a7077e39fcd754e3d01579493cf972f0151b6672f1f",
      authenticatorData:
        "0x49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97631900000000",
      clientData:
        '{"type":"webauthn.get","challenge":"tAuyPmQcczI8CFoTekJz5iITeP80zcJ60VTC4sYz5s8","origin":"http://localhost:3000","crossOrigin":false}',
    };

    const clientDataHash = sha256.arrayBuffer(webauthn_data.clientData);
    const message = Buffer.concat([
      Buffer.from(webauthn_data.authenticatorData.slice(2), "hex"),
      Buffer.from(clientDataHash),
    ]);

    const verificationInstruction = createSecp256r1VerificationInstruction(
      Buffer.from(webauthn_data.signature.slice(2), "hex"),
      Buffer.from(compressedPublicKey.slice(2), "hex"),
      message
    );

    const webauthnData = {
      signature: webauthn_data.signature,
      authenticatorData: webauthn_data.authenticatorData,
      clientData: webauthn_data.clientData,
    };

    const signatureTx = await program.methods
      .verifyWebauthnSignature(webauthnData, compressedPublicKey)
      .accounts({
        instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      })
      .preInstructions([verificationInstruction])
      .rpc();

    console.log("Transaction signature:", signatureTx);
  });

  it("should fail to validate WebAuthN signature with wrong public key", async () => {
    const wrongCompressedPublicKey =
      "0x0220fb23e028391b72c517850b3cc83ba529ef4db766098a29bf3c8d06be957878";

    let webauthnData = {
      signature:
        "0x563a2aba62db8a60c0877a87a2c6db9637bba0b7d8fd505628947e763371c01669ac141b8bc054d27a5cee9438ac7f6f11537523a6ab8affc0557b634f082cea",
      authenticatorData:
        "49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97631d00000000",
      clientData:
        '{"type":"webauthn.get","challenge":"cmFuZG9tLWNoYWxsZW5nZQ","origin":"http://localhost:3000","crossOrigin":false}',
    };

    const computeBudgetIx = ComputeBudgetProgram.setComputeUnitLimit({
      units: 1_400_000,
    });

    const txSignature = await program.methods
      .verifyWebauthnSignature(webauthnData, wrongCompressedPublicKey)
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .preInstructions([computeBudgetIx])
      .rpc({
        skipPreflight: false,
        commitment: "confirmed",
      });

    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
      {
        signature: txSignature,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      },
      "confirmed"
    );

    const txInfo = (await provider.connection.getTransaction(txSignature, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0,
    })) as unknown as {
      meta: {
        returnData: {
          data: string[];
        };
        logMessages: string[];
      };
    };

    let returnValue = txInfo?.meta?.returnData?.data
      ? (Buffer.from(txInfo.meta.returnData.data[0], "base64")[0] as unknown as
          | 0
          | 1)
      : null;

    assert.isTrue(returnValue !== 1, "Should have a return value");
  });
});
