const anchor = require('@project-serum/anchor');
const { expect } = require('chai');
const { SystemProgram } = anchor.web3;

// test case for initalizing list, adding new gif to list
describe('gif-project-sol', ()=>{
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Gifprojectsol;

  const getBaseAcct = async () => {
    let [baseAccount, bump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from("ngGif2"), 
      provider.wallet.publicKey.toBytes()],
      program.programId
    );
    return { baseAccount, bump };
  };


  const newGifList = async() => {
    let { baseAccount, bump } = await getBaseAcct();

    await program.rpc.initList(bump, {
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
      console.log('giflist count', account.bump.toString());
      console.log('Gif-accout', account.gifList.length);
      // expect(account.gifList.length).eq(0);
    });


    it('adds new gif to list', async ()=>{
        let { baseAccount } = await getBaseAcct();
        let account = await program.account.baseAccount.fetch(baseAccount);
        console.log("GIT TOTAL--", account.gifList.length);
    
        // add in a new gift 
        await program.rpc.addNewGif("https://giphy.com/clips/grinchmovie-grinch-illumination-movie-Y0NhkIdyT3TVBDWLkG", {
            accounts: {
              baseAccount,
              listOwner: provider.wallet.publicKey,
              user: provider.wallet.publicKey,
            },    
      })

      account = await program.account.baseAccount.fetch(baseAccount);
      console.log('myGifs', account.gifList);
      // expect(account.gifList.length).eq(1)
    });


    // it("deletes gif list", async () => {
    //   let { baseAccount } = await getBaseAcct();
    //   let account = await program.account.baseAccount.fetch(baseAccount);
    //   console.log(provider);
    //   // console.log(program)
    //     console.log(account.gifList);
    //   await program.rpc.deleteList({
    //     accounts: {
    //       baseAccount,
    //       user: provider.wallet.publicKey,
    //       systemProgram: SystemProgram.programId,
    //     },
    //   });
    //  account = await program.account.baseAccount.fetch(baseAccount);
    //   console.log('account', account.gifList);
    //   // expect(account.gifList.length).eq(1);
    // });

});

