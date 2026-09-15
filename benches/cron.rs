//! Cron expression benchmarks.
//!
//! Times `armature-cron` expression parsing for hand-written and preset
//! schedules. No scheduler, timer or job store is involved - this is the
//! parser and its supporting string handling only.
//!
//! ```bash
//! cargo bench -p armature-cron --bench cron
//! ```

use armature_cron::{CronExpression, expression::CronPresets};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_cron_expression_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("cron_expression");

    let expressions = [
        "0 0 * * * *",        // Every hour
        "*/5 * * * * *",      // Every 5 seconds
        "0 0 12 * * MON-FRI", // Weekdays at noon
        "0 0 0 1 * *",        // First of month
    ];

    for expr in expressions {
        group.bench_with_input(BenchmarkId::from_parameter(expr), expr, |b, expr| {
            b.iter(|| CronExpression::parse(black_box(expr)).unwrap())
        });
    }

    group.bench_function("parse_preset_hourly", |b| {
        b.iter(|| CronExpression::parse(black_box(CronPresets::EVERY_HOUR)).unwrap())
    });

    group.bench_function("parse_preset_daily", |b| {
        b.iter(|| CronExpression::parse(black_box(CronPresets::DAILY)).unwrap())
    });

    group.bench_function("string_split", |b| {
        let expr = "0 0 * * * *";
        b.iter(|| {
            let parts: Vec<&str> = black_box(expr).split_whitespace().collect();
            black_box(parts);
        })
    });

    group.finish();
}

criterion_group!(cron_benches, bench_cron_expression_parsing);

criterion_main!(cron_benches);
