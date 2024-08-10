import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { randomBytes } from "crypto";
import {
  createMint,
  mintTo,
  getOrCreateAssociatedTokenAccount,
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { assert } from "chai";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const program = anchor.workspace.Escrow as Program<Escrow>;

  let [maker, taker, mintA, mintB] = [
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
  ];

  const seed = new anchor.BN(randomBytes(8));
  let maker_ata_mint_a;
  let taker_ata_mint_b;
  let [escrow, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("escrow"),
      maker.publicKey.toBuffer(),
      // seed.toArrayLike(Buffer, "le", 8),
      seed.toBuffer().reverse(),
    ],
    program.programId
  );
  it("setting up accounts and mints", async () => {
    const tx1 = await connection.requestAirdrop(
      maker.publicKey,
      LAMPORTS_PER_SOL * 5
    );

    await connection.confirmTransaction(tx1);

    const tx2 = await connection.requestAirdrop(
      taker.publicKey,
      LAMPORTS_PER_SOL * 5
    );

    await connection.confirmTransaction(tx2);

    await createMint(connection, maker, maker.publicKey, null, 0, mintA);
    await createMint(connection, taker, taker.publicKey, null, 0, mintB);

    maker_ata_mint_a = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      mintA.publicKey,
      maker.publicKey
    );

    taker_ata_mint_b = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      mintB.publicKey,
      taker.publicKey
    );

    await mintTo(
      connection,
      maker,
      mintA.publicKey,
      maker_ata_mint_a.address,
      maker,
      1000
    );

    await mintTo(
      connection,
      taker,
      mintB.publicKey,
      taker_ata_mint_b.address,
      taker,
      1000
    );
  });

  it("make escrow", async () => {
    const vault = getAssociatedTokenAddressSync(
      mintA.publicKey,
      escrow,
      true,
      TOKEN_PROGRAM_ID
    );
    console.log("vault ", vault.toString());
    console.log("escrow", escrow.toString());
    const tx = await program.methods
      .make(seed, new anchor.BN(100), new anchor.BN(200))
      .accounts({
        maker: maker.publicKey,
        mintA: mintA.publicKey,
        mintB: mintB.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc();
    console.log("signature is", tx);
    assert(
      (await connection.getTokenAccountBalance(vault)).value.uiAmount == 100
    );
  });
});
