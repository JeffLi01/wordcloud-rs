use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wordcloud_rs::{Token, WordCloud};

fn gen_counter(length: usize) -> HashMap<String, f32> {
    let mut counter = HashMap::new();
    for i in 0..length {
        counter.insert(format!("数量{}", i + 1), (i + 1) as f32);
    }
    counter
}

fn wordcloud(counter: &HashMap<String, f32>) {
    // Prepare the tokens
    let tokens = counter.into_iter()
        .map(|(k, v)| (Token::Text(k.to_string()), *v))
        .collect();
    // Generate the word-cloud
    let wc = WordCloud::new()
        .font(r#"/usr/share/fonts/truetype/wqy/wqy-microhei.ttc"#)
        .generate(tokens);
    // Save it
    let filename = format!("wordcloud-{}.png", counter.len());
    wc.save(filename).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("wordcloud");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(10);
    let counter = gen_counter(100);
    group.bench_function("wordcloud-100", |b| b.iter(|| wordcloud(black_box(&counter))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);