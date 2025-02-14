use crate::crew::Column;
use crate::prelude::*;
use becks_match::*;

fn estimated_var(login: &Login, crew: Id, other: Id, round_worth: u32) -> f32 {
    // TODO: Better estimation
    round_worth as f32
}

fn calc_elo(lhs: Score, rhs: Score) -> f32 {
    (1.0 + 10.0f32.powf((rhs.0 - lhs.0) as f32 / 400.0)).recip()
}

fn update_crew_option(
    login: &Login,
    lhs_crew: Id,
    rhs_crew: Id,
    lhs_wins: i32,
    rhs_wins: i32,
    round_worth: u32,
) -> Option<(i32, i32)> {
    let lhs = Score::query(login, lhs_crew, true)?;
    let rhs = Score::query(login, rhs_crew, true)?;
    debug!("Starting score: {:?} and {:?}", lhs, rhs);
    debug!(
        "Left wins {} times; Right wins {} times",
        lhs_wins, rhs_wins
    );
    let total_round = lhs_wins + rhs_wins;
    // For lhs:
    let lhs_elo = calc_elo(lhs, rhs);
    debug!("Left elo is {}", lhs_elo);
    let lhs_diff = estimated_var(login, lhs_crew, rhs_crew, round_worth)
        * (lhs_wins as f32 / total_round as f32 - lhs_elo)
        * becks_db::CONFIG.user.elo_scaler;
    let lhs_diff = lhs_diff.round() as i32;
    debug!("Left diff is {}", lhs_diff);
    if !Score::modify(Score(lhs.0 + lhs_diff), login, lhs_crew) {
        return None;
    }
    // For rhs:
    let rhs_elo = calc_elo(rhs, lhs);
    debug!("Right elo is {}", rhs_elo);
    let rhs_diff = estimated_var(login, rhs_crew, lhs_crew, round_worth)
        * (rhs_wins as f32 / total_round as f32 - rhs_elo)
        * becks_db::CONFIG.user.elo_scaler;
    let rhs_diff = rhs_diff.round() as i32;
    debug!("Right diff is {}", rhs_diff);
    if !Score::modify(Score(rhs.0 + rhs_diff), login, rhs_crew) {
        return None;
    }
    Some((lhs_diff, rhs_diff))
}

fn update_beat(login: &Login, main: Id, beaten: Id, beaten_score: Score) -> Option<()> {
    debug!(
        "{:?} beats {:?} with score {:?}, updating",
        main, beaten, beaten_score
    );
    let mut beat = Beat::query(login, main, false).unwrap_or_default();
    beat.0.push(BeatItem {
        oppo: String::query(login, beaten, true)?,
        score: beaten_score,
    });
    let mut index = beat.0.len() - 1;
    while index > 0 && beat.0[index - 1].score.0 < beat.0[index].score.0 {
        beat.0.swap(index, index - 1);
        index -= 1;
    }
    if beat.0.len() > becks_db::CONFIG.db.beat_limit {
        beat.0.pop();
    }
    beat.modify(login, main);
    Some(())
}

/// Updates crew score accordingly, returning left earn and right earn scores if successful
pub fn update_crew(login: &Login, mat: &Match) -> Option<(i32, i32)> {
    let lhs_wins = mat
        .rounds
        .iter()
        .fold(0, |sum, round| if round.left_win { sum + 1 } else { sum });
    let rhs_wins = mat.total_rounds as i32 - lhs_wins;
    if lhs_wins > rhs_wins {
        update_beat(
            login,
            mat.left,
            mat.right,
            Score::query(login, mat.right, true)?,
        )?;
    } else {
        update_beat(
            login,
            mat.right,
            mat.left,
            Score::query(login, mat.left, true)?,
        )?;
    }
    update_crew_option(
        login,
        mat.left,
        mat.right,
        lhs_wins,
        rhs_wins,
        mat.round_worth,
    )
}
