import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { Voting } from '../target/types/voting';
import { BankrunProvider, startAnchor } from 'anchor-bankrun';

const IDL = require('../target/idl/voting.json');

console.log({IDL});


// Copy the program ID from the IDL file or lib.rs file
const votingAddress = new PublicKey("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");


describe('Voting', () => {

  it('Initialize Poll', async () => {

    const context = await startAnchor("", [{name: "voting", programId: votingAddress}], []);
    const provider = new BankrunProvider(context);
    

    const votingProgram = new Program<Voting>(
      IDL,
      provider
    );

    await votingProgram.methods.initializePoll(
      new anchor.BN(1), 
      "What is your favorite type of pizza?", 
      new anchor.BN(0), 
      new anchor.BN(1744086723))
      .rpc();


    const [pollAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
      votingAddress
    );

    const poll = await votingProgram.account.poll.fetch(pollAddress);
    console.log({poll});
    

  });
});



