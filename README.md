# DCA Agent example

This Dollar Cost Average agent runs on an timer and swaps a fixed amount of in tokens to a variable amount of out tokens every X seconds. The agent runs as a smart contract on the [Internet Computer](https://internetcomputer.org) and swaps ERC-20 tokens using [Uniswap](https://app.uniswap.org/).

The backend consists of a Rust canister that uses the [ic-alloy](https://github.com/ic-alloy) library to interact with the Ethereum blockchain. The frontend is built with React and Vite.

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url] [![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url] [![MIT License][license-shield]](LICENSE)

> [!TIP] 
> This is an example app. Use this repository as a starting point for building your own timer based Ethereum agents on the Internet Computer.

![](./media/screenshot.png)

## Setup

There are two main ways to set up the dev environment:

### 1. Using a VS Code Dev Container

The dev containers extension lets you use a Docker container as a full-featured
development environment. This repository includes a dev container configuration
that you can use to open the project with all the necessary tools and
dependencies pre-installed.

Pre-requisites:

- [Docker](https://www.docker.com/products/docker-desktop)
- [Visual Studio Code](https://code.visualstudio.com/)
- [Dev Containers Extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

Once Docker, Visual Studio Code and the Dev Containers Extension are installed,
you can open the project in a container by clicking the button below:

[![Open locally in Dev Containers](https://img.shields.io/static/v1?label=Dev%20Containers&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/ic-alloy/ic-alloy-dca)

### 2. Setup manually

Pre-requisites:

- [Local Internet Computer dev environment](https://internetcomputer.org/docs/current/developer-docs/backend/rust/dev-env)
- [pnpm](https://pnpm.io/installation)

Once you have the prerequisites installed, you can clone this repository and run
the project.

## Running the project

### 1. Start the Internet Computer

```bash
dfx start --background
```

### 2. Deploy the evm-rpc canister

```
dfx deploy evm_rpc
```

### 3. Deploy the DCA agent canister

The agent canister is deployed using a script. Edit the deploy script to
change the default values.

```
bash scripts/deploy-agent.sh
```

Default values:

#### `owner`

The principal ID of the agent owner. You can get the principal ID by running:

Default value: `$(dfx identity get-principal)`

#### `token_in_address`

The Ethereum address of the token to swap from. You can get the address from Etherscan.

Default value: `0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238`

#### `token_in_name`

The name of the token to swap from.

Default value: `USDC`

#### `token_out_address`

The Ethereum address of the token to swap to. You can get the address from Etherscan.

Default value: `0xfff9976782d46cc05630d1f6ebab18b2324d6b14`

#### `token_out_name` 

The name of the token to swap to.

Default value: `WETH`

#### `fee`

The fee tier of the Uniswap pool, used to determine the correct pool contract in which to execute the swap. Uniswap v3 protocol has the 1%, 0.3%, 0.05%, and 0.01% fee tiers. 

Allowed values: `10000`, `3000`, `500`, `100`

#### `amount_in`

The amount of tokens to swap from. For the USDC token, the default amount represents 0.1 USDC. USDC uses fewer decimals than most tokens, so the amount is lower than for other tokens.

Default value: `100000`

#### `slippage`

The maximum slippage percentage allowed for the swap. If the slippage is higher than the allowed percentage, the swap is reverted.

Default value: `5` (5%)

#### `interval`

The interval in seconds between each swap.

Default value: `3600` (1 hour)

### 4. Deploy the frontend

```
pnpm install
dfx deploy frontend
```

## Usage

### Transfer tokens to the agent

Before you start the agent, you need to transfer some tokens to the agent canister, both the "in" token and some of the base chain token to pay for the gas fees. The agent Ethereum address is displayed in the frontend.

### Start the agent

```bash
bash scripts/start.sh
```

### Stop the agent

```bash
bash scripts/stop.sh
```
### Check the agent status

Access the agent status by visiting the frontend canister URL.

### Transfer tokens from the agent

At any point, you can transfer the remaining tokens from the agent canister back to your wallet.

```bash
bash scripts/transfer-in-token.sh
bash scripts/transfer-out-token.sh
bash scripts/transfer-base-token.sh
```

## Develop

During development, you can run the frontend with hot reloading using Vite.

```bash
pnpm run dev
```

## Contributors

<!-- readme: collaborators,contributors -start -->
<table>
 <tbody>
  <tr>
            <td align="center">
                <a href="https://github.com/kristoferlund">
                    <img src="https://avatars.githubusercontent.com/u/9698363?v=4" width="100;" alt="kristoferlund"/>
                    <br />
                    <sub><b>Kristofer</b></sub>
                </a>
            </td>
  </tr>
 <tbody>
</table>
<!-- readme: collaborators,contributors -end -->

## License

This project is licensed under the MIT License. See the LICENSE file for more
details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you
have any suggestions or improvements.

[contributors-shield]:
  https://img.shields.io/github/contributors/ic-alloy/ic-alloy-dca.svg?style=for-the-badge
[contributors-url]: https://github.com/ic-alloy/ic-alloy-dca/graphs/contributors
[forks-shield]:
  https://img.shields.io/github/forks/ic-alloy/ic-alloy-dca.svg?style=for-the-badge
[forks-url]: https://github.com/ic-alloy/ic-alloy-dca/network/members
[stars-shield]:
  https://img.shields.io/github/stars/ic-alloy/ic-alloy-dca?style=for-the-badge
[stars-url]: https://github.com/ic-alloy/ic-alloy-dca/stargazers
[issues-shield]:
  https://img.shields.io/github/issues/ic-alloy/ic-alloy-dca.svg?style=for-the-badge
[issues-url]: https://github.com/ic-alloy/ic-alloy-dca/issues
[license-shield]:
  https://img.shields.io/github/license/ic-alloy/ic-alloy-dca.svg?style=for-the-badge
