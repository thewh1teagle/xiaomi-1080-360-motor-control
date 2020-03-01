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
use std::str::FromStr;
use strum::VariantNames;

#[derive(EnumString, EnumVariantNames, PartialEq, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum Action {
    Pan,
    Tilt,
}
#[derive(EnumString, EnumVariantNames, PartialEq, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum Direction {
    Forward,
    Reverse,
}
type StepCount = u8;

pub struct Motor {
    api: Container<MotorApi>,
}

impl Motor {
    pub fn new(library: &str) -> Motor {
        let api: Container<MotorApi> =
            unsafe { Container::load(library) }.expect("Could not open library or load symbols");
        Motor { api: api }
    }

    pub fn init(&mut self) {
        unsafe { self.api.motor_init() }
    }

    pub fn pan(&mut self, dir: Direction, steps: StepCount) {
        unsafe { self.api.motor_h_dir_set(dir as i32) }
        // unsafe { self.api.motor_h_position_get() }
        unsafe { self.api.motor_h_dist_set(steps as i32) }
        unsafe { self.api.motor_h_move() }
    }

    pub fn tilt(&mut self, dir: Direction, steps: StepCount) {
        unsafe { self.api.motor_v_dir_set(dir as i32) }
        // unsafe { self.api.motor_v_position_get() }
        unsafe { self.api.motor_v_dist_set(steps as i32) }
        unsafe { self.api.motor_v_move() }
    }

    pub fn move_(&mut self, action: Action, dir: Direction, steps: StepCount) {
        self.init();
        match action {
            Action::Pan => self.pan(dir, steps),
            Action::Tilt => self.tilt(dir, steps),
        }
        self.stop();
    }

    pub fn stop(&mut self) {
        unsafe { self.api.motor_h_stop() }
        unsafe { self.api.motor_v_stop() }
    }
}

// #[macro_use]
// extern crate rouille;

// fn serve(motor: &mut Motor, port: u16) {
//     rouille::start_server(format!("localhost:{}", port), move |request| {
//         router!(request,
//             (GET) (/) => { rouille::Response::redirect_302("/help") },
//             (GET) (/help) => { rouille::Response::text("usage: /move/<action>/<dir>/<steps>") },
//             (GET) (/move/{action: String}/{dir: Direction}/{steps: i32}) => {
//                 movee(&mut motor, action, dir, steps);
//                 rouille::Response::text(format!("hello, {}", action))
//             },
//             _ => rouille::Response::empty_404()
//         )
//     });
// }

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
            App::new("motor")
                .about("motor controllers")
                .subcommand(
                    App::new("move")
                        .about("move using cli")
                        .arg(
                            Arg::with_name("action")
                                .possible_values(&Action::VARIANTS)
                                .help("which motor to activate")
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("direction")
                                .possible_values(&Direction::VARIANTS)
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
                .subcommand(SubCommand::with_name("stop").about("stop the move")),
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

    match matches.subcommand() {
        ("motor", Some(matches)) => match matches.subcommand() {
            ("stop", Some(_)) => {
                motor.stop();
            }
            ("move", Some(matches)) => {
                motor.init();
                motor.move_(
                    Action::from_str(matches.value_of("action").unwrap()).unwrap(),
                    Direction::from_str(matches.value_of("direction").unwrap()).unwrap(),
                    matches
                        .value_of("steps")
                        .unwrap()
                        .parse::<StepCount>()
                        .unwrap(),
                );
                // motor.stop();
            }
            _ => unreachable!(),
        },
        ("server", Some(_matches)) => {
            println!("Not implemented.")
            //     motor.stop(
            //     &mut motor,
            //     matches
            //         .subcommand_matches("server")
            //         .unwrap()
            //         .value_of("port")
            //         .unwrap()
            //         .parse::<u16>()
            //         .unwrap(),
            // )
        }
        _ => unreachable!(),
    }
}
