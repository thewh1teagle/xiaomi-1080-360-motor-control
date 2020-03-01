#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;

use dlopen::wrapper::{Container, WrapperApi};

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

extern crate strum;
#[macro_use]
extern crate strum_macros;

#[derive(EnumString, Display)]
pub enum Direction {
    #[strum(serialize = "forward")]
    Forward = 0,
    #[strum(serialize = "reverse")]
    Reverse,
}

pub struct Motor {
    api: Container<MotorApi>,
}

impl Motor {
    pub fn new(library: &str) -> Motor {
        let api: Container<MotorApi> =
            unsafe { Container::load(library) }.expect("Could not open library or load symbols");

        unsafe { api.motor_init() }
        Motor { api: api }
    }

    pub fn pan(&mut self, dir: Direction, steps: i32) {
        unsafe { self.api.motor_h_dir_set(dir as i32) }
        // unsafe { self.api.motor_h_position_get() }
        unsafe { self.api.motor_h_dist_set(steps) }
        unsafe { self.api.motor_h_move() }
    }

    pub fn tilt(&mut self, dir: Direction, steps: i32) {
        unsafe { self.api.motor_v_dir_set(dir as i32) }
        unsafe { self.api.motor_v_position_get() }
        unsafe { self.api.motor_v_dist_set(steps) }
        unsafe { self.api.motor_v_move() }
    }

    pub fn stop(&mut self) {
        unsafe { self.api.motor_h_stop() }
        unsafe { self.api.motor_v_stop() }
    }
}

fn movee(motor: &mut Motor, action: String, dir: Direction, steps: i32) {
    println!("move: {} {} {}", action, dir, steps);
    match action.as_ref() {
        "pan" => motor.pan(dir, steps),
        "tilt" => motor.tilt(dir, steps),
        _ => println!("Test"),
    }
}

#[macro_use]
extern crate rouille;

fn serve(motor: &mut Motor, port: u16) {
    rouille::start_server(format!("localhost:{}", port), move |request| {
        router!(request,
            (GET) (/) => { rouille::Response::redirect_302("/help") },
            (GET) (/help) => { rouille::Response::text("usage: /move/<action>/<dir>/<steps>") },
            (GET) (/move/{action: String}/{dir: Direction}/{steps: i32}) => {
                movee(&mut motor, action, dir, steps);
                rouille::Response::text(format!("hello, {}", action))
            },
            _ => rouille::Response::empty_404()
        )
    });
}

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("camera-controller")
        .version("1.0")
        .author("Zadkiel <hello@zadkiel.fr>")
        .about("Control camera with http.")
        .arg(
            Arg::with_name("library-path")
                .short("L")
                .long("library-path")
                .value_name("PATH")
                .help("Set path to the camera libraries")
                .default_value("./mocks/libdevice_kit.so"),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("move")
                .about("move using cli")
                .arg(
                    Arg::with_name("action")
                        .possible_values(&["pan", "tilt"])
                        .help("which motor to activate")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("direction")
                        .possible_values(&["forward", "reverse"])
                        .help("which direction to activate")
                        .index(2)
                        .required(true),
                )
                .arg(
                    Arg::with_name("steps")
                        .help("which motor to activate")
                        .index(3)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("server")
                .about("start web server")
                .arg(Arg::with_name("host").short("h").help("set target host"))
                .arg(
                    Arg::with_name("port")
                        .default_value("8888")
                        .short("p")
                        .help("set target port"),
                ),
        )
        .get_matches();

    let mut motor = Motor::new(matches.value_of("library-path").unwrap());

    match matches.subcommand_name() {
        Some("move") => movee(
            &mut motor,
            matches
                .subcommand_matches("move")
                .unwrap()
                .value_of("action")
                .unwrap()
                .to_string(),
            matches
                .subcommand_matches("move")
                .unwrap()
                .value_of("direction")
                .unwrap()
                .parse::<Direction>()
                .unwrap(),
            matches
                .subcommand_matches("move")
                .unwrap()
                .value_of("steps")
                .unwrap()
                .parse::<i32>()
                .unwrap(),
        ),
        Some("server") => serve(
            &mut motor,
            matches
                .subcommand_matches("server")
                .unwrap()
                .value_of("port")
                .unwrap()
                .parse::<u16>()
                .unwrap(),
        ),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }

    motor.stop();
}
