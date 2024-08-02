# Changelog

All notable changes to this project will be documented in this file.

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


