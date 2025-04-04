import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair } from '@solana/web3.js'
import { P04votingdapp } from '../target/types/p04votingdapp'

describe('p04votingdapp', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.P04votingdapp as Program<P04votingdapp>

  const p04votingdappKeypair = Keypair.generate()

  it('Initialize P04votingdapp', async () => {
    await program.methods
      .initialize()
      .accounts({
        p04votingdapp: p04votingdappKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([p04votingdappKeypair])
      .rpc()

    const currentCount = await program.account.p04votingdapp.fetch(p04votingdappKeypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment P04votingdapp', async () => {
    await program.methods.increment().accounts({ p04votingdapp: p04votingdappKeypair.publicKey }).rpc()

    const currentCount = await program.account.p04votingdapp.fetch(p04votingdappKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment P04votingdapp Again', async () => {
    await program.methods.increment().accounts({ p04votingdapp: p04votingdappKeypair.publicKey }).rpc()

    const currentCount = await program.account.p04votingdapp.fetch(p04votingdappKeypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement P04votingdapp', async () => {
    await program.methods.decrement().accounts({ p04votingdapp: p04votingdappKeypair.publicKey }).rpc()

    const currentCount = await program.account.p04votingdapp.fetch(p04votingdappKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set p04votingdapp value', async () => {
    await program.methods.set(42).accounts({ p04votingdapp: p04votingdappKeypair.publicKey }).rpc()

    const currentCount = await program.account.p04votingdapp.fetch(p04votingdappKeypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the p04votingdapp account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        p04votingdapp: p04votingdappKeypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.p04votingdapp.fetchNullable(p04votingdappKeypair.publicKey)
    expect(userAccount).toBeNull()
  })
})
