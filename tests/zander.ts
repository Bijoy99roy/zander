import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Zander } from "../target/types/zander";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";
import crypto from "crypto";

describe("zander", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.zander as Program<Zander>;

  const provider = anchor.getProvider()

  const admin = anchor.web3.Keypair.generate()
  const verifier1 = anchor.web3.Keypair.generate()
  const verifier2 = anchor.web3.Keypair.generate()
  const verifier3 = anchor.web3.Keypair.generate()

  const publisher = anchor.web3.Keypair.generate()

  async function getPda(seeds) {
    const [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    return {pda, bump}
  }

  async function getAirdrop(
    publicKey: anchor.web3.PublicKey,
    amount: number = 100 * anchor.web3.LAMPORTS_PER_SOL
  ){
    const airdropTxn = await provider.connection.requestAirdrop(
      publicKey,
      amount
    );

    await provider.connection.confirmTransaction(airdropTxn);
  }

  before(async ()=>{
    await getAirdrop(admin.publicKey);
    await getAirdrop(verifier1.publicKey);  
    await getAirdrop(verifier2.publicKey);  
    await getAirdrop(verifier3.publicKey);  
    await getAirdrop(publisher.publicKey); 
  })

  it("Initialized treasury", async () => {

    const {pda: treasury} = await getPda([Buffer.from("treasury")])
    await program.methods.initializeTreasury()
    .accounts({
      admin: admin.publicKey,
      treasury: treasury
    }).signers([admin])
    .rpc()
  });

  it("become verifier", async () => {

    const {pda: verifier1Pda} = await getPda([Buffer.from("verifier"), verifier1.publicKey.toBuffer()])
    const {pda: verifier1StakeVault} = await getPda([Buffer.from("stake_vault"), verifier1.publicKey.toBuffer()])

    const {pda: verifier2Pda} = await getPda([Buffer.from("verifier"), verifier2.publicKey.toBuffer()])
    const {pda: verifier2StakeVault} = await getPda([Buffer.from("stake_vault"), verifier2.publicKey.toBuffer()])

    const {pda: verifier3Pda} = await getPda([Buffer.from("verifier"), verifier3.publicKey.toBuffer()])
    const {pda: verifier3StakeVault} = await getPda([Buffer.from("stake_vault"), verifier3.publicKey.toBuffer()])

    const stakeLamports = new anchor.BN(6*LAMPORTS_PER_SOL)

    await program.methods.becomeVerifier(stakeLamports)
    .accounts({
      user: verifier1.publicKey,
      verifier: verifier1Pda,
      stakeVault: verifier1StakeVault
    }).signers([verifier1])
    .rpc()

    await program.methods.becomeVerifier(stakeLamports)
    .accounts({
      user: verifier2.publicKey,
      verifier: verifier2Pda,
      stakeVault: verifier2StakeVault
    }).signers([verifier2])
    .rpc()

    await program.methods.becomeVerifier(stakeLamports)
    .accounts({
      user: verifier3.publicKey,
      verifier: verifier3Pda,
      stakeVault: verifier3StakeVault
    }).signers([verifier3])
    .rpc()

    const verifier1StakeVaultBalance = await provider.connection.getBalance(verifier1StakeVault);
    const stakeVaultRent = await provider.connection.getMinimumBalanceForRentExemption(0)
    const verifier1StakeVaultBalanceWithoutRent = verifier1StakeVaultBalance - stakeVaultRent;

    const verifier2StakeVaultBalance = await provider.connection.getBalance(verifier2StakeVault);
    const verifier2StakeVaultBalanceWithoutRent = verifier2StakeVaultBalance - stakeVaultRent;

    const verifier3StakeVaultBalance = await provider.connection.getBalance(verifier3StakeVault);
    const verifier3StakeVaultBalanceWithoutRent = verifier3StakeVaultBalance - stakeVaultRent;

    assert.equal(verifier1StakeVaultBalanceWithoutRent.toString(), stakeLamports.toString())
    assert.equal(verifier2StakeVaultBalanceWithoutRent.toString(), stakeLamports.toString())
    assert.equal(verifier3StakeVaultBalanceWithoutRent.toString(), stakeLamports.toString())

    const verifier1PdaAccount = await program.account.verifier.fetch(verifier1Pda);

    console.log(verifier1PdaAccount)
  });

  it("post news", async()=>{

    const ipfsUrl = "ipfs://kjsdhfweurlbskvjelsuif"
    const headline = "A robbery has happened in citybank by some flying donuts"
    let hexString = crypto.createHash('sha256').update(ipfsUrl, 'utf-8').digest('hex');
    let content_seed = Uint8Array.from(Buffer.from(hexString, 'hex'));

    const {pda: news} = await getPda([Buffer.from("news"), publisher.publicKey.toBuffer(), content_seed]);

    await program.methods.postNews(ipfsUrl, headline)
    .accounts({
      publisher:publisher.publicKey,
      news: news
    })
    .signers([publisher])
    .rpc();


  });

  it("cast vote", async ()=>{
    const {pda: verifier1Pda} = await getPda([Buffer.from("verifier"), verifier1.publicKey.toBuffer()])

    const {pda: verifier2Pda} = await getPda([Buffer.from("verifier"), verifier2.publicKey.toBuffer()])

    const {pda: verifier3Pda} = await getPda([Buffer.from("verifier"), verifier3.publicKey.toBuffer()])

    

    const ipfsUrl = "ipfs://kjsdhfweurlbskvjelsuif"
    const headline = "A robbery has happened in citybank by some flying donuts"
    let hexString = crypto.createHash('sha256').update(ipfsUrl, 'utf-8').digest('hex');
    let content_seed = Uint8Array.from(Buffer.from(hexString, 'hex'));

    const {pda: news} = await getPda([Buffer.from("news"), publisher.publicKey.toBuffer(), content_seed]);

    const {pda: verifier1VoteRecord} = await getPda([Buffer.from("vote"), news.toBuffer(), verifier1.publicKey.toBuffer()])
    const {pda: verifier2VoteRecord} = await getPda([Buffer.from("vote"), news.toBuffer(), verifier2.publicKey.toBuffer()])
    const {pda: verifier3VoteRecord} = await getPda([Buffer.from("vote"), news.toBuffer(), verifier3.publicKey.toBuffer()])

    await program.methods.voteNews({ false: {} })
    .accounts({
      voter: verifier1.publicKey,
      verifier: verifier1Pda,
      news: news,
      voteRecord: verifier1VoteRecord
    }
    ).signers([verifier1])
    .rpc()

    await program.methods.voteNews({ false: {} })
    .accounts({
      voter: verifier2.publicKey,
      verifier: verifier2Pda,
      news: news,
      voteRecord: verifier2VoteRecord
    }
    ).signers([verifier2])
    .rpc()

    await program.methods.voteNews({ false: {} })
    .accounts({
      voter: verifier3.publicKey,
      verifier: verifier3Pda,
      news: news,
      voteRecord: verifier3VoteRecord
    }
    ).signers([verifier3])
    .rpc()

    const verifier1VoteRecordAccount = await program.account.voteRecord.fetch(verifier1VoteRecord);

    console.log(verifier1VoteRecordAccount)
  });

  it("finalize votes", async ()=>{
    const remainingAccounts=[]
    const {pda: treasury} = await getPda([Buffer.from("treasury")])
    const ipfsUrl = "ipfs://kjsdhfweurlbskvjelsuif"
    const headline = "A robbery has happened in citybank by some flying donuts"
    let hexString = crypto.createHash('sha256').update(ipfsUrl, 'utf-8').digest('hex');
    let content_seed = Uint8Array.from(Buffer.from(hexString, 'hex'));
    const {pda: news} = await getPda([Buffer.from("news"), publisher.publicKey.toBuffer(), content_seed]);
    const voteRecords = await program.account.voteRecord.all([
              {
                memcmp: {
                  offset: 8 + 32, // discriminator
                  bytes: news.toBase58(),
                },
              },
            ]);

    for (const vr of voteRecords) {
      const voter = vr.account.verifier;
      console.log(voter)
      const {pda: stakeVault} = await getPda([Buffer.from("stake_vault"), voter.toBuffer()]);
      const {pda: verifierPda} = await getPda([Buffer.from("verifier"), voter.toBuffer()]);
      remainingAccounts.push({
        pubkey: vr.publicKey,
        isWritable: true,
        isSigner: false,
      });
      remainingAccounts.push({
        pubkey: stakeVault,
        isWritable: true,
        isSigner: false,
      });
      remainingAccounts.push({
        pubkey: verifierPda,
        isWritable: true,
        isSigner: false,
      });
    }

    await program.methods.finalizeNews()
    .accounts({
      admin: admin.publicKey,
      news: news,
      treasury: treasury
    })
    .remainingAccounts(remainingAccounts)
    .signers([admin])
    .rpc();

    const newsAccount = await program.account.news.fetch(news);

    console.log(newsAccount)

  });
});
