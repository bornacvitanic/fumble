[![Test](https://github.com/bornacvitanic/fumble/actions/workflows/rust.yml/badge.svg)](https://github.com/bornacvitanic/fumble/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/repo/github/bornacvitanic/fumble/status.svg)](https://deps.rs/repo/github/bornacvitanic/fumble)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/fumble.svg)](https://crates.io/crates/fumble)
[![Download](https://img.shields.io/badge/download-releases-blue.svg)](https://github.com/bornacvitanic/fumble/releases)

# fumble

fumble is an oxidized (Rust-based) implementation of the original clumsy tool, designed to simulate adverse network conditions on Windows systems. Utilizing the powerful capabilities of the WinDivert library, fumble intercepts live network packets and allows users to introduce controlled delays, drops, duplications, and modifications to these packets. This tool is invaluable for debugging network-related bugs, testing application resilience under poor network conditions, and evaluating performance in unreliable network environments.

Just like its predecessor, fumble offers a user-friendly and interactive way to degrade network performance intentionally, making it easier to diagnose issues and improve the robustness of network-dependent applications. Whether you're a developer needing to simulate a flaky connection or a QA engineer stress-testing an application, fumble provides a versatile and reliable solution.

## Features

- **Packet Filtering**: Use filter expressions to capture specific packets.
- **Packet Dropping**: Drop packets with a specified probability.
- **Packet Delay**: Introduce delays to simulate latency.
- **Packet Throttling**: Temporarily hold or drop packets to simulate sporadic network throttling.
- **Packet Reordering**: Reorder packets by applying a random delay to simulate out-of-order delivery.
- **Packet Duplication**: Duplicate packets to simulate packet duplication issues.
- **Bandwidth Limiting**: Limit the bandwidth to simulate a constrained network environment.

## Requirements

`fumble` requires `WinDivert.dll` to function properly. You can download it from the [official WinDivert releases page](https://github.com/basil00/Divert/releases).

### Installing WinDivert

1. Download the appropriate version of `WinDivert.dll` for your system (32-bit or 64-bit).
2. Place `WinDivert.dll` in the same directory as the `fumble` executable, or add its directory to your system's `PATH`.

## Installation

To build `fumble`, ensure you have Rust and Cargo installed.

### From Source

Clone the repository and build the project using Cargo:

```sh
git clone https://github.com/bornacvitanic/fumble.git
cd fumble
cargo build --release
```

### From crates.io as a Library

You can include `fumble` as a dependency in your Rust project by adding it to your `Cargo.toml`:

```toml
[dependencies]
fumble = "0.2.0"
```

Then, run `cargo build` to download and compile the crate.

### From crates.io as a CLI

To install `fumble` as a command-line tool globally, use the following command:

```sh
cargo install fumble
```

This will install the `fumble` binary, allowing you to use the CLI tool from anywhere on your system.

## Usage

Run the `fumble` executable with the desired options:

```sh
fumble --filter "inbound and tcp" --delay 500 --drop 0.1
```

## Logging

The tool uses the `env_logger` crate for logging. By default, info messages are shown.

### Enabling Detailed Logs

To see more detailed logs, set the `RUST_LOG` environment variable before running `fumble`.

### Command-Line Options

- `-f, --filter <FILTER>`: Filter expression for capturing packets.
- `--drop <DROP>`: Probability of dropping packets in the range 0.0 to 1.0.
- `--delay <DELAY>`: Delay to introduce for each packet in milliseconds.
- `--throttle-probability <PROBABILITY>`: Probability of triggering a throttle event, must be between 0.0 and 1.0.
- `--throttle-duration <DURATION>`: Duration in milliseconds for which throttling is applied during a throttle event.
- `--throttle-drop`: Makes throttled packets be dropped instead of delayed.
- `--reorder <DELAY>`: Apply a random delay to reorder packets, simulating out-of-order delivery.
- `--duplicate-count <COUNT>`: Number of times to duplicate packets.
- `--duplicate-probability <PROBABILITY>`: Probability of duplicating packets, must be between 0.0 and 1.0.
- `--bandwidth-limit <KB/s>`: Limit the bandwidth in KB/s to simulate a constrained network environment.

## Examples

- Drop 10% of incoming TCP packets:

  ```sh
  fumble --filter "inbound and tcp" --drop 0.1
  ```

- Delay packets by 500 milliseconds:

  ```sh
  fumble --filter "inbound and tcp" --delay 500
  ```

- Throttle packets with a 10% probability for 30 milliseconds and drop them:

  ```sh
  fumble --filter "inbound and tcp" --throttle-probability 0.1 --throttle-duration 30 --throttle-drop
  ```

- Throttle packets with a 20% probability for 50 milliseconds and delay them:

  ```sh
  fumble --filter "inbound and tcp" --throttle-probability 0.2 --throttle-duration 50
  ```

- Reorder packets with a maximum delay of 100 milliseconds:

  ```sh
  fumble --filter "inbound and tcp" --reorder 100
  ```

- Duplicate packets with a 50% chance:

  ```sh
  fumble --filter "inbound and tcp" --duplicate-count 2 --duplicate-probability 0.5
  ```

- Limit bandwidth to 100 KB/s:

  ```sh
  fumble --filter "inbound and tcp" --bandwidth-limit 100
  ```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## Acknowledgements

- [clap](https://crates.io/crates/clap) - A command-line argument parser for Rust that provides a simple and powerful API for defining complex CLI interfaces.
- [windivert](https://crates.io/crates/windivert) - A Rust binding for the WinDivert library, used for network packet interception and manipulation.
- [rand](https://crates.io/crates/rand) - A Rust library for generating random numbers, used for implementing random packet dropping and duplication.
- [env_logger](https://crates.io/crates/env_logger) - A simple logger for Rust applications that can be configured via environment variables.
- [log](https://crates.io/crates/log) - A logging facade that provides a common interface for various log implementations.

## Contact

- **Email**: [borna.cvitanic@gmail.com](mailto:borna.cvitanic@gmail.com)
- **GitHub Issues**: [GitHub Issues Page](https://github.com/bornacvitanic/fumble/issues)