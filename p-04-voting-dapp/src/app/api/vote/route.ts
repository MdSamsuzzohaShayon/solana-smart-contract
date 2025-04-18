// https://solana.com/developers/guides/advanced/actions#get-response-body
import * as anchor from '@coral-xyz/anchor';
import { ActionGetResponse, ActionPostRequest, ACTIONS_CORS_HEADERS, createPostResponse, LinkedAction } from "@solana/actions";
import { Connection, PublicKey, Transaction } from "@solana/web3.js";
import { Voting } from "../../../../anchor/target/types/voting";
import { Program } from "@coral-xyz/anchor";

const IDL = require('../../../../anchor/target/idl/voting.json');

export const OPTIONS = {
  method: "GET, POST",
  headers: ACTIONS_CORS_HEADERS
};

export async function GET(request: Request) {
  const actionMetadata: ActionGetResponse = {
    type: "action",
    icon: "https://perfectsnacks.com/cdn/shop/articles/shutterstock_1550625911.jpg?v=1717833130",
    title: "Vote for your favorite type of peanut butter!",
    description: "Vote between crunchy and smooth peanut butter.",
    label: "Vote",
    disabled: false,
    links: {
      actions: [
        {
          label: "Vote for Crunchy",
          href: "http://localhost:3000/api/vote/?candidate=Crunchy"
        },
        {
          label: "Vote for Smooth",
          href: "http://localhost:3000/api/vote/?candidate=Smooth"
        },
        {
          label: "Abstain",
          href: "http://localhost:3000/api/vote/?candidate=Abstain"
        }
      ] as LinkedAction[]
    }
  };

  // return new Response(JSON.stringify(actionMetadata), {
  //   status: 200,
  //   headers: { "Content-Type": "application/json" }
  // });
  return Response.json(actionMetadata, {headers: ACTIONS_CORS_HEADERS});
}


// Get the action dialog
// https://dial.to/?action=solana-action:http://localhost:3000/api/vote


export async function POST(request: Request) {
  const url = new URL(request.url);
  const candidate = url.searchParams.get("candidate");
  if(candidate !== "Crunchy" && candidate !== "Smooth"){
    return Response.json("Invalid candidate", 
      {status: 400, headers: ACTIONS_CORS_HEADERS}); 
  }

  const connection = new Connection('http://127.0.0.1:8899', 'confirmed');
  const program: Program<Voting> = new Program(IDL, {connection});
  const body: ActionPostRequest = await request.json();
  let voter;

  try {
    voter = new PublicKey(body.account);
  } catch (error) {
    return Response.json("Invalid account", {status: 400, headers: ACTIONS_CORS_HEADERS});
  }


  const instruction  = await program.methods
  .vote(candidate, new anchor.BN(1))
  .accounts({signer: voter,})
  .instruction();


  const blockhash = await connection.getLatestBlockhash();
  const transaction = new Transaction({
    feePayer: voter,
    blockhash: blockhash.blockhash,
    lastValidBlockHeight: blockhash.lastValidBlockHeight,
  })
  .add(instruction);

  const response = await createPostResponse({
    fields:{
      type: "transaction",
      transaction: transaction,
    }
  });

  return Response.json(response, {headers: ACTIONS_CORS_HEADERS});
  
}



// incomplete
// Till - https://youtu.be/amAq-WHAFs8?t=8791

 