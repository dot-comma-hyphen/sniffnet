# Sniffnet

## Project Overview

Sniffnet is a cross-platform network traffic monitoring utility written in Rust. It provides an intuitive graphical user interface to help users visualize and inspect their network activity in real-time. The application is built with a focus on clarity, customizability, and performance.

### Core Technologies & Architecture

-   **Framework:** The GUI is built using the `iced` application framework, following an Elm-style, message-passing architecture.
-   **Packet Sniffing:** It uses the `pcap` library to capture live network traffic and to read from PCAP files.
-   **Packet Parsing:** Packet headers (Link, Network, Transport layers) are parsed using the `etherparse` library.
-   **Concurrency:** The application is multi-threaded. A dedicated thread handles packet capture and processing (`networking::parse_packets`), communicating with the main UI thread via asynchronous channels (`async_channel`). This ensures the UI remains responsive even under heavy network load.
-   **Data Visualization:** Real-time traffic charts are rendered using `plotters-iced`.
-   **Geolocation & ASN:** IP address geolocation and Autonomous System (AS) information are retrieved from local MaxMind DB (`.mmdb`) files using the `maxminddb` crate.
-   **Configuration:** Application settings and user preferences are managed via a `conf.toml` file, serialized and deserialized using `serde`.
-   **Styling:** A comprehensive styling system allows for extensive theming, including several built-in themes (Dracula, Nord, Solarized, etc.) and the ability to load custom themes from TOML files.

### Key Features

-   Live traffic monitoring from a selected network adapter.
-   Analysis of saved PCAP capture files.
-   Real-time charts for traffic rates (bytes/bits/packets per second).
-   Detailed inspection table of individual network connections.
-   Geolocation and ASN information for remote hosts.
-   Application-layer protocol identification based on port numbers.
-   Configurable notifications for traffic rate thresholds and activity from favorite hosts.
-   BPF support for pre-filtering captured traffic.
-   Extensive theming and UI customization, including multiple languages.
-   Ability to export captures to a PCAP file.

## Code Structure

The project is organized into distinct modules within the `src` directory, each with a clear responsibility:

-   `main.rs`: The application entry point. Initializes the `iced` application, window settings, and the main `Sniffer` state struct.
-   `cli/`: Handles command-line argument parsing using `clap`.
-   `networking/`: Manages all aspects of packet capture and processing.
    -   `parse_packets.rs`: Contains the core logic for the packet processing thread.
    -   `manage_packets.rs`: Includes functions for header analysis, traffic classification, and updating data structures.
-   `gui/`: Contains all UI-related code.
    -   `sniffer.rs`: The main `Sniffer` struct, which holds the entire application state. Its `update` function is the central point for handling all UI events and messages from other threads.
    -   `pages/`: Defines the different views/pages of the application (e.g., `initial_page`, `overview_page`, `inspect_page`, `settings_..._page`).
    -   `components/`: Reusable UI widgets like headers, footers, and buttons.
    -   `styles/`: Implements the theming and styling for all `iced` widgets.
-   `chart/`: Manages the traffic rate chart and donut chart visualizations.
-   `report/`: Contains the logic for sorting, filtering, and displaying the list of network connections on the "Inspect" page.
-   `mmdb/`: Handles lookups in the MaxMind DB files for country and ASN data.
-   `notifications/`: Implements the logic for triggering and logging user-configured notifications.
-   `translations/`: Provides multi-language support for all UI text.
-   `utils/`: Contains miscellaneous helper functions, such as checking for updates, error logging, and string formatting.

## Building and Running

The project uses Cargo, the standard Rust build tool and package manager.

-   **To build the project:**
    ```bash
    cargo build --release
    ```

-   **To run the project:**
    ```bash
    cargo run --release
    ```

-   **To run tests:**
    ```bash
    cargo test
    ```

---

## Autonomous Agent Operational Workflow

This workflow outlines the best practices for an autonomous coding agent to ensure its operations are state-aware, context-aware, and follow a robust, methodical approach to task completion.

### Phase 1: Deconstruction and Contextual Analysis

