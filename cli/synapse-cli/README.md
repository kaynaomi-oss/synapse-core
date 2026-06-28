# synapse-cli

`synapse-cli` is a small CLI for working with Synapse admin reconciliation endpoints.

## Commands

The reconciliation tree is:

- `synapse admin reconciliation reports`
- `synapse admin reconciliation report <REPORT_ID>`
- `synapse admin reconciliation run --account <ACCOUNT> [--period-hours <HOURS>]`

The help text spells out required and optional flags for each subcommand. For example:

```powershell
cargo run --manifest-path cli/synapse-cli/Cargo.toml -- admin reconciliation run --help
```

## Example

In one terminal, start the mock API:

```powershell
cargo run --manifest-path cli/synapse-cli/Cargo.toml --bin mock-server
```

Then run a reconciliation against it and print the resulting summary:

```powershell
cargo run --manifest-path cli/synapse-cli/Cargo.toml -- `
  --base-url http://127.0.0.1:4010 `
  admin reconciliation run `
  --account GA_TEST_ACCOUNT `
  --period-hours 24
```

Sample output:

```text
Reconciliation completed successfully

Report ID: 3f1d8c31-5f1d-4fb8-93e0-112233445566
Generated: 2026-06-27T06:10:12Z
Period: 2026-06-26T06:10:12Z to 2026-06-27T06:10:12Z

Summary:
  Database transactions: 12
  Chain payments: 11
  Missing on chain: 1
  Orphaned payments: 0
  Amount mismatches: 1
  Has discrepancies: yes
```
