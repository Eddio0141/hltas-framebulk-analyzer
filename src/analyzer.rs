//! Contains everything required to analyze a hltas file.

use std::{collections::HashMap, fmt::Display, str::FromStr};

use hltas::{
    types::{LeaveGroundActionType, Line},
    HLTAS,
};
use num_bigint::BigUint;
use rust_decimal::{
    prelude::{FromPrimitive, One, Zero},
    Decimal,
};
use rust_decimal_macros::dec;
use thiserror::Error;

use ansi_term::Colour::*;

/// Function that analyzes a HLTAS, returning a [`AnalyzerResult`][AnalyzerResult] type on success.
/// - Only can fail if the frametime can't be parsed as a [`Decimal`](rust_decimal::Decimal).
pub fn analyze_hltas(hltas: &HLTAS) -> Result<AnalyzerResult, Error> {
    let mut final_time = FinalTime {
        start: Decimal::ZERO,
        end: Decimal::ZERO,
    };
    let mut estimated_time = Decimal::ZERO;
    let mut frametime_stats = HashMap::new();
    let mut save_count = BigUint::zero();
    let mut shared_seed_set_count = BigUint::zero();
    let mut button_set_count = BigUint::zero();
    let mut lgagst_min_speed_set_count = BigUint::zero();
    let mut reset_count = BigUint::zero();
    let mut comment_count = BigUint::zero();
    let mut change_angle_count = BigUint::zero();
    let mut target_yaw_override_count = BigUint::zero();

    // used for tracking the 0ms frame estimation
    let mut zero_ms_counter = Decimal::ZERO;

    let zero_ms_frametime = match &hltas.properties.frametime_0ms {
        Some(zero_ms) => {
            Decimal::from_str(zero_ms).map_err(|err| Error::ZeroMsFrametimeParseError {
                source: err,
                string: zero_ms,
            })?
        }
        None => Decimal::new(1, 10),
    };

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
                // shouldn't be a negative value
                let frame_count = BigUint::from_u32(fb.frame_count.get()).unwrap();

                frametime_stats
                    .entry(frame_time)
                    .and_modify(|count: &mut BigUint| {
                        *count += &frame_count;
                    })
                    .or_insert(frame_count);

                // add final time range
                let fb_time = frame_time * Decimal::from(fb.frame_count.get());

                if !zero_ms_ducktap {
                    final_time.start += fb_time;
                }
                final_time.end += fb_time;

                // add estimated time
                estimated_time += if zero_ms_ducktap {
                    // simulate flat ground 0ms ducktap
                    // 0.201s to reach the ground, then the next frame becomes 0ms
                    let mut fb_time_with_zero_ms = Decimal::ZERO;

                    for _ in 0..fb.frame_count.get() {
                        zero_ms_counter += frame_time;

                        if zero_ms_counter > dec!(0.201) {
                            zero_ms_counter = Decimal::ZERO;
                            fb_time_with_zero_ms += zero_ms_frametime;
                        } else {
                            fb_time_with_zero_ms += frame_time;
                        }
                    }

                    fb_time_with_zero_ms
                } else {
                    fb_time
                };
            }
            Line::Save(_) => save_count += BigUint::one(),
            Line::SharedSeed(_) => shared_seed_set_count += BigUint::one(),
            Line::Buttons(_) => button_set_count += BigUint::one(),
            Line::LGAGSTMinSpeed(_) => lgagst_min_speed_set_count += BigUint::one(),
            Line::Reset { .. } => reset_count += BigUint::one(),
            Line::Comment(_) => comment_count += BigUint::one(),
            Line::VectorialStrafing(_) => (),
            Line::VectorialStrafingConstraints(_) => (),
            Line::Change(_) => change_angle_count += BigUint::one(),
            Line::TargetYawOverride(_) => target_yaw_override_count += BigUint::one(),
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
        estimated_time,
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

/// Error type for the [`analyze_hltas`][analyze_hltas] function.
#[derive(Debug, Error)]
pub enum Error<'a> {
    /// Error when parsing a frametime string.
    /// Happens if the frametime can't be parse as a [`Decimal`](rust_decimal::Decimal).
    #[error("Failed to parse frametime {string} as a decimal")]
    FrametimeParseError {
        #[source]
        source: rust_decimal::Error,
        string: &'a str,
    },
    /// Error when parsing a 0ms frametime property from a string.
    /// Happens if the frametime can't be parse as a [`Decimal`](rust_decimal::Decimal).
    #[error("Failed to parse 0ms frametime {string} as a decimal")]
    ZeroMsFrametimeParseError {
        #[source]
        source: rust_decimal::Error,
        string: &'a str,
    },
}

