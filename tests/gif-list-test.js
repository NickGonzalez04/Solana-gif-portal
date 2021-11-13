const anchor = require('@project-serum/anchor');
const { SystemProgram } = require('@solana/web3.js');
const { expect} = require('chai');

describe('gif-project-sol', ()=>{

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Gifprojectsol;

  const getBaseAcct = async () => {
    let [baseAccount, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("ngGifme"), provider.wallet.publicKey.toBytes()],
      program.programId
    );

    return { baseAccount, bump };
  };



  const newGifList = async() => {
    let { baseAccount, bump } = await getBaseAcct();

    const trxn = await program.rpc.initList(bump, {
      accounts: {
        baseAccount,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });

    let account = await program.account.baseAccount.fetch(baseAccount);
    return { baseAccount, account };
  };


    it('Creates new gif list', async () => {
      console.log('🚀 new gif list coming ...');
      const { account } = await newGifList();
      console.log('giflist count', account.gifList.length);
      expect(account.gifList.length).eq(0);
    })

    it('adds new gif to list', async ()=>{
      let { baseAccount } = await getBaseAcct();
      let account = await program.account.baseAccount.fetch(baseAccount);
      console.log("GIT TOTAL--", account.gifList.length);
  
      // add in a new gift 
      await program.rpc.addNewGif('https://media.giphy.com/media/NEvPzZ8bd1V4Y/giphy.gif', {
          accounts: {
            baseAccount,
            user: provider.wallet.publicKey,
          },
          
    })
    account = await program.account.baseAccount.fetch(baseAccount);
    expect(account.gifList.length).eq(1)
  });
});
