# OpenTelemetry Weaver Examples

This repository contains a set of fully functional, working examples of using OpenTelemetry Weaver.

## Prerequisites

Install weaver by following the instructions from the [release](https://github.com/open-telemetry/weaver/releases) that matches your OS. Be sure to have weaver on your path for the commands in the examples to work.

## Using Weaver in CI/CD

This repository includes GitHub Actions workflows that demonstrate how to use Weaver in CI/CD pipelines:

- **[Validate Examples](.github/workflows/validate-examples.yml)** - Validates semantic convention models and policies
- **[Check Generated Code](.github/workflows/check-generated-code.yml)** - Ensures generated code stays in sync with models
- **[Check Documentation](.github/workflows/check-docs.yml)** - Ensures generated documentation stays in sync with models

These workflows use the [`setup-weaver`](https://github.com/open-telemetry/weaver/tree/main/.github/actions/setup-weaver) action to install Weaver. You can copy these patterns to your own repositories.

## Examples

- [Basic](basic/README.md)
  - This example demonstrates some basic usage of OpenTelemetry Weaver.
  - It defines a simple model dependent on the OpenTelemetry Semantic Conventions.
  - It shows how to generate docs and code, check and resolve the model, and use live-check with a sample application.

- [Live-check Custom Advisor](custom_advisor/README.md)
  - This example shows how to create and use a custom advisor with OpenTelemetry Weaver Live-check.
  - It demonstrates implementing custom validation logic using Rego policies.
  - It shows how to validate telemetry data against custom annotations in your semantic convention model.

- [Emit OTLP Logs](emit_otlp_logs/README.md)
  - This example demonstrates how to set up the OpenTelemetry Collector to export to a running `weaver registry live-check` instance for continuous telemetry validation.
  - It shows how to prevent infinite loops when Weaver sends Findings back to the collector.
  - It illustrates how to integrate Weaver Findings into existing observability pipelines.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

### Maintainers

- [OpenTelemetry Weaver Maintainers](https://github.com/orgs/open-telemetry/teams/weaver-maintainers)

For more information about the maintainer role, see the [community repository](https://github.com/open-telemetry/community/blob/main/guides/contributor/membership.md#maintainer).


### Approvers

- [OpenTelemetry Weaver Approvers](https://github.com/orgs/open-telemetry/teams/weaver-approvers)

For more information about the approver role, see the [community repository](https://github.com/open-telemetry/community/blob/main/guides/contributor/membership.md#approver).