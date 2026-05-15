use anitomy_pure::Parser;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

// Representative sample of real-world anime filenames covering common patterns:
// bracketed group, dotted format, seasons, movies, OVA, multi-episode, checksums.
const FILENAMES: &[&str] = &[
    "[HorribleSubs] Boku no Hero Academia - 01 [720p].mkv",
    "[Coalgirls]_Sword_Art_Online_01_(1280x720_Blu-ray_FLAC)_[75418F57].mkv",
    "[Hatsuyuki-Kaitou]_Fairy_Tail_2_-_52_(227)_[720p][10bit][9DF6B8D5].mkv",
    "[FFF] Love Live! The School Idol Movie [D1A15D2C].mkv",
    "The.Animatrix.08.A.Detective.Story.720p.BluRay.DTS.x264-ESiR.mkv",
    "[Tsundere] Hyouka - 01v2-04 [BDRip h264 1920x1080 10bit FLAC]",
    "[HorribleSubs] One Punch Man - 12 [1080p].mkv",
    "Attack on Titan - Episode 3 - A Dim Light Amid Despair.mkv",
    "[GS] Classroom Crisis Vol.1&2 (BD 1080p 10bit FLAC)",
    "[Nishi-Taku] Tamayura ~graduation photo~ Movie Part 1 [BD][720p][98965607].mkv",
    "[UTW]_Accel_World_-_EX01_[BD][h264-720p_AAC][3E56EE18].mkv",
    "[EveTaku] AKB0048 Vol.03 - Making of Kibou-ni-Tsuite Music Video (BDRip 1080i H.264-Hi10P FLAC)[C09462E2]",
    "Dragon_Ball_Z_Movies_8_&_10_[720p,BluRay,DTS,x264]_-_THORA",
    "[5F] RWBY 14 Forever Fall Part 2 pt-BR.mp4",
    "[BM&T] Toradora! - 07v2 - Pool Opening [720p Hi10 ] [BD] [8F59F2BA]",
];

fn bench_anitomy_pure(c: &mut Criterion) {
    c.bench_function("anitomy-pure", |b| {
        b.iter(|| {
            for filename in FILENAMES {
                let _ = black_box(Parser::new(black_box(filename)).parse());
            }
        })
    });
}

fn bench_anitomy_cpp(c: &mut Criterion) {
    let mut parser = anitomy::Anitomy::new();
    c.bench_function("anitomy (C++ wrapper)", |b| {
        b.iter(|| {
            for filename in FILENAMES {
                let _ = black_box(parser.parse(black_box(filename)));
            }
        })
    });
}

fn bench_hunch(c: &mut Criterion) {
    c.bench_function("hunch", |b| {
        b.iter(|| {
            for filename in FILENAMES {
                let _ = black_box(hunch::hunch(black_box(filename)));
            }
        })
    });
}

fn bench_zantetsu(c: &mut Criterion) {
    let engine = zantetsu::Zantetsu::new().expect("failed to init zantetsu");
    c.bench_function("zantetsu", |b| {
        b.iter(|| {
            for filename in FILENAMES {
                let _ = black_box(engine.parse(black_box(filename)));
            }
        })
    });
}

fn bench_per_filename(c: &mut Criterion) {
    let mut anitomy_cpp = anitomy::Anitomy::new();
    let zantetsu_engine = zantetsu::Zantetsu::new().expect("failed to init zantetsu");

    let mut group = c.benchmark_group("per_filename");
    for (i, filename) in FILENAMES.iter().enumerate() {
        let short_name = filename.chars().take(40).collect::<String>();
        let id = BenchmarkId::new("anitomy-pure", format!("[{i}] {short_name}"));
        group.bench_with_input(id, filename, |b, f| {
            b.iter(|| black_box(Parser::new(black_box(f)).parse()))
        });

        let id = BenchmarkId::new("anitomy (C++)", format!("[{i}] {short_name}"));
        group.bench_with_input(id, filename, |b, f| {
            b.iter(|| black_box(anitomy_cpp.parse(black_box(f))))
        });

        let id = BenchmarkId::new("hunch", format!("[{i}] {short_name}"));
        group.bench_with_input(id, filename, |b, f| {
            b.iter(|| black_box(hunch::hunch(black_box(f))))
        });

        let id = BenchmarkId::new("zantetsu", format!("[{i}] {short_name}"));
        group.bench_with_input(id, filename, |b, f| {
            b.iter(|| black_box(zantetsu_engine.parse(black_box(f))))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_anitomy_pure,
    bench_anitomy_cpp,
    bench_hunch,
    bench_zantetsu,
    bench_per_filename
);
criterion_main!(benches);
