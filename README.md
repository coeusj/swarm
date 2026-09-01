# SWARM

- `bee`: Simulated device that sends telemetry data to `hive`.
- `hive`: Ground station that receives telemetry from the `bee` and pushes it to a NATS server.
- `beekeeper`: Client/HMI that reads device telemetry from the NATS server.

### Run Binaries

`bee`
```bash
cargo run --bin bee -- id [bee-id]
```

`hive`
```bash
cargo run --bin hive
```

`beekeeper`
```bash
cargo run --bin beekeeper
```

### Configuration

File: `Config.toml`

### Compile FlatBuffers

```bash
flatc ./src/fbs/payload.fbs --rust
```

### Dependencies

- FlatBuffers (`https://flatbuffers.dev/`)
- NATS JetStream (`https://docs.nats.io/concepts/jetstream`)