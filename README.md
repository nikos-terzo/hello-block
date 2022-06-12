# hello-block
Mini project to familiarize ourselves with Rust and ethers rs.
The application's purpose is to send messages between clients.

## Architecture
Each client will have a contract uploaded which will receive the messages.
The contract returns the messages only to this client.
The clients can send messages to other clients using the other's client uploaded contract.

## Environment
* Running with a [ganache-cli](https://hub.docker.com/r/trufflesuite/ganache-cli/) docker image open to use as a test blockchain.
* I have vim as a preferred git editor, and I install it in Dockerfile, feel free to change the line.

## Limitations
Initially, the messages will be stored as simple Strings, a vulnerability which we will resolve later on.