
# Contributing to EigenTrust NEAR Smart Contract

We warmly welcome contributions from the community and are grateful for your interest in helping improve the EigenTrust NEAR smart contract. By participating, you agree to abide by our code of conduct and contribute constructively.

## How to Contribute

Contributions can take various forms, from bug reports and feature requests to code submissions. Here’s how you can contribute:

### Reporting Bugs

Before reporting a bug, please check our issue tracker to avoid duplicates. When reporting a bug, include:

- A clear and concise description of what the bug is.
- Steps to reproduce the behavior.
- Expected behavior.
- Screenshots if applicable.
- Additional context or code snippets that might help identify the issue.

### Suggesting Enhancements

Feature requests are welcome. Please open an issue and provide:

- A clear and concise description of the feature.
- Explain the problem it solves or why the feature would be beneficial.
- Any additional context such as mockups, references, or screenshots.

### Pull Requests

We love direct contributions to the codebase. Here’s how you can submit a pull request (PR):

1. **Fork the Repository**: Start by forking the repository and cloning your fork to your local machine.

2. **Create a Branch**: Create a branch in your fork for your contributions.

    ```bash
    git checkout -b feature/AmazingFeature
    ```

3. **Make Your Changes**: Add your changes to this branch. Please adhere to the existing coding style and document your code appropriately.

4. **Write Tests**: If you are adding functionality, write unit tests that cover your changes.

5. **Run Tests**: Ensure all tests pass, including your new ones.

    ```bash
    cargo test
    ```

6. **Commit Your Changes**: Use meaningful commit messages that explain your changes.

    ```bash
    git commit -m 'Add some AmazingFeature'
    ```

7. **Push to the Branch**: Push your changes to your GitHub repository.

    ```bash
    git push origin feature/AmazingFeature
    ```

8. **Open a Pull Request**: Go to the original repository and open a pull request from your feature branch. Fill in the PR template with all relevant information.

### Review Process

Once a pull request is submitted, the repository maintainers will review it. This process might require some discussions and further modifications. Please be patient and address feedback to expedite this process.

## Styleguides

### Git Commit Messages

- Use the imperative mood in the commit message subject line.
- Start the commit message with an applicable verb such as "Add", "Update", "Fix", "Remove", etc.
- First line should be under 50 characters, followed by a blank line and a more detailed description if needed.

### Rust Styleguide

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html).
- Use `cargo fmt` to format your code.
- Ensure your code is well-documented with comments explaining "why" something is done, not just "what".

## Community

- If you need assistance or want to discuss a topic related to the project, use [Discussions](#) in the project repository (link to project discussions if available).

## Code of Conduct

Participation in this project is governed by [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

---