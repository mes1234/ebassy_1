use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum IncomingMessage {
    Sensor(ServoSetup),
    Config(Config),
}

// u16 but uses
#[derive(Debug, Deserialize, Clone)]
pub struct ServoSetup {
    pub position_c0: u8,
    pub position_c1: u8,
    pub position_c2: u8,
    pub position_c3: u8,
    pub position_c4: u8,
    pub position_c5: u8,
    pub position_c6: u8,
    pub position_c7: u8,
    pub position_c8: u8,
    pub position_c9: u8,
    pub position_c10: u8,
    pub position_c11: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub position_steps: u8, // Defines how many steps per setup to interpolate position
    pub c0_config: ServoConfig,
    pub c1_config: ServoConfig,
    pub c2_config: ServoConfig,
    pub c3_config: ServoConfig,
    pub c4_config: ServoConfig,
    pub c5_config: ServoConfig,
    pub c6_config: ServoConfig,
    pub c7_config: ServoConfig,
    pub c8_config: ServoConfig,
    pub c9_config: ServoConfig,
    pub c10_config: ServoConfig,
    pub c11_config: ServoConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServoConfig {
    pub lower: u8,  // lower angle (0-180)
    pub higher: u8, // higher angle (0-180)
}

impl Config {
    pub const fn default() -> Self {
        Config {
            position_steps: 10, // Or any other sensible default value
            c0_config: ServoConfig {
                lower: 0,
                higher: 180,
            },
            c1_config: ServoConfig {
                lower: 0,
                higher: 180,
            },
            c2_config: ServoConfig {
                lower: 0,
                higher: 180,
            },
            c3_config: ServoConfig {
                lower: 0,
                higher: 180,
            },
            c4_config: ServoConfig {
                lower: 0,
                higher: 180,
            },
            c5_config: ServoConfig {
                lower: 0,
                higher: 180,
            },
            c6_config: ServoConfig {
                lower: 0,
                higher: 180,
            },
            c7_config: ServoConfig {
                lower: 0,
                higher: 180,
            },
            c8_config: ServoConfig {
                lower: 0,
                higher: 180,
            },
            c9_config: ServoConfig {
                lower: 0,
                higher: 180,
            },
            c10_config: ServoConfig {
                lower: 0,
                higher: 180,
            },
            c11_config: ServoConfig {
                lower: 0,
                higher: 180,
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::default()
    }
}
