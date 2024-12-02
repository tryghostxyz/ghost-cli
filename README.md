# Ghost CLI

`ghost-cli` is a command-line tool for interacting with the Ghost server.

You can now fork and edit graphs locally which means you can use any editor (and AI tool!) to work on your indexers.

## Installation

1. Ensure that Rust and Cargo are installed on your system. If not, install them
   from [https://rustup.rs/](https://rustup.rs/).

2. Clone the Ghost CLI repository:
   ```bash
   git clone https://github.com/tryghostxyz/ghost-cli.git
   cd ghost-cli
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

4. The binary will be available at `target/release/ghost`.

5. (Optional) Move the binary to your `$PATH` for easy access:
   ```bash
   mv ./target/release/ghost /usr/local/bin/
   ```

## Usage

### Configure Ghost API (Required)

Set up your Ghost API key by running the following command:

```bash
ghost configure <API_KEY>
```

To generate an API key, visit [Ghost Graphs](https://app.ghostlogs.xyz/graphs) and click the "API Key" button.
You must use an admin API key to configure.

### Create a Ghost Graph

Create your first Ghost graph by specifying the chain and directory:

```bash
ghost create --chain <chainId or slug> <directory>
```

Available chains:

```
- eth-mainnet
- eth-sepolia
- base-mainnet
- base-sepolia
- bera-testnet
- blast-mainnet
- abstract-testnet
- uni-testnet
```

```bash
ghost events --api_key <ETHERSCAN_API_KEY> --address <CONTRACT_ADDRESS> 
```

This will return the events from a verified contract that you can be used in your `events.sol` file.

Example:

```bash
ghost create --chain bera-testnet honeypot-finance
```

This will generate a new directory with the following files:

- `schema.sol`
- `events.sol`
- `config.json` (which includes the graph ID and version ID for future commands)

### Code Generation

After modifying `schema.sol` and `events.sol`, generate the necessary indexer and related files:

```bash
ghost codegen
```

### Compile Graph

Compile the graph by sending `indexer.sol` to the Ghost server for validation:

```bash
ghost compile
```

If any errors occur, they will be displayed. Ensure that the code passes compilation before deploying. If you make
changes to `events.sol` or `schema.sol`, re-run the `codegen` command.

### Deploy Graph

Deploy a successfully compiled graph to the Ghost server:

```bash
ghost deploy
```

### List Graphs

To view a list of your active or draft graphs:

```bash
ghost list
```

### Fork an Existing Graph

Fork an existing graph and create a new directory:

```bash
ghost fork --id <graph_id> <directory>
```

### Fork an Existing Graph And Delete The Old Graph

Fork an existing graph in the current Ghost directory and delete the old graph

```bash
ghost fork --replace --delete .
```

### Delete an Existing Graph

Delete an existing graph

```bash
ghost delete --id <graph_id>
```

## Error Handling

If an error occurs during any operation, Ghost CLI will display a detailed error message. Verify that your API key,
graph ID, and version ID are correctly configured, and ensure an active internet connection when interacting with the
Ghost server.

For support, join our [Ghost Telegram Group](https://t.me/ghostlogsxyz).

## Questions?

For additional support, visit our [Documentation](https://docs.tryghost.xyz/ghostgraph/overview) or join
our [Telegram Group](https://t.me/ghostlogsxyz).

## License

This project is licensed under both the Apache and MIT Licenses.
