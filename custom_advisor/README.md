# Live-check Custom Advisor Example

## Description

This example shows how to create and use a custom advisor with OpenTelemetry Weaver Live-check. A custom advisor allows you to implement your own validation logic for telemetry data against your semantic convention model.

## Prerequisites

Weaver version 0.19.0+ must be installed and available on your PATH.

## The Model

The semantic convention model is defined in [model/example.yaml](model/example.yaml). It defines a counter metric called `example.counter` with an attribute `example.name` that has a custom annotation:

```yaml
annotations:
  live_check:
    case: upper
```

This annotation specifies that the `example.name` attribute value must be UPPERCASE. It is used by the policy to validate incoming telemetry data.

## The Sample Data

The sample telemetry data is in [samples/metrics.json](samples/metrics.json) and contains two metric data points:

1. **Valid data point**: Has `example.name` set to `"ID1234"` (uppercase) - this should pass validation
2. **Invalid data point**: Has `example.name` set to `"lowercaseid"` (lowercase) - this should fail validation

## The Policy

The custom advisor policy is implemented in Rego and located at [policies/live_check_advice/upper.rego](policies/live_check_advice/upper.rego).

The policy checks:
1. If the attribute has the `live_check.case: upper` annotation in the model
2. If the attribute value is a string
3. If the value is not entirely uppercase

When these conditions are met, it produces a "violation" level advice with the type `invalid_value_case`, providing a clear message about which attribute value failed the uppercase requirement.

## Run the example
```shell
cd custom_advisor

weaver registry live-check -r model --input-source samples/metrics.json --advice-policies policies
```

The output shows each metric data point with its attributes, and any violations found:

```
Metric example.counter `counter`, `1`
    Data point 17
        example.name = ID1234

Metric example.counter `counter`, `1`
    Data point 18
        example.name = lowercaseid
            - [violation] Value 'lowercaseid' on attribute 'example.name' must be uppercase

Samples
  - total: 6
  - by type:
    - attribute: 2
    - data_point: 2
    - metric: 2
  - by highest advice level:
    - no advice: 5
    - violation: 1

Advisories given
  - total: 1
  - advice level:
    - violation: 1
  - advice type:
    - invalid_value_case: 1
  - advice message:
    - Value 'lowercaseid' on attribute 'example.name' must be uppercase: 1

Registry coverage
  - entities seen: 100.0%
```

The first metric with `"ID1234"` passes validation (uppercase), while the second metric with `"lowercaseid"` triggers a violation. The summary shows that 1 out of 6 samples had a violation, and the registry coverage is 100%.