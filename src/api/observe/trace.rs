use std::str::FromStr;

use anyhow::Context;
use opentelemetry::{global, trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::{WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::{
    runtime,
    trace::{RandomIdGenerator, Sampler, TracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::{
    resource::{SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};

use tonic::metadata::{MetadataMap, MetadataValue};
use tracing::level_filters::LevelFilter;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::settings::{Otlp, Settings};

fn resource() -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        ],
        SCHEMA_URL,
    )
}

fn init_tracer_provider(otlp: Option<&Otlp>) -> TracerProvider {
    let mut builder = opentelemetry_otlp::SpanExporter::builder().with_tonic();
    if let Some(otlp) = otlp {
        builder = builder
            .with_endpoint(otlp.endpoint.clone())
            .with_protocol(opentelemetry_otlp::Protocol::Grpc);
        if let Some(interval) = otlp.interval {
            builder = builder.with_timeout(interval);
        }
        if let (Some(token), Some(org), Some(stream_name)) =
            (&otlp.token, &otlp.organization, &otlp.stream)
        {
            let mut metadata = MetadataMap::new();
            metadata.insert(
                "authorization",
                MetadataValue::from_str(&format!("Basic {}", token)).unwrap(),
            );
            metadata.insert("organization", MetadataValue::from_str(org).unwrap());
            metadata.insert("stream-name", stream_name.parse().unwrap());
            builder = builder.with_metadata(metadata);
        }
    }
    let exporter = builder
        .build()
        .context("build otlp tracer exporter failed")
        .unwrap();

    TracerProvider::builder()
        // Customize sampling strategy
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0,
        ))))
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource())
        .with_batch_exporter(exporter, runtime::Tokio)
        .build()
}

pub fn init_tracing_subscriber(setting: &Settings) -> OtelGuard {
    let level = if setting.debug.unwrap_or(false) {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };

    let otlp = setting.otlp.as_ref();

    // global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer_provider = init_tracer_provider(otlp);
    // let meter_provider = init_meter_provider(otlp);
    // let logger_provider = init_log_provider(otlp);

    let tracer = tracer_provider.tracer("tracing-otel-subscriber");

    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(level.into())
                .from_env_lossy()
                .add_directive("hyper=error".parse().unwrap())
                .add_directive("tower=error".parse().unwrap())
                .add_directive("h2=error".parse().unwrap())
                .add_directive("opentelemetry_sdk=error".parse().unwrap())
                .add_directive("sqlx=error".parse().unwrap()),
        )
        // .with(tracing_subscriber::fmt::layer().with_target(false))
        .with(tracing_subscriber::fmt::layer().with_line_number(true))
        // .with(MetricsLayer::new(meter_provider.clone()))
        .with(OpenTelemetryLayer::new(tracer))
        // .with(
        //     opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(
        //         &logger_provider.clone(),
        //     ),
        // )
        .init();

    OtelGuard {
        tracer_provider,
        // meter_provider,
        // logger_provider,
    }
}

pub struct OtelGuard {
    tracer_provider: TracerProvider,
    // meter_provider: SdkMeterProvider,
    // logger_provider: LoggerProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("{err:?}");
        }
        // if let Err(err) = self.meter_provider.shutdown() {
        //     eprintln!("{err:?}");
        // }
        // if let Err(err) = self.logger_provider.shutdown() {
        //     eprintln!("{err:?}");
        // }
        global::shutdown_tracer_provider();
    }
}
