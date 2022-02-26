// https://21-lessons.com/ship-rust-tracing-spans-to-jaeger/
// https://docs.rs/opentelemetry-jaeger/latest/opentelemetry_jaeger/index.html

use opentelemetry::global;
use opentelemetry::trace::Tracer;
use std::error::Error;
use tracing::info;

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("my_app")
        .install_simple()?;

    tracer.in_span("doing_work", |_cx| {
        shave_all(10);
    });

    shave_all(15);

    global::shutdown_tracer_provider(); // export remaining spans

    Ok(())
}

#[tracing::instrument]
pub(crate) fn shave_all(number_of_yaks: i32) -> i32 {
    for i in 0..number_of_yaks {
        info!(current_yak = i + 1, "Shaving in progress");
        println!("Shaving in progress: {i}");
    }
    number_of_yaks
}
