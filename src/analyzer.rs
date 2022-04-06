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
                Line::Save(_) => todo!(),
                Line::SharedSeed(_) => todo!(),
                Line::Buttons(_) => todo!(),
                Line::LGAGSTMinSpeed(_) => todo!(),
                Line::Reset { .. } => todo!(),
                Line::Comment(_) => todo!(),
                Line::VectorialStrafing(_) => todo!(),
                Line::VectorialStrafingConstraints(_) => todo!(),
                Line::Change(_) => todo!(),
                Line::TargetYawOverride(_) => todo!(),
            }
        }

        // calculate final time bounds

        Ok(AnalyzerResult {
            final_time_range,
            frametime_stats: frametime_stats
                .into_iter()
                .map(|(s, v)| (s.to_owned(), v))
                .collect(),
        })
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),
}

pub struct AnalyzerResult {
    pub final_time_range: Range<f64>,
    pub frametime_stats: HashMap<String, Range<u128>>,
}
