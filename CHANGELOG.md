# Changelog

All notable changes to this project will be documented in this file.

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


