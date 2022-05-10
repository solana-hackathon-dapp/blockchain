import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BoContract } from "../target/types/bo_contract";

describe("bo-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BoContract as Program<BoContract>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
