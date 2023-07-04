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
    gg4_circular1,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/p1234_ensure_that_p1234_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg4_circular2,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/p1234_ensure_p1_gets_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg4_circular3,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/p1234_ensure_that_only_p1_gets_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg4_circular4,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/p1234_eventually_omniscient_TRUE.props"
);

bench!(
    gg4_circular5,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/eventually_10_steps_are_passed_TRUE.props"
);

bench!(
    gg4_circular6,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/p1_guarantee_omniscience_before_10_steps_FALSE.props"
);

bench!(
    gg4_circular7,
    "gossipping_girls/gossipping_girls_circular_4.prism",
    "gossipping_girls/p234_eventually_omniscient_without_p1_TRUE.props"
);

bench!(
    gg5_circular1,
    "gossipping_girls/gossipping_girls_circular_5.prism",
    "gossipping_girls/p12345_ensure_that_p12345_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg5_circular2,
    "gossipping_girls/gossipping_girls_circular_5.prism",
    "gossipping_girls/p12345_ensure_p1_gets_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg5_circular3,
    "gossipping_girls/gossipping_girls_circular_5.prism",
    "gossipping_girls/p12345_ensure_that_only_p1_gets_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg5_circular4,
    "gossipping_girls/gossipping_girls_circular_5.prism",
    "gossipping_girls/p12345_eventually_omniscient_TRUE.props"
);

bench!(
    gg5_circular5,
    "gossipping_girls/gossipping_girls_circular_5.prism",
    "gossipping_girls/eventually_10_steps_are_passed_TRUE.props"
);

bench!(
    gg5_circular6,
    "gossipping_girls/gossipping_girls_circular_5.prism",
    "gossipping_girls/p1_guarantee_omniscience_before_10_steps_FALSE.props"
);

bench!(
    gg5_circular7,
    "gossipping_girls/gossipping_girls_circular_5.prism",
    "gossipping_girls/p2345_eventually_omniscient_without_p1_TRUE.props"
);

bench!(
    gg6_circular1,
    "gossipping_girls/gossipping_girls_circular_6.prism",
    "gossipping_girls/p123456_ensure_that_p123456_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg6_circular2,
    "gossipping_girls/gossipping_girls_circular_6.prism",
    "gossipping_girls/p123456_ensure_p1_gets_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg6_circular3,
    "gossipping_girls/gossipping_girls_circular_6.prism",
    "gossipping_girls/p123456_ensure_that_only_p1_gets_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg6_circular4,
    "gossipping_girls/gossipping_girls_circular_6.prism",
    "gossipping_girls/p123456_eventually_omniscient_TRUE.props"
);

bench!(
    gg6_circular5,
    "gossipping_girls/gossipping_girls_circular_6.prism",
    "gossipping_girls/eventually_10_steps_are_passed_TRUE.props"
);

bench!(
    gg6_circular6,
    "gossipping_girls/gossipping_girls_circular_6.prism",
    "gossipping_girls/p1_guarantee_omniscience_before_10_steps_FALSE.props"
);

bench!(
    gg6_circular7,
    "gossipping_girls/gossipping_girls_circular_6.prism",
    "gossipping_girls/p23456_eventually_omniscient_without_p1_TRUE.props"
);

bench!(
    gg4_total1,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/p1234_ensure_that_p1234_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg4_total2,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/p1234_ensure_p1_gets_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg4_total3,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/p1234_ensure_that_only_p1_gets_omniscient_before_10_steps_TRUE.props"
);

bench!(
    gg4_total4,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/p1234_eventually_omniscient_TRUE.props"
);

bench!(
    gg4_total5,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/eventually_10_steps_are_passed_TRUE.props"
);

bench!(
    gg4_total6,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/p1_guarantee_omniscience_before_10_steps_FALSE.props"
);

bench!(
    gg4_total7,
    "gossipping_girls/gossipping_girls_total_network_4.prism",
    "gossipping_girls/p234_eventually_omniscient_without_p1_TRUE.props"
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

bench!(
    mexican_standoff_5p_3hp_lcgs_survive_threads,
    "mexican_standoff/mexican_standoff_5p_3hp.prism",
    "mexican_standoff/can_p1_guarantee_to_survive_FALSE.props"
);

bench!(
    mexican_standoff_5p_3hp_lcgs_suicide_threads,
    "mexican_standoff/mexican_standoff_5p_3hp.prism",
    "mexican_standoff/can_p1_suicide_FALSE.props"
);


// Rock paper scissors
bench!(
    rps2,
    "rock_paper_scissors/rock_paper_scissors.prism",
    "rock_paper_scissors/p1_can_win_eventually_FALSE.props"
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
    gg4_circular1,
    gg4_circular2,
    gg4_circular3,
    gg4_circular4,
    gg4_circular5,
    gg4_circular6,
    gg4_circular7,
    gg5_circular1,
    gg5_circular2,
    gg5_circular3,
    gg5_circular4,
    gg5_circular5,
    gg5_circular6,
    gg5_circular7,
    gg6_circular1,
    gg6_circular2,
    gg6_circular3,
    gg6_circular4,
    gg6_circular5,
    gg6_circular6,
    gg6_circular7,
    gg4_total1,
    gg4_total2,
    gg4_total3,
    gg4_total4,
    gg4_total5,
    gg4_total6,
    gg4_total7,
);

criterion_group!(
    mexican_standoff,
    mexican_standoff_3p_3hp_lcgs_survive_threads,
    mexican_standoff_3p_3hp_lcgs_suicide_threads,
    mexican_standoff_5p_1hp_lcgs_survive_threads,
    mexican_standoff_5p_1hp_lcgs_suicide_threads,
    mexican_standoff_5p_3hp_lcgs_suicide_threads,
    mexican_standoff_5p_3hp_lcgs_suicide_threads,
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
