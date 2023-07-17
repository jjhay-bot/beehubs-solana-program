## The Blokc `x` Solana Foundation

##### \* Solana Developer Bootcamp

### Step to create Solana Program using Anchor

##

- anchor init `<PROJECT_NAME>` --javascript

- cd `<PROJECT_NAME>`

- solana-keygen new -o key.json <!-- 🔑 Create a local keypair -->

<!--
Wrote new keypair to key.json
pubkey: GhEtnEgMXrBJF358CxGD6S2kgd75fBz7dbVk5KYs6rbK
Save this seed phrase and your BIP39 passphrase to recover your new keypair:
dismiss cargo aerobic subway goddess fence arch jacket beef goose shock embark
==============================================================================
-->

- solana address

- solana config get

- solana-test-validator

- anchor test <!-- if passed, anchor/solana env setup -->

<!-- 😍 Write your first Solana program. -->

- open/modify and lib.rs (store data on contract/programs)

- Deploy to devnet: `solana config set --url devnet`

- solana config get

- solana airdrop 2

- solana airdrop 1 <RECIPIENT_ACCOUNT_ADDRESS> --url https://api.devnet.solana.com



```
 Now, we need to change some variables in Anchor.toml. This is where it gets a little tricky.

In Anchor.toml, change [programs.localnet] to [programs.devnet].

Then, change cluster = "localnet" to cluster = "devnet".
```

- anchor build

- solana address -k target/deploy/myepicproject-keypair.json `(update declared_id on lib.rs)`

- Now, go to Anchor.toml and under [programs.devnet] you'll see something like myepicproject = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS". Go ahead and change this id to the same id output when you run solana address -k target/deploy/myepicproject-keypair.json.

- anchor build

- anchor deploy (if didn't work use solana program deploy)

- anchor deploy --url https://api.devnet.solana.com --program-id 7Q4EY4QVj1o7MyNdzsPed1XYycvgaWmBpnishC8m4yQK

- anchor upgrade ./target/deploy/beehubs.so --program-id 7Q4EY4QVj1o7MyNdzsPed1XYycvgaWmBpnishC8m4yQK

- solana program deploy -u devnet ./target/deploy/beehubs.so

solana airdrop 4 CGZPb2KpyVucADfSoMjzfpP9m4hJHvKdXPBPYjR7uW64 --url https://api.devnet.solana.com