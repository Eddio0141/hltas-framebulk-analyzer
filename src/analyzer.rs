use std::{collections::HashMap, num::ParseFloatError, ops::Range};

use hltas::{
    types::{LeaveGroundActionType, Line},
    HLTAS,
};
use thiserror::Error;

pub struct Analyzer;

impl Analyzer {
    pub fn analyze_hltas(hltas: &HLTAS) -> Result<AnalyzerResult, Error> {
        let mut final_time_range = Range {
            start: 0.0,
            end: 0.0,
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
                    let zero_ms_ducktap = if let Some(action) = &fb.auto_actions.leave_ground_action
                    {
                        if let LeaveGroundActionType::DuckTap { zero_ms } = &action.type_ {
                            *zero_ms
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    let frame_count_range = {
                        let frame_count = fb.frame_count.get() as u128;
                        if zero_ms_ducktap {
                            Range {
                                start: 0,
                                end: frame_count,
                            }
                        } else {
                            Range {
                                start: frame_count,
                                end: frame_count,
                            }
                        }
                    };

                    frametime_stats
                        .entry(&fb.frame_time)
                        .and_modify(|frame_count: &mut Range<u128>| {
                            frame_count.start += frame_count_range.start;
                            frame_count.end += frame_count_range.end;
                        })
                        .or_insert(frame_count_range);

                    // add final time range
                    let frame_time = fb.frame_time.parse::<f32>()? as f64;
                    let fb_time = frame_time * fb.frame_count.get() as f64;

                    if !zero_ms_ducktap {
                        final_time_range.start += fb_time;
                    }
                    final_time_range.end += fb_time;
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

        Ok(AnalyzerResult {
            final_time: final_time_range,
            frametime_stats: frametime_stats
                .into_iter()
                .map(|(s, v)| (s.to_owned(), v))
                .collect(),
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
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),
}

pub struct AnalyzerResult {
    pub final_time: Range<f64>,
    pub frametime_stats: HashMap<String, Range<u128>>,
    pub save_count: u128,
    pub shared_seed_set_count: u128,
    pub button_set_count: u128,
    pub lgagst_min_speed_set_count: u128,
    pub reset_count: u128,
    pub comment_count: u128,
    pub change_angle_count: u128,
    pub target_yaw_override_count: u128,
}
