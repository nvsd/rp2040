use defmt::{Format, info};
use heapless::Vec;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Format)]
pub enum Direction {
    UP,
    DOWN,
    NONE,
}

pub struct ConfidentDirectionFilter {
    direction: Direction,
    current_direction: Direction,
    confidence_threshold: u8,
    confidence: u8,
    calibrated: bool,
    calibration_values: Vec<f32, 20>,
    stationary_value: f32,
}

impl ConfidentDirectionFilter {
    fn get_direction(&mut self, accel_z: f32) -> Direction {
        if accel_z > self.stationary_value + self.stationary_value / 2.0 {
            Direction::DOWN
        } else if accel_z < self.stationary_value - self.stationary_value / 2.0 {
            Direction::UP
        } else {
            Direction::NONE
        }
    }

    fn adjust_confidence(&mut self, direction: Direction) {
        if self.current_direction == direction {
            self.confidence += 1;
        } else {
            self.confidence = 0;
            self.current_direction = direction;
        }
    }

    fn check_confidence(&mut self) {
        if self.confidence >= self.confidence_threshold {
            self.direction = self.current_direction;
            self.confidence = 0;
        }
    }

    fn calibrate(&mut self, value: f32) {
        if self.calibrated {
            return;
        }
        if self.calibration_values.len() != 20 {
            self.calibration_values.push(value).unwrap();
        } else {
            let sum = self
                .calibration_values
                .clone()
                .into_iter()
                .reduce(|a, b| {a + b})
                .unwrap();
            self.stationary_value = sum / 20.0;
            self.calibrated = true;
        }
    }

    pub fn get_confident_direction(&mut self, accel_z: f32) -> Direction {
        if !self.calibrated {
            self.calibrate(accel_z);
            return Direction::NONE;
        }

        let direction = self.get_direction(accel_z);
        self.adjust_confidence(direction);
        self.check_confidence();
        self.direction
    }

    pub fn new() -> ConfidentDirectionFilter {
        ConfidentDirectionFilter {
            current_direction: Direction::NONE,
            direction: Direction::NONE,
            confidence_threshold: 10,
            confidence: 0,
            calibrated: false,
            stationary_value: 0.0,
            calibration_values: Vec::new(),
        }
    }
}
