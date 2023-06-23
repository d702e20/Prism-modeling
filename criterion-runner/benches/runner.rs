#![allow(non_snake_case)]
#![allow(dead_code)]

use criterion::*;
use std::process::Command;

const PRISM_PATH: &str = "../../prism-games/prism/bin/prism";
const PATH_PREFIX_PRISM_BENCHES: &str = "../benchmarking/";
const STDOUT_PATH_PREFIX: &str = "../stdout/";

macro_rules! bench {
    ($name:ident, $model:expr, $formula:expr) => {
        fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($name));
            group.sampling_mode(SamplingMode::Flat);
            group.sample_size(10);
            group.bench_function(stringify!($name), |b| {
                b.iter(|| {
                    // pwd is: /home/user/projects/Prism-modeling/criterion-runner
                    let stdout_path = format!("{}{}.txt", STDOUT_PATH_PREFIX, stringify!($name));
                    let cmd = Command::new("sh")
                        .arg("-c")
                        .arg(format!(
                            "{} {}{} {}{} >> {}",
                            PRISM_PATH,
                            PATH_PREFIX_PRISM_BENCHES,
                            stringify!($model),
                            PATH_PREFIX_PRISM_BENCHES,
                            stringify!($formula),
                            stdout_path
                        ))
                        .output()
                        .expect("failed to execute test");
                    if !cmd.status.success() {
                        panic!(
                            "Prism bench on '{}' exited with status: {:?}\nstdout: {}\nstderr: {}",
                            stringify!($name),
                            cmd.status,
                            String::from_utf8_lossy(&cmd.stdout),
                            String::from_utf8_lossy(&cmd.stderr)
                        );
                    } else {
                        println!("#BEGIN# {}: {}\n#END#", stringify!($name), String::from_utf8_lossy(&cmd.stdout));
                    }
                })
            });
            group.finish();
        }
    };
}

// Matching pennies
bench!(
    mp1,
    "matching_pennies/matching_pennies.prism",
    "matching_pennies/can_odd_win_round_eventually_FALSE.props"
);

bench!(
    mp2,
    "matching_pennies/matching_pennies.prism",
    "matching_pennies/can_they_guarantee_that_odd_always_has_larger_sum_TRUE.props"
);

bench!(
    mp3,
    "matching_pennies/matching_pennies.prism",
    "matching_pennies/can_they_win_simultaneously_FALSE.props"
);

// gossiping girls
bench!(
    gg1_circular,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/all_girls_ensure_that_all_girls_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg2_circular,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/all_girls_ensure_player_one_gets_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg3_circular,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/all_girls_ensure_that_only_player_one_gets_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg4_circular,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/all_omniscient_but_first_after_10_steps_TRUE.props"
);

bench!(
    gg5_circular,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/eventually_10_steps_are_passed_TRUE.props"
);

bench!(
    gg6_circular,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/girl_one_guarantee_to_become_omniscient_before_10_steps_FALSE.props"
);

bench!(
    gg7_circular,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/guarantee_all_girls_eventually_become_omniscient_but_not_girl_one_TRUE.props"
);

bench!(
    gg1_total,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/all_girls_ensure_that_all_girls_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg2_total,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/all_girls_ensure_player_one_gets_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg3_total,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/all_girls_ensure_that_only_player_one_gets_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg4_total,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/all_omniscient_but_first_after_10_steps_TRUE.props"
);

bench!(
    gg5_total,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/eventually_10_steps_are_passed_TRUE.props"
);

bench!(
    gg6_total,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/girl_one_guarantee_to_become_omniscient_before_10_steps_FALSE.props"
);

bench!(
    gg7_total,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/guarantee_all_girls_eventually_become_omniscient_but_not_girl_one_TRUE.props"
);

// Mexican standoff
bench!(
    mexican_standoff_3p_3hp_lcgs_survive_threads,
    "mexican_standoff/mexican_standoff_3p_3hp.prism",
    "mexican_standoff/can_p1_guarantee_to_survive_FALSE.props"
);

bench!(
    mexican_standoff_3p_3hp_lcgs_suicide_threads,
    "mexican_standoff/mexican_standoff_3p_3hp.prism",
    "mexican_standoff/can_p1_suicide_FALSE.props"
);

bench!(
    mexican_standoff_5p_1hp_lcgs_survive_threads,
    "mexican_standoff/mexican_standoff_5p_1hp.prism",
    "mexican_standoff/can_p1_guarantee_to_survive_FALSE.props"
);

bench!(
    mexican_standoff_5p_1hp_lcgs_suicide_threads,
    "mexican_standoff/mexican_standoff_5p_1hp.prism",
    "mexican_standoff/can_p1_suicide_FALSE.props"
);


// Rock paper scissors
bench!(
    rps2,
    "rock_paper_scissors/rock_paper_scissors.prism",
    "rock_paper_scissors/p1_can_win_evantuallly_FALSE.props"
);
bench!(
    rps1,
    "rock_paper_scissors/rock_paper_scissors.prism",
    "rock_paper_scissors/p1_never_lose_FALSE.props"
);

criterion_group!(
    matching_pennies,
    mp1,
    mp2,
    mp3,
);

criterion_group!(
    gossiping_girls,
    gg1_circular,
    gg2_circular,
    gg3_circular,
    gg4_circular,
    gg5_circular,
    gg6_circular,
    gg7_circular,
    gg1_total,
    gg2_total,
    gg3_total,
    gg4_total,
    gg5_total,
    gg6_total,
    gg7_total,
);

criterion_group!(
    mexican_standoff,
    mexican_standoff_3p_3hp_lcgs_survive_threads,
    mexican_standoff_3p_3hp_lcgs_suicide_threads,
    mexican_standoff_5p_1hp_lcgs_survive_threads,
    mexican_standoff_5p_1hp_lcgs_suicide_threads,
);

criterion_group!(
    rock_paper_scissors,
    rps1,
    rps2
);

criterion_main!(
    mexican_standoff,
    matching_pennies,
    gossiping_girls,
    rock_paper_scissors
);
