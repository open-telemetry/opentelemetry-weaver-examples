# Basic Weaver Example

## Description

This example demonstrates practical usage of weaver to define a semantic convention model, generate code and documentation from it, and validate telemetry emitted by an example application against the model in real-time.

## Prerequisites

Weaver must be installed and available on your PATH. To run the example application, you must also have Rust and Cargo installed. See https://www.rust-lang.org/tools/install for instructions.

## Semantic convention model

In the `model` directory is a model that defines attributes, spans and metrics. The OpenTelemetry Semantic Conventions project already defines a large set of attributes and signals which you can reference by declaring a dependency in `registry_manifest.yaml`.

To assist with authoring these yaml files, most modern IDEs support json schemas that provide inline feedback, hints and completion. This project contains an example for vscode. Weaver will also report any schema errors when you run any of its `registry` commands.

### Check the model for errors

The schema can only express fundamental structural requirements for the model. `check` digs deeper to ensure the model is valid. (Note that `check` is run implicitly for all `registry` commands too).

`weaver registry check -r model`

### Get a resolved schema

It's often useful to see how you model has been resolved to a registry with this command. This is useful when you're using references, more so if those references are to a dependency.

`weaver registry resolve -r model`

### Generate documentation

Weaver can pass the resolved schema to jinja templates to render documentation. You can write your own, or reference templates defined elsewhere. This example uses the templates defined in the OpenTelemetry Semantic Conventions project to generate the docs directory and content.

`weaver registry generate -r model --templates "https://github.com/open-telemetry/semantic-conventions/archive/refs/tags/v1.37.0.zip[templates]" markdown docs`

### Generate code

Like the documentation, Weaver uses jinja templates to create code. In this case, custom templates are defined in this project. The command below will create the `attributes.rs` and `metrics.rs` files in the `src` directory right alongside the application code.

`weaver registry generate -r model --templates templates rust src`

### Emit example telemetry

It's useful to see what your telemetry, as defined in your model, will look like in observability tools. `emit` generates OTLP with example data for each signal defined in your model. 

To visualize the generated data, an example tool is [`otel-desktop-viewer`](https://github.com/CtrlSpice/otel-desktop-viewer) - a CLI tool for receiving OpenTelemetry Traces locally:

`otel-desktop-viewer --browser-port 8001`

Then in another shell:

`weaver registry emit -r model`

## Example application

The rust application sends spans and metrics to demonstrate the live-check feature of weaver. `live-check` acts like an OpenTelemetry Collector - it receives telemetry in OTLP and analyzes its compatibility with a model. We can use this in tests locally and in CI to regression test our application's instrumentation.

The application has been contrived so you can provoke a violation in `live-check`. The `example.message` attribute is defined with type `string` but you can produce an `int` or `float` by providing these in the command line parameters: `cargo run -- 42` will send an `int`. `live-check` will return a violation for this type mismatch and exit with code 1.

The `live-check.rs` unit test checks for pass and fail by providing strings and ints as above.

### Use live-check on the command line

`weaver registry live-check -r model --inactivity-timeout 20`

In another shell:

`cargo run -- foo 42`
