#![allow(non_snake_case)]
#![allow(dead_code)]

use criterion::*;
use std::process::Command;

const PRISM_PATH: &str = "../prism-games-3.0-linux64/bin/prism";
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
    matching_pennies_can_odd_win_round_eventually_FALSE,
    "matching-pennies/matching-pennies.prism",
    "matching-pennies/can_odd_win_round_eventually_FALSE.props"
);
bench!(
    matching_pennies_can_they_guarantee_that_odd_always_has_larger_sum_TRUE,
    "matching-pennies/matching-pennies.prism",
    "matching-pennies/can_they_guarantee_that_odd_always_has_larger_sum_TRUE.props"
);
bench!(
    matching_pennies_can_they_win_simultaneously_FALSE,
    "matching-pennies/matching-pennies.prism",
    "matching-pennies/can_they_win_simultaneously_FALSE.props"
);

// Gossiping girls circular
bench!(
    gossipping_girls_circular_all_girls_ensure_player_one_gets_omniscient_before_10_steps_TRUE,
    "gossipping-girls/gossipping-circular.prism",
    "gossipping-girls/all_girls_ensure_player_one_gets_omniscient_before_10_steps_TRUE.props"
);
bench!(
    gossipping_girls_circular_all_girls_ensure_that_all_girls_omnicient_before_10_steps_TRUE,
    "gossipping-girls/gossipping-circular.prism",
    "gossipping-girls/all_girls_ensure_that_all_girls_omnicient_before_10_steps_TRUE.props"
);
bench!(gossipping_girls_circular_all_girls_ensure_that_only_player_one_gets_omniscient_before_10_steps_TRUE,
    "gossipping-girls/gossipping-circular.prism",
    "gossipping-girls/all_girls_ensure_that_only_player_one_gets_omniscient_before_10_steps_TRUE.props");
bench!(
    gossipping_girls_circular_all_omniscient_but_first_after_10_steps_TRUE,
    "gossipping-girls/gossipping-circular.prism",
    "gossipping-girls/all_omniscient_but_first_after_10_steps_TRUE.props"
);
bench!(
    gossipping_girls_circular_eventually_10_steps_are_passed_TRUE,
    "gossipping-girls/gossipping-circular.prism",
    "gossipping-girls/eventually_10_steps_are_passed_TRUE.props"
);
bench!(
    gossipping_girls_circular_girl_one_gurantee_to_become_omniscient_before_10_steps_FALSE,
    "gossipping-girls/gossipping-circular.prism",
    "gossipping-girls/girl_one_gurantee_to_become_omniscient_before_10_steps_FALSE.props"
);
bench!(
    gossipping_girls_circular_guarantee_all_girls_eventually_become_omniscient_but_not_girl_one_TRUE,
    "gossipping-girls/gossipping-circular.prism",
    "gossipping-girls/guarantee_all_girls_eventually_become_omniscient_but_not_girl_one_TRUE.props"
);

// Gossiping girls total

bench!(
    gossipping_girls_total_all_girls_ensure_player_one_gets_omniscient_before_10_steps_TRUE,
    "gossipping-girls/gossipping-total.prism",
    "gossipping-girls/all_girls_ensure_player_one_gets_omniscient_before_10_steps_TRUE.props"
);
bench!(
    gossipping_girls_total_all_girls_ensure_that_all_girls_omnicient_before_10_steps_TRUE,
    "gossipping-girls/gossipping-total.prism",
    "gossipping-girls/all_girls_ensure_that_all_girls_omnicient_before_10_steps_TRUE.props"
);
bench!(gossipping_girls_total_all_girls_ensure_that_only_player_one_gets_omniscient_before_10_steps_TRUE,
    "gossipping-girls/gossipping-total.prism",
    "gossipping-girls/all_girls_ensure_that_only_player_one_gets_omniscient_before_10_steps_TRUE.props");
bench!(
    gossipping_girls_total_all_omniscient_but_first_after_10_steps_TRUE,
    "gossipping-girls/gossipping-total.prism",
    "gossipping-girls/all_omniscient_but_first_after_10_steps_TRUE.props"
);
bench!(
    gossipping_girls_total_eventually_10_steps_are_passed_TRUE,
    "gossipping-girls/gossipping-total.prism",
    "gossipping-girls/eventually_10_steps_are_passed_TRUE.props"
);
bench!(
    gossipping_girls_total_girl_one_gurantee_to_become_omniscient_before_10_steps_FALSE,
    "gossipping-girls/gossipping-total.prism",
    "gossipping-girls/girl_one_gurantee_to_become_omniscient_before_10_steps_FALSE.props"
);
bench!(
    gossipping_girls_total_guarantee_all_girls_eventually_become_omniscient_but_not_girl_one_TRUE,
    "gossipping-girls/gossipping-total.prism",
    "gossipping-girls/guarantee_all_girls_eventually_become_omniscient_but_not_girl_one_TRUE.props"
);

