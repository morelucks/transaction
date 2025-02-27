# Transaction struct in Rust

This project defines a Transaction struct in Rust, representing a blockchain transaction, along with a Network enum to specify different blockchain networks.

# Overview

Transactions are cryptographically signed instructions from accounts. An account will initiate a transaction to update the state of the Ethereum network. The simplest transaction is transferring ETH from one account to another. Transactions, which change the state of the EVM, need to be broadcast to the whole network. Any node can broadcast a request for a transaction to be executed on the EVM; after this happens, a validator will execute the transaction and propagate the resulting state change to the rest of the network.

Transactions require a fee and must be included in a validated block. To make this overview simpler, we'll cover gas fees and validation elsewhere.
