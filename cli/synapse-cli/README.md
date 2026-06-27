# synapse-cli

Command-line interface for the [Synapse API](https://github.com/Synapse-bridgez/synapse-core).

## Installation

```bash
cargo build --release -p synapse-cli
# binary at: target/release/synapse
```

## Configuration

| Option | Env var | Default |
|---|---|---|
| `--base-url` | `SYNAPSE_BASE_URL` | `http://localhost:3000` |
| `--api-key` | `SYNAPSE_API_KEY` | _(empty)_ |

## Health commands

### `synapse health live`

Liveness probe — returns HTTP 200 if the process is running. No dependency checks.

```
$ synapse health live
{
  "status": "alive"
}
```

```
$ synapse health live --json
{
  "status": "alive"
}
```

### `synapse health ready`

Readiness probe — returns `ready` when the service can accept traffic, `not_ready` when draining.

```
$ synapse health ready
{
  "status": "ready",
  "draining": false
}
```

### `synapse health check`

Full health check — aggregates database connectivity, pool stats, queue depth, and WebSocket connections.

```
$ synapse health check
{
  "status": "healthy",
  "version": "0.1.0",
  "db": "connected",
  "db_pool": {
    "active_connections": 2,
    "idle_connections": 8,
    "max_connections": 10,
    "usage_percent": 20.0
  },
  "pending_queue_depth": 0,
  "current_batch_size": 50,
  "ws_connection_count": 3
}
```

Returns exit code `1` when the server responds with a non-2xx status.

### `synapse health errors`

Lists all registered API error codes and their descriptions.

```
$ synapse health errors --json
{
  "errors": {
    "E001": "Transaction not found",
    "E002": "Invalid asset code"
  },
  "version": "1.0.0"
}
```

An empty error catalog is a valid response — never null.

## Stats commands

### `synapse stats status`

Transaction counts grouped by status.

```
$ synapse stats status
STATUS     COUNT
---------  -----
pending    12
completed  340
failed     3
```

```
$ synapse stats status --json
[
  { "status": "pending", "count": 12 },
  { "status": "completed", "count": 340 },
  { "status": "failed", "count": 3 }
]
```

### `synapse stats daily`

Daily totals for the last N days (default 7, max 365).

```
$ synapse stats daily --days 3
DATE        TRANSACTIONS  TOTAL AMOUNT
----------  ------------  ------------
2026-06-25  18            9000.00
2026-06-26  22            11500.00
2026-06-27  10            5100.00
```

### `synapse stats assets`

Totals grouped by asset code.

```
$ synapse stats assets
ASSET  TRANSACTIONS  TOTAL AMOUNT
-----  ------------  ------------
USD    290           145000.00
XLM    65            3250.00
```

### `synapse stats cache`

Query cache and idempotency cache metrics.

```
$ synapse stats cache --json
{
  "query_cache": { ... },
  "idempotency_cache_hits": 512,
  "idempotency_cache_misses": 44,
  ...
}
```

## Flags

All subcommands accept `--json` to switch from the default table output to pretty-printed JSON.
