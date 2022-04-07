use std::{num::NonZeroU32, ops::Range};

use hltas::types::*;
use rust_decimal_macros::dec;

use crate::analyzer::{analyze_hltas, FrametimeStats};

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

    let result = analyze_hltas(&hltas).unwrap();

    assert_eq!(
        result.final_time,
        Range {
            start: dec!(57.290005719),
            end: dec!(72.743005719),
        }
    );
}

#[test]
fn frametime_stats() {
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

                auto_actions: Default::default(),
                movement_keys: Default::default(),
                action_keys: Default::default(),
                pitch: Default::default(),
                console_command: Default::default(),
            }),
        ],
    };

    let result = analyze_hltas(&hltas).unwrap();

    assert_eq!(
        result.frametime_stats,
        vec![
            FrametimeStats {
                frametime: dec!(0.001),
                frame_count: 100,
            },
            FrametimeStats {
                frametime: dec!(0.003),
                frame_count: 5151,
            },
            FrametimeStats {
                frametime: dec!(0.010000001),
                frame_count: 5719,
            },
        ]
    );
}

#[test]
fn save_count() {
    let hltas = HLTAS {
        properties: Default::default(),
        lines: vec![
            Line::Save("buffer".to_string()),
            Line::FrameBulk(FrameBulk {
                frame_time: "0.001".to_string(),
                frame_count: NonZeroU32::new(100).unwrap(),
                auto_actions: Default::default(),
                movement_keys: Default::default(),
                action_keys: Default::default(),
                pitch: Default::default(),
                console_command: Default::default(),
            }),
            Line::Save("buffer2".to_string()),
            Line::Save("buffer3".to_string()),
        ],
    };

    let result = analyze_hltas(&hltas).unwrap();

    assert_eq!(result.save_count, 3);
}
