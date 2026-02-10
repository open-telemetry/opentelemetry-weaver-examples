# Generate OTel Collector Transform Processor Config

This example shows how to generate [transform processor config](https://github.com/open-telemetry/opentelemetry-collector-contrib/tree/main/processor/transformprocessor) to include all attribute and metric renames.
Run

```bash
weaver registry generate -r https://github.com/open-telemetry/semantic-conventions/archive/refs/tags/v1.39.0.zip[model] --templates ./templates --v2  yaml ./output --param next_version=1.40.0
```

Check out [output](./output/transformation-processor.yaml).
Depending on the registry size and amount of renames it may or may not be a practical approach.
