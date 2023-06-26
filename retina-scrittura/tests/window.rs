// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#![feature(deadline_api)]

use std::{
    collections::HashMap,
    sync::mpsc::channel, time::{Instant, Duration},
};

use log::{error, info};
use retina_scrittura::test_helper::create_simple_context_and_document;

#[test]
fn runner() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/window");

    rayon::ThreadPoolBuilder::new()
        .stack_size(25 * 1024 * 1024)
        .build_global()
        .unwrap();

    _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();

    let (sender, receiver) = channel();

    let mut results = HashMap::new();

    for file in std::fs::read_dir(path).unwrap().flatten() {
        if file.file_name().to_string_lossy().ends_with(".js") {
            let path = file.path().clone();

            results.insert(path.clone(), None);
            let sender = sender.clone();

            rayon::spawn(move || {
                let mut context = create_simple_context_and_document();

                let script_source = std::fs::read_to_string(&path)
                    .expect(&format!("failed to read test data of: {}", path.display()));

                if let Err(error) = context.run_script_from_string_source(&script_source) {
                    _ = sender.send((path, Err(error.to_string())));
                } else {
                    _ = sender.send((path, Ok(())));
                }
            })
        }
    }

    drop(sender);

    let deadline = Instant::now().checked_add(Duration::from_secs(30)).unwrap();

    while let Ok((test_path, test_result)) = receiver.recv_deadline(deadline) {
        match results.entry(test_path) {
            std::collections::hash_map::Entry::Occupied(mut o) => {
                if o.get().is_some() {
                    panic!("Test {} returned a result twice!", o.key().display());
                }
                o.insert(Some(test_result));
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(Some(test_result));
            }
        }
    }

    let mut failed = 0;

    for (test_path, test_result) in results {
        match test_result {
            Some(Ok(())) => info!("PASSED  {}", test_path.display()),
            Some(Err(error)) => {
                error!("FAILED  {}: {}", test_path.display(), error);
                failed += 1;
            }
            None => error!("TIMEOUT {}", test_path.display()),
        }
    }

    if failed > 0 {
        panic!("{failed} test(s) failed");
    }
}
