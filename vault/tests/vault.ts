import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";

describe("vault", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  let connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.Vault as Program<Vault>;

  let user = anchor.web3.Keypair.generate();
  let [vault, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), user.publicKey.toBuffer()],
    program.programId
  );
  it("airdrop", async () => {
    const tx = await connection.requestAirdrop(
      user.publicKey,
      100 * anchor.web3.LAMPORTS_PER_SOL
    );
    console.log("airdrop", tx);
    await connection.confirmTransaction(tx);
  });
  it("Is initialized!", async () => {
    const tx = await program.methods
      .initalize()
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();
    console.log("initalized", tx);
  });

  it("deposit", async () => {
    const tx = await program.methods
      .deposit(new anchor.BN(20))
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();

    console.log("deposit", tx);
  });

  it("withdrawl", async () => {
    const tx = await program.methods
      .withdraw(new anchor.BN(5))
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();

    console.log("withdrawl", tx);
  });
});
