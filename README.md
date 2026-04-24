# Project Title

Time Attestation – A Soroban Smart Contract for Proof-of-Time Worked on Stellar

## Project Vision

This project provides a ** decentralized solution for tracking and attesting time worked** on the Stellar blockchain. It demonstrates:
- How to write a Soroban smart contract in Rust
- How to manage persistent storage (tracking time logs per user)
- How to handle user authentication in smart contracts
- How to implement admin-only operations (attestation)
- How to deploy and interact with contracts on Stellar Testnet

The goal is to provide a transparent, immutable record of work hours that can be verified by anyone on the blockchain.

---

## Description

A Soroban smart contract dApp that allows workers to **log their time worked** and receive **official attestation** from an admin on Stellar Testnet. Designed for freelancers, contractors, and teams who need verifiable proof of work hours on-chain.

---

## Features

### 1. Time Logging
- Users can log time with hours and description
- Each log is timestamped with the current ledger timestamp
- All logs are stored permanently on-chain

### 2. Time Retrieval
- Get all time logs for a specific user
- View total hours logged across all entries

### 3. Admin Attestation
- Admin can sign off (attest) specific time logs
- Attested logs are marked as verified on-chain

### 4. On-chain Transparency
- All time logs stored permanently on Stellar blockchain
- Anyone can verify logs and attestation status
- Immutable audit trail

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CA7U57H624ZXH7OMWOIQLKBBPPYIXOXHCHTMAXP6NHLIGEHXRF72AON4](https://stellar.expert/explorer/testnet/tx/19cd4d884a9898d07256407b36c105ef2b931a8297ba787856517d061e297fc4)

![screenshot](https://i.ibb.co/JR1jchGv/image.png)

---

## Future Scopes

### 1. Multi-Admin Support
- Add multiple admin signers for attestation
- Require N-of-M admin signatures for attestation

### 2. Tokenized Rewards
- Issue tokens to users based on attested hours
- Create an incentive mechanism for accurate time tracking

### 3. Client Dashboard
- Build a React/web frontend for easier time logging
- Display visualizations of logged hours and attestation status

### 4. Integration with Stellar Classic
- Enable attestation by Stellar Classic accounts
- Cross-chain verification capabilities

### 5. Rate Limiting
- Prevent spam by limiting logs per user per time period
- Add cooldown between log entries

### 6. Project/Client Management
- Associate time logs with specific projects or clients
- Generate reports per project

---

## Profile

- **Name:** :q1956299-art
