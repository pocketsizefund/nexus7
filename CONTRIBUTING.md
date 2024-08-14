# Contributing to nexus7

We're excited that you're interested in contributing to nexus7! This document will guide you through setting up your development environment and the process of contributing to the project.

## Setting up the development environment

We use [mise](https://github.com/jdx/mise) (formerly rtx) to manage our development environment. This ensures that all contributors are using the same versions of tools and dependencies.

### 1. Install mise

First, install mise by following the instructions in the [mise documentation](https://github.com/jdx/mise#installation).

### 2. Clone the repository

```sh
git clone https://github.com/your-username/nexus7.git
cd nexus7
```

### 3. Set up the project with mise

mise will automatically read the `.mise.toml` file in the project root and install the specified versions of tools. Run:

```sh
mise install
```

This command will install the correct versions of Rust, Cargo, and any other dependencies required for the project.

### 4. Set up pre-commit hooks

We use pre-commit to run checks before each commit. Install and set it up:

```sh
mise use pre-commit
pre-commit install
```

## Contributing guidelines

Once you have set up your development environment, you can start contributing to the project. Here are some guidelines to keep in mind:

* Make sure to follow the Rust coding conventions and best practices.
* Write comprehensive tests for any new code you add.
* Document your changes and updates in the code and in the commit messages.
* Open a pull request with your changes and wait for review and approval from the maintainers.

## Reporting issues

If you find any issues or bugs in the project, please open an issue on the GitHub repository. Provide as much detail as possible, including steps to reproduce the issue and any error messages you may have encountered.

## Code of Conduct

Be a nice person.