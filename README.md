# Parallel Data Processing Pipeline

## Overview
A high-performance parallel data processing system built in Rust, showcasing concurrent programming patterns.

## Features
- Multi-stage data processing pipeline
- Configurable worker pools
- Async processing with Tokio
- Thread-safe communication
- Robust error handling

## Installation

First, ensure you have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Clone and build the project:
```bash
git clone https://github.com/yourusername/parallel-processor
cd parallel-processor
cargo build --release
```


## Project Structure:
```bash
src/
├── main.rs         # Entry point
├── pipeline.rs     # Pipeline implementation
├── worker.rs       # Worker logic
├── data.rs         # Data structures
└── error.rs        # Error handling
```

## Project Structure:
```bash
[dependencies]
tokio = { version = "1.28", features = ["full"] }
rand = "0.8"
```

### Architecture
## Pipeline Stages

``` bash
// Configure pipeline stages
pipeline.add_stage(Stage::new("Data Generation", 4));
pipeline.add_stage(Stage::new("Transformation", 3));
pipeline.add_stage(Stage::new("Analysis", 2));
pipeline.add_stage(Stage::new("Aggregation", 1));
```

## Data Flow
```bash
graph TD
    A[Data Generation - 4 workers] --> B[Transformation - 3 workers]
    B --> C[Analysis - 2 workers]
    C --> D[Aggregation - 1 worker]
```




