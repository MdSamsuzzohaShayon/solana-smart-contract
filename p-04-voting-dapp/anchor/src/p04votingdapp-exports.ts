// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import P04votingdappIDL from '../target/idl/p04votingdapp.json'
import type { P04votingdapp } from '../target/types/p04votingdapp'

// Re-export the generated IDL and type
export { P04votingdapp, P04votingdappIDL }

// The programId is imported from the program IDL.
export const P04VOTINGDAPP_PROGRAM_ID = new PublicKey(P04votingdappIDL.address)

// This is a helper function to get the P04votingdapp Anchor program.
export function getP04votingdappProgram(provider: AnchorProvider, address?: PublicKey) {
  return new Program({ ...P04votingdappIDL, address: address ? address.toBase58() : P04votingdappIDL.address } as P04votingdapp, provider)
}

// This is a helper function to get the program ID for the P04votingdapp program depending on the cluster.
export function getP04votingdappProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the P04votingdapp program on devnet and testnet.
      return new PublicKey('coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF')
    case 'mainnet-beta':
    default:
      return P04VOTINGDAPP_PROGRAM_ID
  }
}
