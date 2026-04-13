# Postgres Query Analyzer

![Status](https://img.shields.io/badge/Status-In%20Development-yellow)

## Overview

A high-performance query analysis tool built in Rust that ingests PostgreSQL query logs and produces actionable insights, with a focus on detecting inefficient query patterns such as N+1 queries, redundant lookups, and missing indexes.

This tool is designed to bridge the gap between database-level metrics and application-level fixes (e.g., Django ORM optimizations).

---

## Goals

* Detect and group queries by normalized structure (fingerprinting)
* Identify performance anti-patterns:

  * N+1 queries
  * Sequential scans on large tables
  * High-frequency low-latency queries (inefficient repetition)
* Provide actionable recommendations
* Operate in near real-time with low resource overhead
* Remain framework-agnostic but optionally provide Django-specific hints

---

## Non-Goals

* Acting as a full APM (Application Performance Monitoring) system
* Replacing existing observability stacks (Prometheus, OpenTelemetry)
* Deep SQL execution plan optimization

---

## Architecture

```
                ┌──────────────────────┐
                │ PostgreSQL Logs /    │
                │ pg_stat_statements   │
                └──────────┬───────────┘
                           │
                           ▼
                ┌──────────────────────┐
                │ Ingestion Layer      │
                │ (log tail / stream)  │
                └──────────┬───────────┘
                           │
                           ▼
                ┌──────────────────────┐
                │ Parser Layer         │
                │ (SQL normalization)  │
                └──────────┬───────────┘
                           │
                           ▼
                ┌──────────────────────┐
                │ Aggregation Engine   │
                │ (fingerprinting,     │
                │ metrics collection)  │
                └──────────┬───────────┘
                           │
                           ▼
                ┌──────────────────────┐
                │ Pattern Detection    │
                │ (N+1, scans, etc.)   │
                └──────────┬───────────┘
                           │
                           ▼
                ┌──────────────────────┐
                │ Output Layer         │
                │ (CLI / JSON / HTTP)  │
                └──────────────────────┘
```

---

## Project Structure

```
query-intel/
├── Cargo.toml
├── README.md
├── config/
│   └── default.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   │
│   ├── ingestion/
│   │   ├── mod.rs
│   │   ├── logfile.rs        # Tail PostgreSQL logs
│   │   ├── pg_stat.rs        # Pull from pg_stat_statements
│   │
│   ├── parser/
│   │   ├── mod.rs
│   │   ├── normalize.rs      # Query normalization
│   │   ├── fingerprint.rs    # Query hashing
│   │
│   ├── aggregation/
│   │   ├── mod.rs
│   │   ├── metrics.rs        # Counters, latency stats
│   │   ├── store.rs          # In-memory store (HashMap)
│   │
│   ├── detection/
│   │   ├── mod.rs
│   │   ├── n_plus_one.rs     # Detect repeated patterns
│   │   ├── sequential_scan.rs
│   │   ├── frequency.rs      # High-frequency queries
│   │
│   ├── output/
│   │   ├── mod.rs
│   │   ├── cli.rs            # Terminal output
│   │   ├── json.rs           # Structured output
│   │   ├── http.rs           # Optional API server
│   │
│   ├── models/
│   │   ├── mod.rs
│   │   ├── query.rs
│   │   ├── pattern.rs
│   │
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── time.rs
│   │   ├── logging.rs
│   │
│   └── config.rs
│
├── tests/
│   ├── integration_tests.rs
│
└── scripts/
    └── sample_logs.sql
```

---

## Core Concepts

### Query Fingerprinting

Normalize queries by removing variable values:

```sql
SELECT * FROM users WHERE id = 123;
SELECT * FROM users WHERE id = 456;
```

Becomes:

```sql
SELECT * FROM users WHERE id = ?;
```

This allows grouping and aggregation.

---

### Metrics Collected

* Execution count
* Average latency
* Max latency
* Standard deviation
* Time window frequency

---

### Pattern Detection

#### N+1 Detection

Heuristic:

* Same query fingerprint
* High frequency within short time window
* Low variance in structure

#### Sequential Scan Detection

* Parse EXPLAIN logs or pg_stat
* Detect missing index usage

#### High-Frequency Queries

* Identify queries executed excessively
* Suggest caching or batching

---

## Example Output

```
[WARNING] Potential N+1 Query Detected
Query: SELECT * FROM author WHERE id = ?
Calls: 12,431
Avg Time: 2ms

Recommendation:
Use select_related("author") in Django ORM
```

---

## Configuration

`config/default.toml`

```toml
[ingestion]
mode = "logfile"
log_path = "/var/log/postgresql/postgresql.log"

[analysis]
window_seconds = 60
n_plus_one_threshold = 100

[output]
mode = "cli"
```

---

## Dependencies (Planned)

* tokio (async runtime)
* sqlparser (SQL parsing)
* serde (serialization)
* clap (CLI interface)
* tracing (logging)

---

## MVP Scope

1. Log ingestion (file tailing)
2. Query normalization + fingerprinting
3. Basic aggregation (count + avg latency)
4. N+1 detection heuristic
5. CLI output

---

## Future Enhancements

* Web dashboard
* PostgreSQL extension integration
* Redis support for distributed aggregation
* Machine learning-based anomaly detection
* Django-specific plugin

---

## Development Roadmap

### Phase 1

* Project scaffolding
* Log ingestion
* Basic parsing

### Phase 2

* Aggregation engine
* Fingerprinting

### Phase 3

* Pattern detection (N+1)
* CLI output

### Phase 4

* Optimization + testing
* Packaging and release

---

## License

MIT
