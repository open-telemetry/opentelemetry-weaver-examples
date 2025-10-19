pub mod attributes;
pub mod metrics;

use anyhow::Result;
use opentelemetry::trace::{Span, Tracer};
use opentelemetry::{KeyValue, Value, global};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::env;

const WEAVER_EXAMPLE: &str = "weaver-example";

#[derive(Debug)]
pub struct ParamValue<'a>(&'a str);

impl From<&ParamValue<'_>> for Value {
    fn from(msg_val: &ParamValue<'_>) -> Self {
        let arg = msg_val.0;
        // Try to parse as integer first
        if let Ok(int_val) = arg.parse::<i64>() {
            return Value::I64(int_val);
        }

        // Try to parse as float
        if let Ok(float_val) = arg.parse::<f64>() {
            return Value::F64(float_val);
        }

        // Default to string
        Value::String(arg.to_string().into())
    }
}

fn init_tracer_provider() -> Result<SdkTracerProvider> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()?;
    Ok(SdkTracerProvider::builder()
        .with_resource(
            Resource::builder()
                .with_service_name(WEAVER_EXAMPLE)
                .build(),
        )
        .with_batch_exporter(exporter)
        .build())
}

fn init_meter_provider() -> Result<SdkMeterProvider> {
    let resource = Resource::builder()
        .with_service_name(WEAVER_EXAMPLE)
        .build();

    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .build()?;

    let reader = opentelemetry_sdk::metrics::PeriodicReader::builder(exporter).build();

    Ok(SdkMeterProvider::builder()
        .with_resource(resource)
        .with_reader(reader)
        .build())
}

fn get_hostname() -> String {
    hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

fn example_span(message: &ParamValue<'_>) {
    let tracer = global::tracer(WEAVER_EXAMPLE);
    let mut span = tracer
        .span_builder("example_message")
        .with_attributes(vec![
            KeyValue::new(attributes::EXAMPLE_MESSAGE, message),
            KeyValue::new(attributes::HOST_ARCH, std::env::consts::ARCH),
        ])
        .start(&tracer);

    let hostname = get_hostname();

    span.set_attribute(KeyValue::new(attributes::HOST_NAME, hostname));
}

fn example_metric(message_count: usize) {
    let meter = global::meter(WEAVER_EXAMPLE);
    let counter = meter
        .u64_counter(metrics::EXAMPLE_COUNTER)
        .with_unit("1")
        .build();

    counter.add(
        message_count as u64,
        &[
            KeyValue::new(attributes::HOST_NAME, get_hostname()),
            KeyValue::new(attributes::HOST_ARCH, std::env::consts::ARCH),
        ],
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    let messages: Vec<ParamValue> = if args.len() > 1 {
        args[1..].iter().map(|arg| ParamValue(arg)).collect()
    } else {
        vec![ParamValue("Hello, World!")]
    };

    let tracer_provider = init_tracer_provider()?;
    let _ = global::set_tracer_provider(tracer_provider.clone());

    let meter_provider = init_meter_provider()?;
    global::set_meter_provider(meter_provider.clone());

    for message in &messages {
        example_span(message);
    }

    example_metric(messages.len());

    tracer_provider.force_flush()?;
    meter_provider.shutdown()?;

    Ok(())
}
