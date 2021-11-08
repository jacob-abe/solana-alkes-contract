const anchor = require('@project-serum/anchor')

// Need the system program, will talk about this soon.
const { SystemProgram } = anchor.web3

const main = async () => {
  console.log('🚀 Starting test...')

  const provider = anchor.Provider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.SolanaAlkesContract;
  const baseAccount = anchor.web3.Keypair.generate();
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Word Count', account.totalWords.toString())

  await program.rpc.addWord("Some word", {
    accounts: {
      baseAccount: baseAccount.publicKey,
    },
  });
  
  // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Word Count', account.totalWords.toString())

  console.log('👀 Word List', account.contributerList)
}

const addWord =async(program, baseAccount) =>{
  await program.rpc.addWord({
    accounts: {
      baseAccount: baseAccount.publicKey,
    },
  });
}

const runMain = async () => {
  try {
    await main()
    process.exit(0)
  } catch (error) {
    console.error(error)
    process.exit(1)
  }
}

runMain()
