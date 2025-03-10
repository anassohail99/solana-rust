import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import fs from "fs";
import { Day5 } from "../target/types/day_5";
let idl = JSON.parse(fs.readFileSync("target/idl/day_5.json", "utf-8"));

describe("deployed", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Change this to be your programID
  const program = new Program(idl, anchor.getProvider());
  // console.log(program.programId);

  // const program = anchor.workspace.Day4 as Program<Day5>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
