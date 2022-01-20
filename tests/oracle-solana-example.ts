import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { OracleSolanaExample } from '../target/types/oracle_solana_example';
import {
  Keypair,
  SystemProgram,
} from "@solana/web3.js";
import * as assert from "assert";

describe('oracle-solana-example', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.OracleSolanaExample as Program<OracleSolanaExample>;
  const payerKeypair = Keypair.fromSecretKey(
    (provider.wallet as any).payer.secretKey
  );

  before(async () => {
    const [btcPriceAddress, btcPriceBump] = anchor.utils.publicKey.findProgramAddressSync(
      [Buffer.from("BTCPrice")],
      program.programId
    );

    await assert.doesNotReject(async function() {
      await program.rpc.initBtcPrice({
        btcPriceBump: btcPriceBump
      }, {
        accounts: {
          btcPriceAccount: btcPriceAddress,
          payer: payerKeypair.publicKey,
          systemProgram: SystemProgram.programId,
        },
      });
    });

  });

  it("Works", async () => {
    assert.equal(1,1);
  });

  /*it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });*/

  
});
