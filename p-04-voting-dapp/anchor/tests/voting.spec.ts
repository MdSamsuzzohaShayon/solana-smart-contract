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

  let context: anchor.AnchorProvider;
  let provider: BankrunProvider;
  let votingProgram: Program<Voting>;

  beforeAll(async () => {
    // @ts-ignore
    context = await startAnchor("", [{name: "voting", programId: votingAddress}], []);
    // @ts-ignore
    provider = new BankrunProvider(context);
    

    votingProgram = new Program<Voting>(
      IDL,
      provider
    );
  });

  it('Initialize Poll', async () => {



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

    expect(poll.pollId.toNumber()).toEqual(1);
    expect(poll.description).toEqual("What is your favorite type of pizza?");
    expect(poll.pollStart.toNumber()).toBeLessThan(poll.pollEnd.toNumber());
    

  });


  it("Initialize Candidate", async () => {

    await votingProgram.methods.initializeCandidate(
      "Smooth",
      new anchor.BN(1)
    ).rpc();

    await votingProgram.methods.initializeCandidate(
      "Crunchy",
      new anchor.BN(1)
    ).rpc();

    const [crunchyAddress ]= PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8), Buffer.from("Crunchy")],
      votingAddress
    );

    const crunchyCandidate = await votingProgram.account.candidate.fetch(crunchyAddress);
    console.log({crunchyCandidate});
    expect(crunchyCandidate.candidateName).toEqual("Crunchy");
    expect(crunchyCandidate.candidateVotes.toNumber()).toEqual(0);


    const [smoothAddress ]= PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8), Buffer.from("Smooth")],
      votingAddress
    );

    const smoothCandidate = await votingProgram.account.candidate.fetch(smoothAddress);
    console.log({smoothCandidate}); 
    expect(smoothCandidate.candidateName).toEqual("Smooth");
    expect(smoothCandidate.candidateVotes.toNumber()).toEqual(0);
  });

  it("Vote", async () => {

    await votingProgram.methods.vote(
      "Smooth",
      new anchor.BN(1)
    ).rpc();

    const [smoothAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8), Buffer.from("Smooth")],
      votingAddress
    );

    const smoothCandidate = await votingProgram.account.candidate.fetch(smoothAddress);
    console.log({smoothCandidate});
    expect(smoothCandidate.candidateVotes.toNumber()).toEqual(1);
    


  });
});