// Mexican standoff
bench!(
    mexican_standoff_can_2_players_guarantee_atleast_one_of_them_survives,
    "mexican-standoff/mexican-3-players-1-lives.prism",
    "mexican-standoff/can_2_players_guarantee_atleast_one_of_them_survives.props"
);
bench!(
    mexican_standoff_can_3_players_guarantee_atleast_one_of_them_survives,
    "mexican-standoff/mexican-4-players-1-lives.prism",
    "mexican-standoff/can_3_players_guarantee_atleast_one_of_them_survives.props"
);
bench!(
    mexican_standoff_can_4_players_guarantee_atleast_one_of_them_survives,
    "mexican-standoff/mexican-5-players-1-lives.prism",
    "mexican-standoff/can_4_players_guarantee_atleast_one_of_them_survives.props"
);
bench!(
    mexican_standoff_can_5_players_guarantee_atleast_one_of_them_survives,
    "mexican-standoff/mexican-6-players-1-lives.prism",
    "mexican-standoff/can_5_players_guarantee_atleast_one_of_them_survives.props"
);
bench!(
    mexican_standoff_can_6_players_guarantee_atleast_one_of_them_survives,
    "mexican-standoff/mexican-7-players-1-lives.prism",
    "mexican-standoff/can_6_players_guarantee_atleast_one_of_them_survives.props"
);
bench!(
    mexican_standoff_can_p1_guarantee_to_survive_FALSE,
    "mexican-standoff/mexican-3-players-3-lives.prism",
    "mexican-standoff/can_p1_guarantee_to_survive_FALSE.props"
);
bench!(
    mexican_standoff_can_p1_kill_p2,
    "mexican-standoff/mexican-3-players-1-lives.prism",
    "mexican-standoff/can_p1_kill_p2.props"
);
bench!(
    mexican_standoff_can_p1_suicide_FALSE,
    "mexican-standoff/mexican-3-players-3-lives.prism",
    "mexican-standoff/can_p1_suicide_FALSE.props"
);

// Rock paper scissors
bench!(
    rock_paper_scissors_p1_can_win_evantuallly_FALSE,
    "rock-paper-scissors/rock-paper-scissors.prism",
    "rock-paper-scissors/p1_can_win_evantuallly_FALSE.props"
);
bench!(
    rock_paper_scissors_p1_never_lose_FALSE,
    "rock-paper-scissors/rock-paper-scissors.prism",
    "rock-paper-scissors/p1_never_lose_FALSE.props"
);

//fixme: commented out benches are due to prism erroring out
// errors with: java.lang.NullPointerException: Cannot invoke "parser.ast.Coalition.isPlayerIndexInCoalition(int, java.util.Map)" because "<parameter2>" is null

criterion_group!(
    matching_pennies,
    matching_pennies_can_odd_win_round_eventually_FALSE,
    matching_pennies_can_they_guarantee_that_odd_always_has_larger_sum_TRUE,
    matching_pennies_can_they_win_simultaneously_FALSE
);

criterion_group!(
    gossiping_girls,
    gossipping_girls_circular_all_girls_ensure_player_one_gets_omniscient_before_10_steps_TRUE,
    gossipping_girls_circular_all_girls_ensure_that_all_girls_omnicient_before_10_steps_TRUE,
    gossipping_girls_circular_all_girls_ensure_that_only_player_one_gets_omniscient_before_10_steps_TRUE,
    gossipping_girls_circular_all_omniscient_but_first_after_10_steps_TRUE,
    gossipping_girls_circular_eventually_10_steps_are_passed_TRUE,
    gossipping_girls_circular_girl_one_gurantee_to_become_omniscient_before_10_steps_FALSE,
    gossipping_girls_circular_guarantee_all_girls_eventually_become_omniscient_but_not_girl_one_TRUE,
    gossipping_girls_total_all_girls_ensure_player_one_gets_omniscient_before_10_steps_TRUE,
    gossipping_girls_total_all_girls_ensure_that_all_girls_omnicient_before_10_steps_TRUE,
    gossipping_girls_total_all_girls_ensure_that_only_player_one_gets_omniscient_before_10_steps_TRUE,
    gossipping_girls_total_all_omniscient_but_first_after_10_steps_TRUE,
    gossipping_girls_total_eventually_10_steps_are_passed_TRUE,
    gossipping_girls_total_girl_one_gurantee_to_become_omniscient_before_10_steps_FALSE,
    gossipping_girls_total_guarantee_all_girls_eventually_become_omniscient_but_not_girl_one_TRUE,
);

criterion_group!(
    mexican_standoff,
    mexican_standoff_can_2_players_guarantee_atleast_one_of_them_survives,
    mexican_standoff_can_3_players_guarantee_atleast_one_of_them_survives,
    mexican_standoff_can_4_players_guarantee_atleast_one_of_them_survives,
    mexican_standoff_can_5_players_guarantee_atleast_one_of_them_survives,
    mexican_standoff_can_6_players_guarantee_atleast_one_of_them_survives,
    mexican_standoff_can_p1_guarantee_to_survive_FALSE,
    mexican_standoff_can_p1_kill_p2,
    mexican_standoff_can_p1_suicide_FALSE
);

criterion_group!(
    rock_paper_scissors,
    rock_paper_scissors_p1_can_win_evantuallly_FALSE,
    rock_paper_scissors_p1_never_lose_FALSE
);

criterion_main!(
    mexican_standoff,
    matching_pennies,
    gossiping_girls,
    rock_paper_scissors
);
