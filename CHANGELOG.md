# Changelog

All notable changes to this project will be documented in this file.

## [0.6.0] - 2024-08-20

### Bug Fixes

- Fix start stop toggle input working while focus is on filter aswell

- Fix TextArea set_text extension not overriding existing text

- Fix sending and receiving windivert instances being ont he same priority causing one to randomly absorb all the packets


### Documentation

- Add docstrings to trait methods for CLI and TUI extension traits


### Features

- Add start and stop toggle to be able to toggle all processing

- Add keybind info for filter and logs widgets

- Add conditional block border highlight method to unify highlighting logic

- Add BandwidthStats to track throughput and storage buffer packet count

- Add DuplicateStats to track duplicate multiplier and display it in the tui

- Add TamperStats to visualize the packet data being tampered

- Add ReorderStats to keep track of reordering percentage and number of delayed packets

- Add ThrottleStats to keep track of the throttling state and package drop count

- Add DelayStats to keep track of the number of actively delayed packets

- Add statistic tracking support

- Add TUI support using ratatui

- Update main to add a separate thread for packet processing


### Moves

- Move EWMA to util subfolder


### Refactors

- Refactor TUI widgets by modularazing utility functions and improving code organization

- Refactor packet manipulation settings to use optional structs and improve code consistency

This commit refactors various packet manipulation settings to
encapsulate their fields within optional structs. This change improves
code consistency by grouping related settings together and ensuring that
they are either fully provided or omitted as a whole. Additionally, the
code has been cleaned up to remove unnecessary unwraps and serialization
logic, resulting in more robust and maintainable code. The update also
includes improvements in how CLI state is initialized and updated,
reducing redundancy and improving clarity.

- Refactor tui cli methods to reduce nesting via early return unpacks

- Refactor rest of tui cli methods to reduce repetition

- Refactor tui updating from statistics method to reduce repetition

- Refactor CLI and TUI state management: Rename AppState to TuiState, introduce extension traits for modular updates

- Refactor main.rs to extract cli and tui state interaction to separate module

- Refactor cli updating logic to extract text area parsing logic into extension methods


### Removals

- Remove validate_probability method for text areas

- Remove validate_usize method for text areas which was doing both validation and block rendering updates


### Styling

- Style code using fmt

- Clean up code using Clippy


### Updates

- Update TUI cli command to default to false

- Improve console statistic logging

- Improve custom logger to have the capabilities of the default logger

- Update tui input to use tui state for better isolation

- Improve FilterWidget validation logic by caching last valid filter state and reverting to it on escape

- Update LogsWidget to give feedback upon chaning log level

- Update Widgets to handle Enter the same as Esc as to prevent entering of new lines in text areas

- Improve TamperWidget validation logic

- Improve ReorderWidget validation logic

- Improve DuplicateWidget validation logic

- Improve ThrottleWidget validation logic

- Improve DropWidget validation logic

- Improve DelayWidget validation logic

- Improve BandiwdthWidget validation logic

- Update FilterWidget and packet receiving thread to be able to change the filter via the tui

- Update sending and receiving windivert handles to set matching flags to send and read only

- Update TamperWidget to change info block border color logic

- Update main to unify thread joining methods


## [0.5.0] - 2024-08-07

### Bug Fixes

- Fix packet manipulation module tests failing

- Squash merge fixing-packet-capture into develop


### Documentation

- Update CHANGELOG.md to contain 0.5.0 changes

- Update README.md to be up to date with the latest changes

- Update README.md to mention known issues


### Features

- Add configuration file support and packet manipulation settings serialization


### Refactors

- Refactor configuration management to use the users configuration directory

- Refactor CLI architecture to modularize cli components

- Squash merge cleanup-refactor into develop


### Removals

- Remove unused import in tamper.rs


### Revert

- Revert back to using standard threads instead of tokio async

even though tokio async looked promising as a way to get around the
WinDivert receive blocking call testing showed that it caused packages
getting skipped when sent at low amounts.


### Styling

- Style code using fmt

- Clean up code using clippy


### Updates

- Update duplicate packets method to use the count number as the duplicate count not the total count of outgoing packet copies

- Update project to 0.4.2


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

- Update CHANGELOG to contain 0.4.0 changes

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