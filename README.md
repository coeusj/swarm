# SWARM

- `bee`: Simulated device that sends telemetry data to `hive`.
- `hive`: Ground station that receives telemetry from the `bee` and pushes it to a NATS server.
- `client`: Client/HMI that reads device telemetry from the NATS server.

### Run Binaries

`bee`
```bash
cargo run --bin bee
```

`hive`
```bash
cargo run --bin hive
```

`client`
```bash
cargo run --bin client
```

### Configuration

File: `Config.toml`

### Dependencies

- FlatBuffers (`https://flatbuffers.dev/`)
- NATS JetStream (`https://docs.nats.io/concepts/jetstream`)