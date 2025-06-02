# A Practical Guide to Telemetry in Rust

I hope this repository will serve as an entry point to generating telemetry in your Rust applications.
It contains a very simple web-server that should demonstrate how the logical "flow" in your code can be traced and visualized.

Note: Keep in mind that all topics touched in this repository are evolving quickly which means that by the time of reading some dependencies will most likely
be outdated.

## Disambiguation

So, what are traces, spans and metrics?

The `otel` [documentation](https://docs.rs/opentelemetry/latest/opentelemetry/#getting-started-with-traces) comes with a nice definition of a **trace**:
> A trace is a tree of Spans which are objects that represent the work being done by individual services or components involved in a request as it flows through a system.

In my words I would say: Traces help us understand how data and requests flow through our programs.

### Spans

I like to think of `Span`s as sections in my tracing. A `Span` has a start and an end - they enable us to mark the flow of our application in a meaningful way.
**Spans** are just objects that carry metadata (like line numbers, IDs etc.) that we might be interested in.
They can be nested and are a great way of dividing the flow of our program into separate steps.

Lets say a flow in my application looks like this:

```text
A: Request comes in
B: User is authenticated
C: Data of user is queried, transformed and sent back
```

I might build a span around `B` just to group all telemetry events produced during that step together. As we see in the Grafana visualization, spans also help a great deal with
showing how much time each step of the logical flow has taken.

### Metrics

Moving away from individual requests or happenings in the code, we might be interested in overall statistics (like requests per minute, the distribution of latencies of our responses etc.).
Here **metrics** come into play. They keep track of the quantifiable aspects of our program and help us find bottlenecks/optimization strategies etc.

## Stack

For the purpose of this presentation/repository, "telemetry" will simply mean data that is collected and shared in the form specified by `OpenTelemetry` (short **OTEL**, see their [website](https://opentelemetry.io/)).

There is a whole zoo of crates in the Rust crates ecosystem we can use to instrument our code to emit telemetry data (it is quite a lot to take in, [have a look](https://github.com/open-telemetry/opentelemetry-rust/tree/main)).

As Rust developers, we will focus on generating telemetry-data from within our application. This repository contains only some very rough instructions on collecting and visualizing the generated data using **Grafana**.

### Setting up Grafana

So, let's get the visualizing bit out of the way first. Please follow the instructions in this repository <https://github.com/grafana/docker-otel-lgtm>.

Thanks to the magic of containers, when everything works as expected, you should be able to open the Grafana web interface at `localhost:3000`.

### Setting up OTEL

Let's ignore the whole `Pepsi` vs. `Cola` debate (see the [discussion](https://github.com/open-telemetry/opentelemetry-rust/issues/1571) about the question what to do with co-existing `tokio-tracing` and `otel` APIs to create spans etc.) and move forward with the following mindset:

- We use the `tracing` crate of the `tokio` ecosystem to create/annotate spans. We are aware of issues that arise from mixing `tokio-tracing` and `otel` APIs ([#1690](https://github.com/open-telemetry/opentelemetry-rust/issues/1690#issue-2270527939)), so we will not be mixing those :-)
- We use the `otel-sdk` to create and record metrics.

Why use the `tokio-tracing` API at all? Because I really like it and I hope you will too :-)

todo!: Configuration, Startup, Instrumentation (Spans), Metrics ==> Easy dashboard

#### Sending Logs

For now, this section remains a todo item.

#### Sending Spans/Events

Check the code at [otel.rs](./src/otel.rs) for the specific steps to take for the otel setup.

Essentially, we are setting up a tracing subscriber with a layer, that

1. Attaches some useful metadata for every span/event:

    ```Rust
        .with_id_generator(RandomIdGenerator::default())
            .with_resource(resource()) // later this will help to understand where exactly a trace came from
    ```

2. Exports all generated data in batches `.with_batch_exporter(exporter)`.
3. Transmits the data via gRPC `.with_tonic()`.

Here we build a tracing subscriber registry that prints our output to `stdout` and also uses the tracing layer we just created:

```Rust
    .with(tracing_subscriber::fmt::layer().with_line_number(true))
        .with(OpenTelemetryLayer::new(tracer))
```

#### Sending Metrics

We set up the metrics layer in the same way we set up the tracing layer.

We create the exporter in the function `fn init_meter_provider() -> SdkMeterProvider { /*..*/` and then add the layer to our tracing registry.

```Rust
    .with(MetricsLayer::new(meter_provider.clone()))
```

As we have set up the metrics exporter so that it also logs to `stdout`, we can see the output in the terminal:

```text
Metric #0
    Name         : http.server.latency
    Description  : Latency of HTTP requests
    Unit         : us
    Type         : Histogram
    Temporality  : Cumulative
    StartTime    : 2025-06-02 15:43:28.430779
    EndTime      : 2025-06-02 15:48:55.829332
    Histogram DataPoints
    DataPoint #0
            Count        : 12
            Sum          : 4204.0
            Min          : 265.0
            Max          : 554.0
            Attributes   :
            Buckets
                        -inf to 0 : 0
                        0 to 5 : 0
                        5 to 10 : 0
                        10 to 25 : 0
                        25 to 50 : 0
                        50 to 75 : 0
                        75 to 100 : 0
                        100 to 250 : 0
                        250 to 500 : 11
                        500 to 750 : 1
                        750 to 1000 : 0
                        1000 to 2500 : 0
                        2500 to 5000 : 0
                        5000 to 7500 : 0
                        7500 to 10000 : 0
                    10000 to +Infinity : 0

```

Here we see, that there have been 11 requests that have taken 250 to 500 micro seconds to resolve, and 1 request that has taken 500 to 750 micro seconds to resolve.

[This very simple Grafana dashboard](./dashboard.json) renders a simple bar chart (histogram) that visualizes the data.

Follow the steps described [in in Grafana docs](https://grafana.com/docs/grafana/latest/dashboards/build-dashboards/import-dashboards/) to import the dashboard.

## Test and run

1. First start the environment as described in [Setting up Grafana](#setting-up-grafana). I just `cd` into the cloned repository and run `./run-lgtm.sh`.
2. Then run `cargo run` in this repository.
3. Then you can send HTTP requests to the endpoints I have prepared and observe the terminal, and the Grafana output.
Examples: `curl localhost:5173/hello`, `curl -X POST localhost:5173/users/add/mert` etc.

The Grafana frontend should be available at localhost:3000. To see the most recent spans, click on `Explore` in the left sidebar, select `Tempo` in the dropdown on the top and click on the `Search` tab to see the page that lists the most recent spans.
