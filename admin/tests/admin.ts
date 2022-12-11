import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey"
import { assert } from "chai"
import { Admin } from "../target/types/admin"

describe("admin", () => {
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.Admin as Program<Admin>

  const adminConfig = findProgramAddressSync(
    [Buffer.from("admin")],
    program.programId
  )[0]

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        adminAccount: adminConfig,
      })
      .rpc()
  })

  it("Update bool!", async () => {
    const tx = await program.methods
      .updateBool(false)
      .accounts({
        adminAccount: adminConfig,
      })
      .rpc()

    assert.strictEqual(
      (await program.account.config.fetch(adminConfig)).bool,
      false
    )
  })

  it("Update u8!", async () => {
    const tx = await program.methods
      .updateFirstNumber(100)
      .accounts({
        adminAccount: adminConfig,
      })
      .rpc()

    assert.strictEqual(
      (await program.account.config.fetch(adminConfig)).firstNumber,
      100
    )
  })

  it("Update u64!", async () => {
    const tx = await program.methods
      .updateSecondNumber(new anchor.BN(200))
      .accounts({
        adminAccount: adminConfig,
      })
      .rpc()

    assert.strictEqual(
      (await program.account.config.fetch(adminConfig)).secondNumber.toNumber(),
      200
    )
  })

  it("Update Admin!", async () => {
    const keypair = anchor.web3.Keypair.generate()

    const tx = await program.methods
      .updateAuth(keypair.publicKey)
      .accounts({
        adminAccount: adminConfig,
      })
      .rpc()

    assert.strictEqual(
      (await program.account.config.fetch(adminConfig)).auth.toString(),
      keypair.publicKey.toString()
    )
  })
})
