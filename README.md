# Tool that analysis a hltas file or framebulks

## Example
```rust
let hltas = HLTAS {
    properties: Properties::default(),
    lines: vec![
        Line::FrameBulk(FrameBulk {
            frame_time: "0.001".to_string(),
            frame_count: NonZeroU32::new(150).unwrap(),
            auto_actions: Default::default(),
            movement_keys: Default::default(),
            action_keys: Default::default(),
            pitch: Default::default(),
            console_command: Default::default(),
       }),
       Line::FrameBulk(FrameBulk {
           frame_time: "0.004".to_string(),
           frame_count: NonZeroU32::new(150).unwrap(),
           auto_actions: Default::default(),
           movement_keys: Default::default(),
           action_keys: Default::default(),
           pitch: Default::default(),
           console_command: Default::default(),
       }),
    ],
};

let analysis = analyze_hltas(&hltas).unwrap();

// prints in a nice format
println!("{}", analysis);

assert_eq!(analysis.estimated_time, dec!(0.75));
assert_eq!(analysis.frametime_stats, vec![
    FrametimeStats {
       frametime: dec!(0.001),
       frame_count: BigUint::from_u32(150).unwrap(),
    },
    FrametimeStats {
       frametime: dec!(0.004),
       frame_count: BigUint::from_u32(150).unwrap(),
    },
]);
```

## Q & A
- Why is this colourful?
- It looks cool thats why

- Why so many stats are printed on the console? It seems pointless
- Idk more the better I guess

- Why not analyze the bxt logs directly instead if hltas only gives an estimated time?
- That's why I'm making an analyzer that will take bxt logs, and maybe hltas together to made a better analysis of the TAS