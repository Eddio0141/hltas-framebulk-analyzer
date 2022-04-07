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

#[test]
fn shared_seed_set_count() {
    let hltas = HLTAS {
        properties: Default::default(),
        lines: vec![
            Line::SharedSeed(0),
            Line::SharedSeed(1),
            Line::FrameBulk(FrameBulk {
                frame_time: "0.001".to_string(),
                frame_count: NonZeroU32::new(100).unwrap(),
                auto_actions: Default::default(),
                movement_keys: Default::default(),
                action_keys: Default::default(),
                pitch: Default::default(),
                console_command: Default::default(),
            }),
            Line::SharedSeed(2),
        ],
    };

    let result = analyze_hltas(&hltas).unwrap();

    assert_eq!(result.shared_seed_set_count, 3);
}

#[test]
fn button_set_count() {
    let hltas = HLTAS {
        properties: Default::default(),
        lines: vec![
            Line::Buttons(Buttons::Reset),
            Line::FrameBulk(FrameBulk {
                frame_time: "0.001".to_string(),
                frame_count: NonZeroU32::new(100).unwrap(),
                auto_actions: Default::default(),
                movement_keys: Default::default(),
                action_keys: Default::default(),
                pitch: Default::default(),
                console_command: Default::default(),
            }),
            Line::Buttons(Buttons::Set {
                air_left: Button::Back,
                air_right: Button::Back,
                ground_left: Button::Back,
                ground_right: Button::Back,
            }),
        ],
    };

    let result = analyze_hltas(&hltas).unwrap();

    assert_eq!(result.button_set_count, 2);
}

#[test]
fn lgagst_min_speed_set_count() {
    let hltas = HLTAS {
        properties: Default::default(),
        lines: vec![
            Line::LGAGSTMinSpeed(0.0),
            Line::LGAGSTMinSpeed(0.0),
            Line::FrameBulk(FrameBulk {
                frame_time: "0.001".to_string(),
                frame_count: NonZeroU32::new(100).unwrap(),
                auto_actions: Default::default(),
                movement_keys: Default::default(),
                action_keys: Default::default(),
                pitch: Default::default(),
                console_command: Default::default(),
            }),
            Line::LGAGSTMinSpeed(0.0),
        ],
    };

    let result = analyze_hltas(&hltas).unwrap();

    assert_eq!(result.lgagst_min_speed_set_count, 3);
}

#[test]
fn reset_count() {
    let hltas = HLTAS {
        properties: Default::default(),
        lines: vec![
            Line::Reset { non_shared_seed: 0 },
            Line::FrameBulk(FrameBulk {
                frame_time: "0.001".to_string(),
                frame_count: NonZeroU32::new(100).unwrap(),
                auto_actions: Default::default(),
                movement_keys: Default::default(),
                action_keys: Default::default(),
                pitch: Default::default(),
                console_command: Default::default(),
            }),
            Line::Reset { non_shared_seed: 1 },
        ],
    };

    let result = analyze_hltas(&hltas).unwrap();

    assert_eq!(result.reset_count, 2);
}

#[test]
fn comment_count() {
    let hltas = HLTAS {
        properties: Default::default(),
        lines: vec![
            Line::Comment("comment".to_string()),
            Line::FrameBulk(FrameBulk {
                frame_time: "0.001".to_string(),
                frame_count: NonZeroU32::new(100).unwrap(),
                auto_actions: Default::default(),
                movement_keys: Default::default(),
                action_keys: Default::default(),
                pitch: Default::default(),
                console_command: Default::default(),
            }),
            Line::Comment("comment2".to_string()),
        ],
    };

    let result = analyze_hltas(&hltas).unwrap();

    assert_eq!(result.comment_count, 2);
}

#[test]
fn change_angle_count() {
    let hltas = HLTAS {
        properties: Default::default(),
        lines: vec![
            Line::Change(Change {
                target: ChangeTarget::Pitch,
                final_value: 50.0,
                over: 0.4,
            }),
            Line::FrameBulk(FrameBulk {
                frame_time: "0.001".to_string(),
                frame_count: NonZeroU32::new(100).unwrap(),
                auto_actions: Default::default(),
                movement_keys: Default::default(),
                action_keys: Default::default(),
                pitch: Default::default(),
                console_command: Default::default(),
            }),
            Line::Change(Change {
                target: ChangeTarget::Pitch,
                final_value: 50.0,
                over: 0.4,
            }),
        ],
    };

    let result = analyze_hltas(&hltas).unwrap();

    assert_eq!(result.change_angle_count, 2);
}

#[test]
fn target_yaw_override_count() {
    let hltas = HLTAS {
        properties: Default::default(),
        lines: vec![
            Line::TargetYawOverride(vec![0.1, 0.2, 0.3]),
            Line::FrameBulk(FrameBulk {
                frame_time: "0.001".to_string(),
                frame_count: NonZeroU32::new(100).unwrap(),
                auto_actions: Default::default(),
                movement_keys: Default::default(),
                action_keys: Default::default(),
                pitch: Default::default(),
                console_command: Default::default(),
            }),
            Line::TargetYawOverride(vec![0.1, 0.2, 0.3]),
        ],
    };

    let result = analyze_hltas(&hltas).unwrap();

    assert_eq!(result.target_yaw_override_count, 2);
}
