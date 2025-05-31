use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum IncomingMessage {
    Sensor(ServoSetup),
    Config(Config),
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServoSetup {
    pub position_c0: u16,
    pub position_c1: u16,
    pub position_c2: u16,
    pub position_c3: u16,
    pub position_c4: u16,
    pub position_c5: u16,
    pub position_c6: u16,
    pub position_c7: u16,
    pub position_c8: u16,
    pub position_c9: u16,
    pub position_c10: u16,
    pub position_c11: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub position_steps: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            position_steps: 10, // Or any other sensible default value
        }
    }
}
