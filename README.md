# Ludo Game with zk Proof and Verification System

Welcome to the Ludo Game project, enhanced with a zero-knowledge proof and verification system using RISC0. This project demonstrates how to integrate cryptographic proofs into a traditional board game to ensure fair play and secure game state updates.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Game Rules](#game-rules)
- [zk Proof System](#zk-proof-system)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Introduction

This project is a digital version of the classic Ludo game, enhanced with a zero-knowledge proof system to verify each game state update. The integration of RISC0 ensures that all moves are validated without revealing the players' private information.

Once a proof is generated, it is sent to the aligned verifier to verify that the game state is valid.

## Features

- Classic Ludo gameplay
- Secure game state updates using zk proofs
- Verification system using RISC0
- User-friendly interface
- Multiplayer support

## Installation

To get started with the Ludo game, follow these steps:

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

2. Clone the repository:
   ```bash
   git clone https://github.com/jelilat/ludo-zk.git
   ```
3. Navigate to the project directory:
   ```bash
   cd ludo-zk
   ```
4. Build the project:
   ```bash
   cargo build --release
   ```
5. Run the game:
   ```bash
   cargo run --bin host
   ```

## API

The API is available at `http://127.0.0.1:3003`. Run `cargo run --bin api` to start the API.

## Usage

The game frontend is available at `https://github.com/jelilat/onchain-ludo`.

## Game Rules

The rules of the game are based on the traditional Ludo game. Players take turns rolling a die and moving their pieces around the board. The first player to get all their pieces to the home area wins.

## zk Proof System

This project uses a zero-knowledge proof system to ensure that each game state update is valid. The RISC0 framework is used to generate and verify proofs, ensuring that all moves are legitimate without revealing any private information.
