use std::{num::NonZeroU32, ops::Range};

use hltas::types::*;

use crate::analyzer::Analyzer;

#[test]
fn final_time() {
    let hltas = HLTAS {
        properties: Default::default(),
        lines: vec![
            Line::FrameBulk(FrameBulk {
                frame_time: "0.001".to_string(),
                frame_count: NonZeroU32::new(100).unwrap(),
                auto_actions: Default::default(),
                movement_keys: Default::default(),
                action_keys: Default::default(),
                pitch: Default::default(),
                console_command: Default::default(),
            }),
            Line::FrameBulk(FrameBulk {
                frame_time: "0.010000001".to_string(),
                frame_count: NonZeroU32::new(5719).unwrap(),
                auto_actions: Default::default(),
                movement_keys: Default::default(),
                action_keys: Default::default(),
                pitch: Default::default(),
                console_command: Default::default(),
            }),
            Line::FrameBulk(FrameBulk {
                frame_time: "0.003".to_string(),
                frame_count: NonZeroU32::new(5151).unwrap(),
                auto_actions: AutoActions {
                    leave_ground_action: Some(LeaveGroundAction {
                        speed: LeaveGroundActionSpeed::Any,
                        times: Times::UnlimitedWithinFrameBulk,
                        type_: LeaveGroundActionType::DuckTap { zero_ms: true },
                    }),
                    ..Default::default()
                },
                movement_keys: Default::default(),
                action_keys: Default::default(),
                pitch: Default::default(),
                console_command: Default::default(),
            }),
        ],
    };

    let result = Analyzer::analyze_hltas(&hltas).unwrap();

    assert_eq!(
        result.final_time,
        Range {
            start: 57.290005719,
            end: 72.743005719,
        }
    )
}
