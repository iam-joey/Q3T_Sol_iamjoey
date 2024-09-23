import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PredictionMarket } from "../target/types/prediction_market";

import { randomBytes } from "crypto";
import { seed } from "@coral-xyz/anchor/dist/cjs/idl";

describe("prediction-market", () => {
  let provider = anchor.AnchorProvider.env();
  let connection = provider.connection;
  anchor.setProvider(provider);
  const program = anchor.workspace
    .PredictionMarket as Program<PredictionMarket>;
  //admin, user1,user2
  let [admin, maker, betTaker] = [
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
  ];

  let [housePda, housePdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("house"), admin.publicKey.toBuffer()],
    program.programId
  );

  let [treasuryPda, treasuryPdaBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("treasury"), housePda.toBuffer()],
      program.programId
    );
  //bet details
  const betSeed = new anchor.BN(100);
  const tokenMint = new anchor.web3.PublicKey("");
  const makerOdds = new anchor.BN(1);
  const fees = 100;
  it("airdrop some sol", async () => {
    const tx = await connection.requestAirdrop(
      admin.publicKey,
      5 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(tx);
    console.log("Airdroped some 5 sol to admin", tx);
  });

  it("initialize house", async () => {
    const tx = await program.methods
      .initalizeProtocol(fees)
      .accountsPartial({
        admin: admin.publicKey,
        house: housePda,
        treasury: treasuryPda,
      })
      .signers([admin])
      .rpc();

    console.log("initialized house", tx);
  });

  it("createbet", async () => {
    // const tx = await program.methods.createBet(betSeed).accountsPartial({});
  });
});
