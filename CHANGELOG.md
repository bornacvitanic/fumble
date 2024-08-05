# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] - 2024-08-05

### Bug Fixes

- Fix filter validation method not properly closing WinDivert handle

- Fix tests to use new Probability type


### Documentation

- Document packet tampering functionality in README.md


### Features

- Add async packet receiving and processing with Tokio integration

- Implement package tampering support and CLI commands


### Removals

- Remove unused extract_payload method from tamper.rs


### Styling

- Clean up comments and logs in capture logic

- Style capture.rs using fmt

- Clean up unused imports in capture file

- Style scripts using fmt

- Clean up tamper logic using clippy


### Updates

- Update project version to 0.4.0

- Update Cargo.toml to remove unused tokio features


## [0.3.0] - 2024-08-03

### Bug Fixes

- Fix capture not clearing packets vector in between loop iterations

- Fix start_packet_processing method usign the wrong packets vector


### Documentation

- Update README.md with feature roadmap

- Update README.md to increment version number in installation example

- Update CHANGELOG.md with changes for v0.2.0


### Features

- Implement forced exit on double Ctrl + C for immediate termination

- Add filter port validation support

- Add graceful shutdown and improved loggin with Ctrl-C signal handling

- Refactor capture.rs to add a PacketProcessingState structs for easier state passing between methods


### Refactors

- Refactor CLI argument parsing with custom comamnd and field names


### Renames

- Update .idea meta files to rename project to fumble


### Styling

- Clean up scripts using fmt

- Clean up scripts using clippy


### Testing

- Add unit tests for bandwidth limiting functionality


### Updates

- Update project to 0.3.0

- Update main method to extract Ctrl-C handing logic to submethod

- Update bandwidth limiter to define the max buffer size using memory size and not package count to better handle high amounts of small packets

- Update Cargo.toml package version to 0.2.0


## [0.2.0] - 2024-08-02

### Bug Fixes

- Update github workflow action to try and fix WinDivert download


### Documentation

- Update README.md to mention new throttling feature

- Update README.md to mention new reorder and bandwidth limitin functionalities

- Update CHANGELOG.md with changes for v0.1.0


### Features

- Add throttling feature to temporarily hold or drop packets to simulate sporadic network throttling

- Add bandwidth limiting feature to control package transmission rate

- Implement package capture on a separate thread as to not block processing of delayed packets while there are no new packets

- Add packet reordering feature with CLI support


### Refactors

- Refactor main.rs to extract logic into methods and move them into the right files

- Refactor delay packet storage to use VecDeque for FIFO order


### Styling

- Style files using fmt


### Updates

- Update github workflow action to set WinDivert download source and to remove linux build

- Update github workflow action to handle windows builds


## [0.1.0] - 2024-07-31

### Documentation

- Add README.md

- Add LICENSE.md


### Features

- Add github workflow for automatic building and testing when pushing to main

- Implement automatic changelog generation using cliff config

- Implement improved logging with env_logger setup and detailed CLI help

- Add packet delay feature with CLI support for delay duration

- Add packet duplication feature with CLI support for count and probability

- Update capture.ts to add Clone implementation

- Add drop probability value validation

- Implement From trait for PacketData

- Add better logging

- Add better error handling for WinDivert initialization

- Add library interface

- Add CLI network filter specification support

- Add CLI support using Clap

- Add RustRover meta files

- Add dependencies and implement basic packet dropping functionality with logging


### Refactors

- Refactor project structure to modularize network and utility functions


### Renames

- Rename the project to fumble


### Testing

- Add Unit tests for capture and dropping logic