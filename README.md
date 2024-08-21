[![Test](https://github.com/bornacvitanic/fumble/actions/workflows/rust.yml/badge.svg)](https://github.com/bornacvitanic/fumble/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/repo/github/bornacvitanic/fumble/status.svg)](https://deps.rs/repo/github/bornacvitanic/fumble)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/fumble.svg)](https://crates.io/crates/fumble)
[![Download](https://img.shields.io/badge/download-releases-blue.svg)](https://github.com/bornacvitanic/fumble/releases)

# fumble

fumble is an oxidized (Rust-based) implementation of the original clumsy tool, designed to simulate adverse network conditions on Windows systems. Utilizing the powerful capabilities of the WinDivert library, fumble intercepts live network packets and allows users to introduce controlled delays, drops, duplications, and modifications to these packets. This tool is invaluable for debugging network-related bugs, testing application resilience under poor network conditions, and evaluating performance in unreliable network environments.

Just like its predecessor, fumble offers a user-friendly and interactive way to degrade network performance intentionally, making it easier to diagnose issues and improve the robustness of network-dependent applications. Whether you're a developer needing to simulate a flaky connection or a QA engineer stress-testing an application, fumble provides a versatile and reliable solution.

![image](https://github.com/user-attachments/assets/857b528f-8b0d-4c51-a777-c7fe84e9e4cb)

## Features
### Packet Manipulation Features
- **Packet Filtering**: Use filter expressions to capture specific packets.
- **Packet Dropping**: Drop packets with a specified probability.
- **Packet Delay**: Introduce delays to simulate latency.
- **Packet Throttling**: Temporarily hold or drop packets to simulate sporadic network throttling.
- **Packet Reordering**: Reorder packets by applying a random delay to simulate out-of-order delivery.
- **Packet Tampering:** Modify packet payloads by altering, flipping, or injecting data to simulate corrupted transmissions.
- **Packet Duplication**: Duplicate packets to simulate packet duplication issues.
- **Bandwidth Limiting**: Limit the bandwidth to simulate a constrained network environment.
### Binary Features
- **General CLI Usage:** Utilize a comprehensive command-line interface for flexible and detailed control over network manipulation settings. Easily specify parameters for packet filtering, dropping, delaying, throttling, reordering, tampering, duplicating, and bandwidth limiting.
- **Configuration Support:** Easily manage your settings through configuration files. Create, list, and use configuration files to save and load your preferred settings, simplifying the setup and ensuring consistent behavior across different runs.
- **Text User Interface (TUI) Mode:** a Text User Interface (TUI) for users who prefer an interactive and visual interface over the command line. The TUI provides a more user-friendly way to configure and manage network manipulation settings in real-time.

## Roadmap

- **TUI/CLI Enhancements:** Enhance the Text User Interface (TUI) to offer a graph visualization of the network traffic and all the modifications being applied by fumble.
- **Graphical User Interface (GUI):** Implement a GUI to cater to users who prefer not to use the command line.

## Requirements

**Important:** `fumble` requires `WinDivert.dll` and `WinDivert64.sys` to function properly. You can download them from the [official WinDivert releases page](https://github.com/basil00/Divert/releases).

<details>
<summary><strong>Installing WinDivert</strong></summary>

1. Download the latest version of `WinDivert.dll` and `WinDivert64.sys`.
2. Place `WinDivert.dll` and `WinDivert64.sys` in the same directory as the `fumble` binary executable, or add the directory containing these files to your system's `PATH` environment variable.

</details>

## Installation
### From Source
To build `fumble`, ensure you have Rust and Cargo installed.\
Clone the repository and build the project using Cargo:

```sh
git clone https://github.com/bornacvitanic/fumble.git
cd fumble
cargo build --release
```
To ensure proper functionality, place WinDivert.dll and WinDivert64.sys in the same directory as the fumble binary (typically `./target/debug` or `./target/release`). Alternatively, you can add the directory containing these files to your system's `PATH` environment variable.

### From GitHub Releases
You can download pre-built binaries from the  [GitHub Releases](https://github.com/bornacvitanic/fumble/releases) page:

1. Download the appropriate release for your platform.
2. Extract the files from the release archive.

The release archive already contains a copy of the `WinDivert.dll` and `WinDivert64.sys` files.

### From crates.io as a CLI

To install fumble as a command-line tool globally, use:

```sh
cargo install fumble
```

This installs the fumble binary, enabling you to use the CLI tool globally.
After installation, ensure that `WinDivert.dll` and `WinDivert64.sys` are placed in the same directory as the fumble binary (typically located at `C:\Users\username\.cargo\bin` on Windows). Alternatively, you can add the directory containing these files to your system's `PATH` environment variable.

### Using fumble as a Library

To include `fumble` as a dependency in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
fumble = "0.6.0"
```

Run cargo build to download and compile the crate.\
To ensure proper functionality, place WinDivert.dll and WinDivert64.sys in the same directory as the fumble binary (typically `./target/debug` or `./target/release`). Alternatively, you can add the directory containing these files to your system's `PATH` environment variable.

## Usage

Run the `fumble` executable with the desired options:

```sh
fumble --filter "inbound and tcp" --delay-duration 500 --drop-probability 0.1
```

## TUI Mode

fumble offers a Text User Interface (TUI) mode for those who prefer a more interactive experience. The TUI allows you to view, configure, and manage network manipulation settings in a visual interface, making it easier to adjust settings on the fly. You can initialise the TUI via either a config or normal cli commands.

### Launching the TUI

To start `fumble` in TUI mode, use the following command:

```sh
fumble  -t
```
Once in the TUI, you can navigate through different settings using your keyboard. The TUI provides real-time feedback and allows for quick adjustments to your configurations.

You can initialize the TUI with default values from either individual commands of a config. You can also specify a initial filter:
```sh
fumble --filter "outbound and udp"  -t
```
```sh
fumble --filter "outbound and udp" --delay-duration 500  -t
```
```sh
fumble --filter "inbound and udp" --use-config config_name  -t
```

<details>
  <summary>Command-Line Options</summary>

- `-f, --filter <FILTER>`: Filter expression for capturing packets.
- `--drop-probability <drop-probability>`: Probability of dropping packets, ranging from 0.0 to 1.0.
- `--delay-duration <delay-duration>`: Delay in milliseconds to introduce for each packet.
- `--throttle-probability <throttle-probability>`: Probability of triggering a throttle event, ranging from 0.0 to 1.0.
- `--throttle-duration <throttle-duration>`: Duration in milliseconds for which throttling should be applied.
  - **Default**: `30`
- `--throttle-drop`: Indicates whether throttled packets should be dropped.
- `--reorder-probability <reorder-probability>`: Probability of reordering packets, ranging from 0.0 to 1.0.
- `--reorder-max-delay <reorder-max-delay>`: Maximum random delay in milliseconds to apply when reordering packets.
  - **Default**: `100`
- `--tamper-probability <tamper-probability>`: Probability of tampering packets, ranging from 0.0 to 1.0.
- `--tamper-amount <tamper-amount>`: Amount of tampering that should be applied, ranging from 0.0 to 1.0.
  - **Default**: `0.1`
- `--tamper-recalculate-checksums <tamper-recalculate-checksums>`: Whether tampered packets should have their checksums recalculated to mask the tampering and avoid the packets getting automatically dropped.
  - **Possible values**: `true`, `false`
- `--duplicate-probability <duplicate-probability>`: Probability of duplicating packets, ranging from 0.0 to 1.0.
- `--duplicate-count <duplicate-count>`: Number of times to duplicate each packet.
  - **Default**: `1`
- `--bandwidth-limit <bandwidth-limit>`: Maximum bandwidth limit in KB/s.
- `-t, --tui`: Launch the Text User Interface (TUI).
- `-h, --help`: Print help (see a summary with `-h`).

**Configuration Management:**

- `--create-default <CREATE_DEFAULT>`: Command to create a default configuration file with the specified name.
- `--use-config <USE_CONFIG>`: Command to use an existing configuration file based on the specified name.
- `--list-configs`: Command to list all available configuration files.
</details>
<details>
  <summary>Examples</summary>

- Drop 10% of incoming TCP packets:

  ```sh
  fumble --filter "inbound and tcp" --drop-probability 0.1
  ```

- Delay packets by 500 milliseconds:

  ```sh
  fumble --filter "inbound and tcp" --delay-duration 500
  ```

- Throttle packets with a 10% probability for 30 milliseconds and drop them:

  ```sh
  fumble --filter "inbound and tcp" --throttle-probability 0.1 --throttle-duration 30 --throttle-drop
  ```

- Throttle packets with a 20% probability for 50 milliseconds and delay them:

  ```sh
  fumble --filter "inbound and tcp" --throttle-probability 0.2 --throttle-duration 50
  ```

- Reorder packets with a 10% probability and a maximum delay of 100 milliseconds:

  ```sh
  fumble --filter "inbound and tcp" --reorder-probability 0.1 --reorder-max-delay 100
  ```

- Tamper packets with a 25% probability and a tamper amount of 0.2, recalculating checksums:

  ```sh
  fumble --filter "inbound and tcp" --tamper-probability 0.25 --tamper-amount 0.2 --tamper-recalculate-checksums true
  ```

- Tamper packets with a 30% probability, and do not recalculate checksums:

  ```sh
  fumble --filter "inbound and tcp" --tamper-probability 0.3 --tamper-recalculate-checksums false
  ```

- Duplicate packets with a 50% chance:

  ```sh
  fumble --filter "inbound and tcp" --duplicate-probability 0.5 --duplicate-count 2
  ```

- Limit bandwidth to 100 KB/s:

  ```sh
  fumble --filter "inbound and tcp" --bandwidth-limit 100
  ```
</details>

## Logging

The tool uses the env_logger crate for logging. By default, informational messages are shown.

### Enabling Detailed Logs

To see more detailed logs, set the `RUST_LOG` environment variable before running `fumble`.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## Acknowledgements

- [clap](https://crates.io/crates/clap) - A command-line argument parser for Rust that provides a simple and powerful API for defining complex CLI interfaces.
- [windivert](https://crates.io/crates/windivert) - A Rust binding for the WinDivert library, used for network packet interception and manipulation.
- [rand](https://crates.io/crates/rand) - A Rust library for generating random numbers, used for implementing random packet dropping and duplication.
- [ratatui](https://crates.io/crates/ratatui) - A Rust library for building terminal user interfaces with an emphasis on simplicity and ease of use.
- [tui-textarea](https://crates.io/crates/tui-textarea) - A Rust crate for managing text input within terminal user interfaces.
- [lazy_static](https://crates.io/crates/lazy_static) - A Rust macro for defining statically initialized variables that are computed lazily.
- [ctrlc](https://crates.io/crates/ctrlc) - A Rust library for handling Ctrl-C signals, enabling graceful shutdowns and clean thread termination.
- [regex](https://crates.io/crates/regex) - A Rust library for regular expressions, used for string matching operations.
- [env_logger](https://crates.io/crates/env_logger) - A simple logger for Rust applications that can be configured via environment variables.
- [log](https://crates.io/crates/log) - A logging facade that provides a common interface for various log implementations.
- [serde](https://crates.io/crates/serde) - For serialization and deserialization of configuration files.
- [toml](https://crates.io/crates/toml) - For parsing and serializing TOML configuration files.
- [dirs](https://crates.io/crates/dirs) - For handling configuration directories across different operating systems.
- [thiserror](https://crates.io/crates/thiserror) - For ergonomic error handling.

## Contact

- **Email**: [borna.cvitanic@gmail.com](mailto:borna.cvitanic@gmail.com)
- **GitHub Issues**: [GitHub Issues Page](https://github.com/bornacvitanic/fumble/issues)