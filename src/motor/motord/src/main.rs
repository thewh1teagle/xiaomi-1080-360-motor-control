use std::thread;
use std::time::Duration;

extern crate dlopen;
#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct RawApi<> {
    motor_init: unsafe extern "C" fn(),
    motor_exit: unsafe extern "C" fn(),
    motor_h_dir_set: unsafe extern "C" fn(direction: i32),
    motor_h_position_get: unsafe extern "C" fn(),
    motor_h_dist_set: unsafe extern "C" fn(steps: i32),
    motor_h_move: unsafe extern "C" fn(),
    motor_h_stop: unsafe extern "C" fn(),
    motor_v_dir_set: unsafe extern "C" fn(direction: i32),
    motor_v_position_get: unsafe extern "C" fn(),
    motor_v_dist_set: unsafe extern "C" fn(steps: i32),
    motor_v_move: unsafe extern "C" fn(),
    motor_v_stop: unsafe extern "C" fn()
}

fn load_raw_api() -> Container<RawApi> {
    let mut cont: Container<RawApi> =
    unsafe { Container::load("libdevice_kit.so") }.expect("Could not open library or load symbols");
    return cont;
}

enum Direction {    
    reverse = 0,
    forward = 1
}

struct PtzApi{
    raw_api: Container<RawApi>
}
impl PtzApi {
    fn pan(&self, direction: i32, steps: i32) {
        unsafe{
            self.raw_api.motor_init();
            self.raw_api.motor_h_dir_set(direction);
            self.raw_api.motor_h_position_get();
            self.raw_api.motor_h_dist_set(steps);
            self.raw_api.motor_h_move();
            self.raw_api.motor_h_stop();
        }
    }
    fn tilt(&self, direction: i32, steps: i32) {
        unsafe {
            self.raw_api.motor_init();
            self.raw_api.motor_v_dir_set(direction);
            self.raw_api.motor_v_position_get();
            self.raw_api.motor_v_dist_set(steps);
            self.raw_api.motor_v_move();
            self.raw_api.motor_v_stop();
        }
    }
}

fn main(){
    let ptz_api = PtzApi{
        raw_api: load_raw_api()
    };
    ptz_api.pan(Direction::forward as i32, 5);
    thread::sleep(Duration::from_millis(1000));
    ptz_api.pan(Direction::reverse as i32, 5);
    thread::sleep(Duration::from_millis(1000));
    ptz_api.tilt(Direction::forward as i32, 5);
    thread::sleep(Duration::from_millis(1000));
    ptz_api.tilt(Direction::reverse as i32, 5);
    
}