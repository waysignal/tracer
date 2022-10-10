import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Tracer} from "../target/types/tracer";

describe("tracer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Tracer as Program<Tracer>;
  
  it("Is initialized!", async () => {
    // Add your test here.
    const user = (program.provider as anchor.AnchorProvider).wallet;
    const wallet_1 = anchor.web3.Keypair.generate();
    const address_1 = wallet_1.publicKey;
  
    const tx_init = await program.methods.new()
                                    .accounts({
                                      eqn: address_1,
                                      operator: user.publicKey,
                                    })
                                    .signers([wallet_1])
                                    .rpc();

    const tx = await program.methods.operate()
                                    .accounts({
                                      eqn: address_1,
                                    })
                                    .signers([])
                                    .rpc();
    console.log("Your transaction signature", tx_init);
  });
});
