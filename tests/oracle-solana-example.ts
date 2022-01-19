import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { OracleSolanaExample } from '../target/types/oracle_solana_example';

describe('oracle-solana-example', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.OracleSolanaExample as Program<OracleSolanaExample>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