1.  **Deconstruct the Request:**
    *   Identify the user's primary intent and the explicit requirements of the task.
    *   Break down complex requests into a series of smaller, logical, and achievable sub-tasks.
    *   Identify any ambiguities or missing information. If critical details are absent, formulate clarifying questions for the user before proceeding.

2.  **Analyze the Project Context:**
    *   **File System Exploration:** Use tools like `glob` and `list_directory` to map the project structure and locate relevant files and modules related to the task.
    *   **Codebase Comprehension:** Use `read_file` and `search_file_content` to thoroughly examine the existing code. The goal is to understand the current implementation, logic, and data flow. *Never assume; always verify by reading the code.*
    *   **Identify Conventions and Patterns:** Scrutinize the surrounding code to identify established patterns, including:
        *   **Coding Style:** Formatting, indentation, and naming conventions.
        *   **Architectural Patterns:** MVC, message-passing, entity-component-system, etc.
        *   **State Management:** How application state is stored, accessed, and modified.
        *   **Error Handling:** The established strategy for managing and propagating errors (e.g., `Result`, `Option`, exceptions).
    *   **Dependency and Configuration Review:** Check configuration files (`Cargo.toml`, `package.json`, `build.gradle`, etc.) to understand project dependencies, available scripts, and build configurations.

### Phase 2: Strategic Planning

1.  **Formulate a Step-by-Step Plan:**
    *   Based on the context analysis, create a detailed, sequential plan for implementation.
    *   Each step in the plan should be a concrete action (e.g., "1. Add field `xyz` to struct `MyStruct` in `file.rs`. 2. Update the constructor to initialize the new field. 3. Modify function `abc` to utilize the new field.").

2.  **Define a Verification Strategy:**
    *   Crucially, the plan must include a strategy for verifying the correctness of the changes.
    *   Identify how to run the project's tests, linters, and build scripts.
    *   If the existing test coverage is insufficient for the changes, the plan should include a step to add new unit or integration tests.

3.  **Propose the Plan (If Necessary):**
    *   For any non-trivial task, present a concise summary of the plan to the user for confirmation. This ensures alignment and prevents wasted effort on incorrect paths.

### Phase 3: Idiomatic Implementation

1.  **Execute the Plan:**
    *   Carry out the plan step-by-step using the appropriate tools (`write_file`, `replace`, `run_shell_command`).
    *   Make changes incrementally and ensure each change is a logical progression.

2.  **Adhere to Conventions:**
    *   Ensure all new or modified code strictly conforms to the coding style, patterns, and conventions identified during the analysis phase. The goal is for the agent's code to be indistinguishable from code written by the project's original developers.
    *   **Formatting:** For Rust projects, always use `rustfmt` to ensure the strictest adherence to the established code style. Run `cargo fmt` after making changes.

### Phase 4: Rigorous Self-Verification (State Awareness)

1.  **Execute Verification Suite:**
    *   After each significant change, run the verification suite defined in the plan.
    *   **Static Analysis:** Run linters and formatters (`ruff check`, `cargo fmt`, `prettier`).
    *   **Compilation/Build:** Run the build command (`cargo build`, `npm run build`) to check for compilation errors.
    *   **Testing:** Execute the project's test suite (`cargo test`, `npm test`).

2.  **Analyze and Debug:**
    *   If any verification step fails, **do not proceed.**
    *   Carefully analyze the error messages, compiler output, or test failures.
    *   Form a hypothesis about the root cause of the failure.
    *   Return to the implementation phase to correct the issue, treating the failure as a state change that requires a new plan of action. This iterative debug loop is the core of state-aware operation.

### Phase 5: Task Completion and Handoff

1.  **Final Confirmation:**
    *   Once all implementation steps are complete and the entire verification suite passes, the task is considered complete.

2.  **Report Results:**
    *   Inform the user of the successful completion of the task.
    *   Briefly summarize the changes made.

3.  **Propose Next Steps:**
    *   Anticipate the user's needs by suggesting logical next actions, such as:
        *   "Would you like me to write a commit message for these changes?"
        *   "Should I now address the next sub-task?"
        *   "I can now remove the debugging statements I added. Shall I proceed?"
