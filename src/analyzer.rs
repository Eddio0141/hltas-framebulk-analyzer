use std::{collections::HashMap, fmt::Display, ops::Range};

use hltas::{
    types::{LeaveGroundActionType, Line},
    HLTAS,
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use thiserror::Error;

use ansi_term::Colour::*;

pub fn analyze_hltas(hltas: &HLTAS) -> Result<AnalyzerResult, Error> {
    let mut final_time = Range {
        start: dec!(0.0),
        end: dec!(0.0),
    };
    let mut frametime_stats = HashMap::new();
    let mut save_count = 0;
    let mut shared_seed_set_count = 0;
    let mut button_set_count = 0;
    let mut lgagst_min_speed_set_count = 0;
    let mut reset_count = 0;
    let mut comment_count = 0;
    let mut change_angle_count = 0;
    let mut target_yaw_override_count = 0;

    for line in &hltas.lines {
        match line {
            Line::FrameBulk(fb) => {
                let zero_ms_ducktap = if let Some(action) = &fb.auto_actions.leave_ground_action {
                    if let LeaveGroundActionType::DuckTap { zero_ms } = &action.type_ {
                        *zero_ms
                    } else {
                        false
                    }
                } else {
                    false
                };

                let frame_time =
                    fb.frame_time
                        .parse::<Decimal>()
                        .map_err(|err| Error::FrametimeParseError {
                            source: err,
                            string: &fb.frame_time,
                        })?;
                let frame_count = fb.frame_count.get() as u128;

                frametime_stats
                    .entry(frame_time)
                    .and_modify(|count: &mut u128| {
                        *count += frame_count;
                    })
                    .or_insert(frame_count);

                // add final time range
                let fb_time = frame_time * Decimal::from(fb.frame_count.get());

                if !zero_ms_ducktap {
                    final_time.start += fb_time;
                }
                final_time.end += fb_time;
            }
            Line::Save(_) => save_count += 1,
            Line::SharedSeed(_) => shared_seed_set_count += 1,
            Line::Buttons(_) => button_set_count += 1,
            Line::LGAGSTMinSpeed(_) => lgagst_min_speed_set_count += 1,
            Line::Reset { .. } => reset_count += 1,
            Line::Comment(_) => comment_count += 1,
            Line::VectorialStrafing(_) => (),
            Line::VectorialStrafingConstraints(_) => (),
            Line::Change(_) => change_angle_count += 1,
            Line::TargetYawOverride(_) => target_yaw_override_count += 1,
        }
    }

    let frametime_stats = {
        let mut frametime_stats_res = Vec::new();

        for (s, v) in frametime_stats {
            frametime_stats_res.push(FrametimeStats {
                frametime: s,
                frame_count: v,
            });
        }

        frametime_stats_res.sort_by(|f, s| f.frametime.cmp(&s.frametime));

        frametime_stats_res
    };

    Ok(AnalyzerResult {
        final_time,
        frametime_stats,
        save_count,
        shared_seed_set_count,
        button_set_count,
        lgagst_min_speed_set_count,
        reset_count,
        comment_count,
        change_angle_count,
        target_yaw_override_count,
    })
}

#[derive(Debug, Error)]
pub enum Error<'a> {
    #[error("Failed to parse frametime {string} as a decimal")]
    FrametimeParseError {
        #[source]
        source: rust_decimal::Error,
        string: &'a str,
    },
}

pub struct AnalyzerResult {
    pub final_time: Range<Decimal>,
    pub frametime_stats: Vec<FrametimeStats>,
    pub save_count: u128,
    pub shared_seed_set_count: u128,
    pub button_set_count: u128,
    pub lgagst_min_speed_set_count: u128,
    pub reset_count: u128,
    pub comment_count: u128,
    pub change_angle_count: u128,
    pub target_yaw_override_count: u128,
}

impl Display for AnalyzerResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}: {}s ~ {}s",
            Red.paint("Final Time"),
            self.final_time.start,
            self.final_time.end
        )?;
        writeln!(f)?;
        writeln!(f, "{}", Green.paint("Frametime stats"))?;
        for stats in &self.frametime_stats {
            writeln!(f, "    {stats}")?;
        }
        writeln!(f)?;
        writeln!(f, "Save count: {}", self.save_count)?;
        writeln!(f, "Shared seed set count: {}", self.shared_seed_set_count)?;
        writeln!(f, "Button set count: {}", self.button_set_count)?;
        writeln!(
            f,
            "LGAGST min speed set count: {}",
            self.lgagst_min_speed_set_count
        )?;
        writeln!(f, "Reset count: {}", self.reset_count)?;
        writeln!(f, "Comment count: {}", self.comment_count)?;
        writeln!(f, "Change angle count: {}", self.change_angle_count)?;
        writeln!(
            f,
            "Target yaw override count: {}",
            self.target_yaw_override_count
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrametimeStats {
    pub frametime: Decimal,
    pub frame_count: u128,
}

impl Display for FrametimeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}ms for {} frames", self.frametime, self.frame_count)
    }
}
