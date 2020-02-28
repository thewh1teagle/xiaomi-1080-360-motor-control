#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;

use dlopen::wrapper::{Container, WrapperApi};

use num_enum::TryFromPrimitive;
use std::convert::TryInto;


#[derive(WrapperApi)]
struct MotorApi {
    motor_init: unsafe extern "C" fn(),
    motor_h_dir_set: unsafe extern "C" fn(dir: i32),
    motor_v_dir_set: unsafe extern "C" fn(dir: i32),
    motor_h_position_get: unsafe extern "C" fn(),
    motor_v_position_get: unsafe extern "C" fn(),
    motor_h_dist_set: unsafe extern "C" fn(steps: i32),
    motor_v_dist_set: unsafe extern "C" fn(steps: i32),
    motor_h_move: unsafe extern "C" fn(),
    motor_v_move: unsafe extern "C" fn(),
    motor_h_stop: unsafe extern "C" fn(),
    motor_v_stop: unsafe extern "C" fn(),
    motor_exit: unsafe extern "C" fn(),
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Direction {
    Forward,
    Reverse,
}

pub struct Motor {
    api: Container<MotorApi>,
}

impl Motor {
    pub fn new(library: &str) -> Motor {
        let api: Container<MotorApi> = unsafe { Container::load(library) }.expect("Could not open library or load symbols");

        unsafe{api.motor_init()}
        Motor {
            api: api,
        }
    }

    pub fn pan(&mut self, dir: Direction, steps: i32) {
        match dir {
            Direction::Forward => unsafe{self.api.motor_h_dir_set(0)},
            Direction::Reverse => unsafe{self.api.motor_h_dir_set(1)},
        }
        unsafe{self.api.motor_h_position_get()}
        unsafe{self.api.motor_h_dist_set(steps)}
        unsafe{self.api.motor_h_move()}
    }

    pub fn tilt(&mut self, dir: Direction, steps: i32) {
        match dir {
            Direction::Forward => unsafe{self.api.motor_v_dir_set(0)},
            Direction::Reverse => unsafe{self.api.motor_v_dir_set(1)},
        }
        unsafe{self.api.motor_v_position_get()}
        unsafe{self.api.motor_v_dist_set(steps)}
        unsafe{self.api.motor_v_move()}
    }

    pub fn stop(&mut self) {
        unsafe{self.api.motor_h_stop()}
        unsafe{self.api.motor_v_stop()}
    }

}

use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let steps = args[4].parse().unwrap();
    let dir: u8 = args[3].parse().unwrap();
    let direction: Direction = dir.try_into().unwrap();

    let mut motor = Motor::new(&args[1]);
    match args[2].as_str() { 
        "pan" => motor.pan(direction, steps),
        "tilt" => motor.tilt(direction, steps),
        _ => println!("usage: "),
    }
    motor.stop();
}
