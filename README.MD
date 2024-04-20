# How to Run Rush Hour Applications

This repository contains two applications related to the Rush Hour game: a Next.js application for the user interface (front) and a Solana program to solve the game (solver).

### Requirements

- Node.js and npm installed
- A modern web browser
- Install Phantom wallet in your browser (solana / adapter compatible)
- Solana CLI installed to run the Solana program
- Balance in your account a a local cluster running (we will cover it later)

## Setup

- Clone this repository
- Navigate to the front application directory:
  `cd front`
- Install dependencies: `yarn install`
- Navigate to the solver directory: `cd ../solver`

## Set up a local validator cluster

- `solana-test-validator` start the cluster.
- (optional) inspect the logs:
  - In other terminal window: `solana logs`

## Set up a Solana wallet.

`solana-keygen new -o ./keypair.json` to generate a new wallet address.

`solana airdrop <number>` to airdrop a ballance to your account.

## Install Solana program dependencies:

- to build the program solana binaries `cargo build-sbf`
- to deploy the program to solana localnet: `solana program deploy ./target/deploy/rush_solver_solana.so`

Once you have sufficient balance in your account to deploy the program (you need to pay fee for 2 years), your program will be deployed successfully.

## Running the applications

- Get your program environment variable and add it to nextjs .env
- If you are running it locally, add the cluster endpoint to: `"http://127.0.0.1:8899"`

### Frontend (Next.js)

In the front application directory, start the development server:
`yarn dev`

Open your web browser and go to http://localhost:3000 to view the Rush Hour game user interface.

### Usage

In the frontend (Next.js), interact with the Rush Hour game as a regular player. In the Solana program (solver), observe the game solution being calculated and displayed.