/// Analysis result of a HLTAS.
pub struct AnalyzerResult {
    /// The final time of the HLTAS.
    /// - `start` will be the shortest possible time of the hltas, assuming all 0ms ducktap framebulks are 0ms.
    /// - `end` will be the longest possible time of the hltas, assuming all 0ms ducktap framebulks aren't 0ms.
    pub final_time: FinalTime,
    /// The estimated time of the HLTAS.
    /// Assumes the 0ms ducktap framebulks are landing on a flat ground, with normal gravity.
    pub estimated_time: Decimal,
    /// The frametime stats of the HLTAS, containing `frametime` and total `frame_count`.
    pub frametime_stats: Vec<FrametimeStats>,
    /// The number of `save` special frames in the HLTAS.
    pub save_count: BigUint,
    /// The number of `shared_seed` sets in the HLTAS.
    pub shared_seed_set_count: BigUint,
    /// The number of strafing `button` mapping in the HLTAS, including resetting.
    pub button_set_count: BigUint,
    /// The number of `lgagst_min_speed` sets in the HLTAS.
    pub lgagst_min_speed_set_count: BigUint,
    /// The number of `reset` done in the HLTAS.
    pub reset_count: BigUint,
    /// The number of `comment` in the HLTAS.
    pub comment_count: BigUint,
    /// The number of `change` in the HLTAS.
    pub change_angle_count: BigUint,
    /// The number of `target_yaw_override` in the HLTAS.
    pub target_yaw_override_count: BigUint,
}

impl Display for AnalyzerResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let minutes = |seconds: &Decimal| (seconds / dec!(60.0)).floor();
        let sub_seconds = |seconds: &Decimal| (seconds % dec!(60.0)).round_dp(3);

        let final_time_minutes = minutes(&self.final_time.start)..minutes(&self.final_time.end);
        let final_time_sub_seconds =
            sub_seconds(&self.final_time.start)..sub_seconds(&self.final_time.end);

        let estimated_time_minutes = minutes(&self.estimated_time);
        let estimated_time_sub_seconds = sub_seconds(&self.estimated_time);

        let final_time_string = {
            let start = if final_time_minutes.start.is_zero() {
                format!("{}s", final_time_sub_seconds.start)
            } else {
                format!(
                    "{}m {}s",
                    final_time_minutes.start, final_time_sub_seconds.start
                )
            };

            let end = if final_time_minutes.end.is_zero() {
                format!("{}s", final_time_sub_seconds.end)
            } else {
                format!(
                    "{}m {}s",
                    final_time_minutes.end, final_time_sub_seconds.end
                )
            };

            (start, end)
        };

        let estimated_time_string = if estimated_time_minutes.is_zero() {
            format!("{}s", estimated_time_sub_seconds)
        } else {
            format!(
                "{}m {}s",
                estimated_time_minutes, estimated_time_sub_seconds
            )
        };

        writeln!(
            f,
            "{}: {} ~ {}",
            Red.paint("Final time"),
            final_time_string.0,
            final_time_string.1
        )?;
        writeln!(
            f,
            "{}: {}",
            Blue.paint("Estimated time"),
            estimated_time_string
        )?;

        writeln!(f)?;

        writeln!(
            f,
            "{}: {}s ~ {}s",
            RGB(0xFF, 0x5F, 0x1F).paint("Final time secs"),
            self.final_time.start,
            self.final_time.end
        )?;
        writeln!(
            f,
            "{}: {}s",
            RGB(0x29, 0xB6, 0xF6).paint("Estimated secs"),
            self.estimated_time
        )?;
        writeln!(f)?;
        writeln!(f, "{}", Green.paint("Frametime stats"))?;
        for stats in &self.frametime_stats {
            writeln!(f, "    {stats}")?;
        }
        writeln!(f)?;
        writeln!(f, "{}: {}", Fixed(93).paint("Save count"), self.save_count)?;
        writeln!(
            f,
            "{}: {}",
            Fixed(99).paint("Shared seed set count"),
            self.shared_seed_set_count
        )?;
        writeln!(
            f,
            "{}: {}",
            Fixed(105).paint("Button set count"),
            self.button_set_count
        )?;
        writeln!(
            f,
            "{}: {}",
            Fixed(111).paint("LGAGST min speed set count"),
            self.lgagst_min_speed_set_count
        )?;
        writeln!(
            f,
            "{}: {}",
            Fixed(117).paint("Reset count"),
            self.reset_count
        )?;
        writeln!(
            f,
            "{}: {}",
            Fixed(123).paint("Comment count"),
            self.comment_count
        )?;
        writeln!(
            f,
            "{}: {}",
            Fixed(129).paint("Change angle count"),
            self.change_angle_count
        )?;
        writeln!(
            f,
            "{}: {}",
            Fixed(135).paint("Target yaw override count"),
            self.target_yaw_override_count
        )
    }
}

/// The frametime stats of a HLTAS.
/// Contains `frametime` and total `frame_count`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrametimeStats {
    pub frametime: Decimal,
    pub frame_count: BigUint,
}

impl Display for FrametimeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}ms for {} frames", self.frametime, self.frame_count)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FinalTime {
    pub start: Decimal,
    pub end: Decimal,
}
