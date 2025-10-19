# Contributing to OpenTelemetry Weaver Examples

Thank you for your interest in contributing to the OpenTelemetry Weaver Examples repository! This project is part of the [OpenTelemetry Weaver](https://github.com/open-telemetry/weaver) project and aims to provide clear, working examples of using Weaver.

## Getting Started

### Prerequisites

Before you begin, ensure you have:
- [OpenTelemetry Weaver](https://github.com/open-telemetry/weaver/releases) installed and on your PATH
- Git for version control
- Familiarity with OpenTelemetry concepts

### Setting Up Your Development Environment

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/opentelemetry-weaver-examples.git
   cd opentelemetry-weaver-examples
   ```
3. Add the upstream repository as a remote:
   ```bash
   git remote add upstream https://github.com/open-telemetry/opentelemetry-weaver-examples.git
   ```

## How to Contribute

### Reporting Issues

If you find a bug or have a suggestion for improvement:
- Check if the issue already exists in the [issue tracker](https://github.com/open-telemetry/opentelemetry-weaver-examples/issues)
- If not, create a new issue with a clear title and description
- Include steps to reproduce the issue, expected behavior, and actual behavior
- Add relevant labels to help categorize the issue

### Adding a New Example

We welcome new examples that demonstrate different use cases of OpenTelemetry Weaver! When adding a new example:

1. Create a new directory under the repository root with a descriptive name
2. Include a `README.md` file that explains:
   - What the example demonstrates
   - Prerequisites and setup instructions
   - Step-by-step instructions to run the example
   - Expected output or results
3. Ensure all necessary files are included (templates, configurations, etc.)
4. Test that the example works from a fresh clone of the repository
5. Update the main [README.md](README.md) to include your example in the list

### Improving Existing Examples

To improve an existing example:
- Ensure changes maintain backward compatibility where possible
- Update the example's README.md if the changes affect usage
- Test thoroughly before submitting

### Submitting Pull Requests

1. Create a new branch for your changes:
   ```bash
   git checkout -b my-example-branch
   ```

2. Make your changes and commit them with clear, descriptive messages:
   ```bash
   git add .
   git commit -m "Add example demonstrating custom templates"
   ```

3. Push your branch to your fork:
   ```bash
   git push origin my-example-branch
   ```

4. Open a Pull Request (PR) on GitHub with:
   - A clear title describing the change
   - A description explaining what the PR does and why
   - References to any related issues
   - Screenshots or example output if applicable

### Pull Request Guidelines

- Keep PRs focused on a single example or improvement
- Ensure all examples are tested and working
- Follow the existing structure and style of the repository
- Update documentation as needed
- Make sure your PR title clearly describes the contribution
- Be responsive to feedback and questions during review

## Code of Conduct

This project follows the [OpenTelemetry Code of Conduct](https://github.com/open-telemetry/community/blob/main/code-of-conduct.md). By participating, you are expected to uphold this code.

## Getting Help

- Join the [#otel-weaver](https://cloud-native.slack.com/archives/C0697EXNTL3) channel on CNCF Slack
- Check the [OpenTelemetry Weaver documentation](https://github.com/open-telemetry/weaver)

## Review Process

Pull requests will be reviewed by the project maintainers. We aim to:
- Provide initial feedback within a few days
- Merge PRs that meet the contribution guidelines and pass review
- Ensure examples are accurate, clear, and useful to the community

## Maintainers

See the main [README.md](README.md) for the current list of maintainers and approvers.

## License

By contributing to this repository, you agree that your contributions will be licensed under the same license as the project (Apache 2.0).
