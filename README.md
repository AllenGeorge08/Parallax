# Parallax — LiteSVM Oracle SDK

## Brief

Parallax is a **LiteSVM-based oracle SDK** for **deterministic DeFi scenario simulation**. It lets you model “market stress” events (crashes / volatility spikes) while **warping slots and epochs deterministically**, so tests are repeatable but still feel closer to real chain-time progression.

The repo includes:

- A Rust SDK (this crate) that wraps LiteSVM with a harness + time controls.
- An Anchor oracle program (local) used as the on-chain oracle inside the VM.
- Scenario-style tests showing how to use the SDK to simulate a **price crash** and assert downstream behavior (e.g. liquidation thresholds).

## Explanation and project structure

### How it works (conceptually)

In a normal Rust test you:

1. Create a `TestHarness` (LiteSVM instance + payer).
2. Deploy the oracle program into LiteSVM.
3. Derive / create an oracle PDA and initialize it with a starting price.
4. Warp forward in **slots** (and/or epochs) deterministically.
5. Update oracle price as your scenario requires (e.g. crash).
6. Read oracle state back and assert your protocol logic.

### Repo layout

- `src/`
  - `litesvm/`
    - `harness.rs`: `TestHarness` wrapper for deploying programs, funding accounts, sending txs, and warping slots.
  - `oracle/`
    - `behaviour.rs`: `OracleBehaviour` helper to initialize the oracle, set price, read oracle, and run scenario moves like `price_crash(...)`.
  - `timecontroller/`
    - `controller.rs`: `TimeController` for deterministic slot/epoch warping.
  - `tests/`
    - `scenario_testing/`: scenario tests demonstrating “how you might use it”.
- `oracle/`
  - Anchor workspace
  - `oracle/programs/oracle/`: the Anchor oracle program crate used inside LiteSVM

### Artifacts expected for deployment

The harness deploys the oracle program from local artifacts:

- `artifacts/oracle-keypair.json`
- `artifacts/oracle.so`

So the usual workflow is:

- Build the oracle program (Anchor) to produce the `.so` and program keypair.
- Ensure those files exist under `artifacts/`.
- Run the Rust tests (`cargo test`) to execute deterministic scenarios.

## Current scope and scenarios added (price crash)

### Price crash scenario

There’s a scenario test that simulates a **price crash** over a deterministic slot jump and verifies that a sample lending position becomes liquidatable after the crash.

- Test file: `src/tests/scenario_testing/liquidation_scenario_testing.rs`
- Scenario outline:
  - Initialize oracle at `from_price`
  - Read price + slot (baseline)
  - Warp ahead by `slot_jump`
  - Set oracle to `to_price`
  - Assert slot advanced, price updated, and liquidation condition flips as expected

### Running the scenario tests

From repo root:

```bash
cargo test
```

If the program artifacts are missing, build the Anchor oracle program and place the resulting `.so` + keypair JSON under `artifacts/` so the harness can deploy it.

## Exact deps and versions

From the repo root `Cargo.toml`:

- `anchor-lang = { version = "0.31.1", features = ["derive", "init-if-needed"] }`
- `litesvm = "0.6.1"`
- `litesvm-token = "0.6.1"`
- `solana-instruction = "2.2.1"`
- `solana-keypair = "2.2.1"`
- `solana-native-token = "2.2.1"`
- `solana-pubkey = "2.2.1"`
- `solana-signer = "2.2.1"`
- `solana-system-interface = "1.0.0"`
- `solana-transaction = "2.2.1"`
- `solana-message = "2.2.1"`
- `solana-sdk-ids = "2.2.1"`
- `solana-address = "1.0.0"`
- `solana-account = "2.2.1"`
- `solana-sdk = "2.2"`
- `solana-program-runtime = "=2.2.4"`
- `oracle = { path = "oracle/programs/oracle" }`

