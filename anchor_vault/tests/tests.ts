import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault_types";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { BN } from "bn.js";

const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    "confirmed"
  );
  return signature;
};

describe("vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorVault as Program<AnchorVault>;

  const provider = anchor.getProvider();

  const user = new Keypair();

  const vaultState = PublicKey.findProgramAddressSync(
    [Buffer.from("state"), user.publicKey.toBuffer()],
    program.programId
  )[0];
  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultState.toBuffer()],
    program.programId
  )[0];

  it("Airdrop", async () => {
    await provider.connection
      .requestAirdrop(user.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL)
      .then(confirmTx);
  });

  it("Initialize", async () => {
    try {
      const tx = await program.methods
        .initialize()
        .accountsStrict({
          user: user.publicKey,
          vaultState,
          vault,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc()
        .then(confirmTx);
      console.log("Your transaction signature", tx);
    } catch (e) {
      console.error(e);
      throw e;
    }
  });

  it("Deposit", async () => {
    try {
      const tx = await program.methods
        .deposit(new BN(10000000))
        .accountsStrict({
          user: user.publicKey,
          vaultState,
          vault,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc()
        .then(confirmTx);
      console.log("Your transaction signature", tx);
    } catch (e) {
      console.error(e);
      throw e;
    }
  });

  it("Withdraw", async () => {
    try {
      const tx = await program.methods
        .withdraw(new BN(1000000))
        .accountsStrict({
          user: user.publicKey,
          vaultState,
          vault,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc()
        .then(confirmTx);
      console.log("Your transaction signature", tx);
    } catch (e) {
      console.error(e);
      throw e;
    }
  });

  it("Close", async () => {
    try {
      const tx = await program.methods
        .close()
        .accountsStrict({
          user: user.publicKey,
          vaultState,
          vault,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc({ skipPreflight: true })
        .then(confirmTx);
      console.log("Your transaction signature", tx);
    } catch (e) {
      console.error(e);
      throw e;
    }
  });
});
