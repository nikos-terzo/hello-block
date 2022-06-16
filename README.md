# hello-block
Mini project to familiarize ourselves with Rust and ethers rs.
The application's purpose is to send messages between clients.

## Architecture
Each client will have a contract uploaded which will receive the messages.
The contract returns the messages only to this client.
The clients can send messages to other clients using the other's client uploaded contract.

## Environment
* Running with a [ganache-cli](https://hub.docker.com/r/trufflesuite/ganache-cli/) docker image open to use as a test blockchain.
I decided that in order to simulate how it would work in a public blockchain. More specifically, if we run ganache from ethers rs then we already have all the data available in the same application (eg. all the account private keys).
In order to make this work without updating the account keys every-time the container restarts, one should use --db argument for the entrypoint of the ganache-cli container (note that also the --mnemonic is needed).
* To run on your environment, change Chatter1.ini, Chatter2.ini to match it. The logic is that each user has its private key and two public keys for each contact. The contact's contract key, where the user sends messages and the contact's account key, from where the user receives the messages. When you first run the ganache host, the contracts have not yet been created. You can create and upload them by running `cargo test contract_uploads -- --ignored --nocapture`. The public key of the contract will show. If we assume we run it for Chatter1-conf.ini, that is the contract that chatter-1 uploads, the printed address should be written in Chatter2-conf.ini for the chatter2 to send messages to chatter1.
* I have vim as a preferred git editor, and I install it in Dockerfile, feel free to change the line.

## Limitations
Initially, the messages will be stored as simple Strings, a vulnerability which we will resolve later on.