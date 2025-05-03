# Solvibe Solana Starter CLI

[![crates.io](https://img.shields.io/crates/v/solvibe.svg)](https://crates.io/crates/solvibe)

`solvibe` helps you kickstart your Solana development by providing handy project templates. Spend less time on boilerplate setup and more time building.

## Installation

### Prerequisites

Ensure you have Rust installed. If not, follow the official installation guide: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Install Solvibe

```bash
cargo install solvibe
```

## Usage

`solvibe` offers two main commands to streamline your workflow:

### 1. Initialize a New Project (`solvibe init`)

This command clones pre-configured starter templates directly to your machine.

**Steps:**

1.  Run the command:
    ```bash
    solvibe init
    ```
2.  Choose your desired template:
    ```
    ? What template do you want to use? ›
    ❯ Client code (Typescript or Rust)
      Fullstack Anchor program with Nextjs frontend
      Cancel
    ```
3.  Specify a directory name for your new project:
    ```
    ✔ What template do you want to use? · Client code (Typescript or Rust)
    ? Enter directory name to clone into (default: starter) › my-solana-app
    ```

**Available Templates:**

- **Client code:** Offers options for setting up either a Typescript or Rust client templates to interact with Solana programs.
  ```
  ❯ All
    Typescript Client
    Rust Client
    Back
    Cancel
  ```
- **Fullstack Anchor program with Nextjs frontend:** Provides a complete setup including an Anchor smart contract and a Next.js web frontend.

### 2. Install Solana Dependencies (`solvibe install`)

This command helps you install other essential tools commonly used in Solana development (Solana CLI, Anchor CLI, etc.).

```bash
solvibe install
```

This simplifies the setup process for your development environment.
