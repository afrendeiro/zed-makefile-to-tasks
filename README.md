Makefile Task Generator for Zed
===============================

![CAUTION](https://img.shields.io/badge/CAUTION-Under%20Development-red)
This extension is still under development and may not work as expected.

This Zed extension reads the `Makefile` in your project's root directory and generates a .tasks.json file with tasks corresponding to each target defined in the Makefile.

# Features

- Automatically parses the `Makefile` in the project's root directory.
- Generates a project-specific `.tasks.json` file with tasks for each Makefile target.
- Enables easy execution of make targets directly from Zed's task runner.

# Installation

1. Install the Extension:
    Open Zed and navigate to the Extensions view by pressing `Cmd+Shift+X` (macOS) or `Ctrl+Shift+X` (Linux).
    Search for "Makefile Task Generator" and click "Install".
2. Verify Installation:
    After installation, the extension will be listed among your installed extensions.

# Usage

1. Generate Tasks:
    Open the command palette in Zed by pressing `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux).
    Type generate-tasks and select the command "Generate Tasks from Makefile".
    The extension will parse the Makefile in your project's root directory and create a .tasks.json file with corresponding tasks.

2. Run Tasks:
    Open the command palette and type task to see the list of available tasks.
    Select the desired task to execute it.

# Requirements

- Ensure that a Makefile exists in the root directory of your project.
- Zed editor version 0.80.0 or higher.

# Contributing

Contributions are welcome!

Feel free to open issues or submit pull requests on the [GitHub repository](https://github.com/afrendeiro/zed-makefile-to-tasks).

# License
This project is licensed under the MIT License.

# Acknowledgments
This extension utilizes the [zed_extension_api crate](https://docs.rs/zed_extension_api/0.2.0/zed_extension_api/index.html).
