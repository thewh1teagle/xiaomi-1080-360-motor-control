use std::thread;
use std::time::Duration;
use std::os::raw::{c_char};
use std::ffi::{CString, CStr};


extern crate dlopen;
use dlopen::wrapper::{Container, WrapperApi};


enum Direction {    
    Reverse,
    Forward
}

#[derive(WrapperApi)]
struct Dlibrary<> {
    motor_init: unsafe extern "C" fn(),
    motor_exit: unsafe extern "C" fn(),
    motor_h_dir_set: unsafe extern "C" fn(direction: i32),
    motor_h_position_get: unsafe extern "C" fn() -> i32,
    motor_h_dist_set: unsafe extern "C" fn(steps: i32),
    motor_h_move: unsafe extern "C" fn(),
    motor_h_stop: unsafe extern "C" fn(),
    motor_v_dir_set: unsafe extern "C" fn(direction: i32),
    motor_v_position_get: unsafe extern "C" fn() -> i32,
    motor_v_dist_set: unsafe extern "C" fn(steps: i32),
    motor_v_move: unsafe extern "C" fn(),
    motor_v_stop: unsafe extern "C" fn()
}


fn load_raw_api(name: &str) -> Container<Dlibrary> {
    let cont: Container<Dlibrary> =
    unsafe { Container::load(name) }.expect("Could not open library or load symbols");
    return cont;
}
struct RawApi {
    dlibrary: Container<Dlibrary>
}

impl RawApi {
    pub fn new() -> Self {
        RawApi {
            dlibrary: load_raw_api("libdevice_kit.so")
        }
    }
    pub fn pan(&self, direction: Direction, steps: i32) {
        unsafe {
        self.dlibrary.motor_init();
        self.dlibrary.motor_h_dir_set(direction as i32);
        self.dlibrary.motor_h_position_get();
        self.dlibrary.motor_h_dist_set(steps);
        self.dlibrary.motor_h_move();
        self.dlibrary.motor_h_stop();
        self.dlibrary.motor_exit();
        }
    }
    pub fn tilt(&self, direction: Direction, steps: i32) {
        unsafe {
        self.dlibrary.motor_init();
        self.dlibrary.motor_v_dir_set(direction as i32);
        self.dlibrary.motor_v_position_get();
        self.dlibrary.motor_v_dist_set(steps);
        self.dlibrary.motor_v_move();
        self.dlibrary.motor_v_stop();
        self.dlibrary.motor_exit();
        }
    }
}

pub struct PtzApi {
    raw_api: RawApi,
    
    pan_angle: f32,
    pan_total_steps: f32,
    pan_min_angle: f32,
    pan_max_angle: f32,

    tilt_angle: f32,
    tilt_total_steps: f32,
    tilt_min_angle: f32,
    tilt_max_angle: f32,
}
impl PtzApi {

    pub fn new(
            pan_min_angle: f32,
            pan_max_angle: f32,
            pan_total_steps: f32,

            tilt_min_angle: f32,
            tilt_max_angle: f32,
            tilt_total_steps: f32
        ) -> Self {
        
        return PtzApi {
            raw_api: RawApi::new(),
            pan_angle: 0.0,
            pan_total_steps: pan_total_steps,
            pan_min_angle: pan_min_angle,
            pan_max_angle: pan_max_angle,

            tilt_angle: 0.0,
            tilt_total_steps: tilt_total_steps,
            tilt_min_angle: tilt_min_angle,
            tilt_max_angle: tilt_max_angle
        };
    }
    pub fn pan_abs(&mut self, angle: f32) {
        assert!(self.pan_min_angle <= angle && self.pan_max_angle >=  angle, "Angle overflow");
        let direction: Direction;
        let mut steps: f32;
        if angle > self.pan_angle {
            direction = Direction::Forward;
        } else {
            direction = Direction::Reverse;
        }
        if angle != self.pan_angle {
            let angles = (self.pan_angle - angle).abs();
            steps = angles / ( (self.pan_max_angle.abs() + self.pan_min_angle.abs()) / self.pan_total_steps );
            steps = steps.round();
            self.raw_api.pan(direction, steps as i32);
            self.pan_angle = angle;
        }
    }
    pub fn tilt_abs(&mut self, angle: f32) {
        assert!(self.tilt_min_angle <= angle && self.tilt_max_angle >=  angle, "Angle overflow");
        let direction: Direction;
        let mut steps: f32;
        if angle > self.tilt_angle {
            direction = Direction::Forward;
        } else {
            direction = Direction::Reverse;
        }
        if angle != self.tilt_angle {
            let angles = (self.tilt_angle - angle).abs();
            steps = angles / ( (self.tilt_max_angle.abs() + self.tilt_min_angle.abs()) / self.tilt_total_steps );
            steps = steps.round();
            self.raw_api.tilt(direction, steps as i32);
            self.tilt_angle = angle;
        }
    }

    pub fn pan_relative(&self, angle: i32) {

    }
    pub fn tilt_relative(&self, angle: i32) {
        
    }

    pub fn calibrate(&mut self) {
        self.raw_api.tilt(Direction::Forward, self.tilt_total_steps as i32);
        self.raw_api.tilt(Direction::Reverse, (self.tilt_total_steps / 2.0) as i32 );
        self.tilt_angle = self.tilt_max_angle / 2.0;
        
        self.raw_api.pan(Direction::Forward, self.pan_total_steps as i32);
        self.raw_api.pan(Direction::Reverse, (self.pan_total_steps / 2.0) as i32);
        self.pan_angle = self.pan_max_angle / 2.0;
    }

}