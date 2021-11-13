
const anchor = require('@project-serum/anchor');
const { SystemProgram } = require('@solana/web3.js');


const main = async() => {
  console.log("🚀 Launching test...")

  // Grabs the local Solana cluster
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Gifprojectsol;
  // Create an account keypair for the program to use
  // Base account credentials
  // const baseAccount = anchor.web3.Keypair.generate();

  const getBaseAcct = async () => {
    let [baseAccount, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("ngGifme"), provider.wallet.publicKey.toBytes()],
      program.programId
    );

    return { baseAccount, bump };
  };


    let { baseAccount, bump } = await getBaseAcct();

    const trxn = await program.rpc.initList(bump, {
      accounts: {
        baseAccount,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });

    console.log("📝 Your transaction signature", trxn);
    // Fetch account data
    let account = await program.account.baseAccount.fetch(baseAccount);
    console.log("GIT TOTAL--", account.gifList.length);

    // add in a new gift 
    await program.rpc.addNewGif('https://media.giphy.com/media/NEvPzZ8bd1V4Y/giphy.gif', {
        accounts: {
          baseAccount,
          user: provider.wallet.publicKey,
        },
      });


  // getting account info again and check to see update gifList ammount
  account2 = await program.account.baseAccount.fetch(baseAccount);
  console.log('GIT TOTAL--', account2.gifList.length);

  // see that an item was added to a new list
  console.log('👀 GIF List', account2.gifList);

};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
