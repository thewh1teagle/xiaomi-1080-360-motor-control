#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
use dlopen::wrapper::{Container, WrapperApi};

// TODO: Ensure we stay in the field
const _MAX_LEFT: u8 = 66;
const _MAX_RIGHT: u8 = 66;
const _MAX_DOWN: u8 = 13;
const _MAX_UP: u8 = 20;


#[derive(WrapperApi)]
struct PTZApi {
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

#[derive(EnumString, EnumVariantNames, PartialEq, Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum Action {
    Pan,
    Tilt,
}
#[derive(EnumString, EnumVariantNames, PartialEq, Display, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum Direction {
    Forward,
    Reverse,
}
type StepCount = u8;

pub struct PTZService {
    api: Container<PTZApi>,
}

impl PTZService {
    pub fn new(library: String) -> PTZService {
        let api: Container<PTZApi> =
            unsafe { Container::load(&library) }
            .unwrap_or_else(|_| panic!("cloud not load '{}' library", library));
            PTZService { api: api }
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

#[macro_use]
extern crate rouille;
use std::net::{ToSocketAddrs, SocketAddr};

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("control")
        .version("1.0")
        .arg(
            Arg::with_name("library-path")
                .short("L")
                .long("library-path")
                .value_name("PATH")
                .help("Set path to the camera libraries")
                .env("MIJIA_LIB_PATH")
                .default_value("./mocks/libdevice_kit.so")
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            App::new("ptz")
                .about("ptz service")
                .subcommand(
                    App::new("move")
                        .arg(
                            Arg::with_name("action")
                                .possible_values(&Action::VARIANTS)
                                .index(1)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("direction")
                                .possible_values(&Direction::VARIANTS)
                                .index(2)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("steps")
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
                    Arg::with_name("listen")
                        .default_value("127.0.0.1:8888")
                        .short("p")
                        .help("set target port"),
                ),
        )
        .get_matches();

    let mut ptz = PTZService::new(matches.value_of("library-path").unwrap().to_string());

    match matches.subcommand() {
        ("ptz", Some(matches)) => match matches.subcommand() {
            ("stop", Some(_)) => {
                ptz.stop();
            }
            ("move", Some(matches)) => {
                ptz.move_(
                    Action::from_str(matches.value_of("action").unwrap()).unwrap(),
                    Direction::from_str(matches.value_of("direction").unwrap()).unwrap(),
                    matches
                        .value_of("steps")
                        .unwrap()
                        .parse::<StepCount>()
                        .unwrap(),
                );
            }
            _ => println!("{}", matches.usage()),
        },
        ("server", Some(matches)) => {
            let addrs: Vec<_> = matches.value_of("listen").unwrap()
                .to_socket_addrs()
                .expect("Unable to parse socket address")
                .collect();
            let addr : SocketAddr = *addrs.first().unwrap();

            println!("listening on {}", addr);

            rouille::start_server(addr, move |request| {
                router!(request,
                    (GET) (/ptz/move/{action: Action}/{dir: Direction}/{steps: StepCount}) => {
                        // TODO: Reimplement
                        // motor.move_(action, dir, steps);
                        println!("called /ptz/move/{}/{}/{}", action, dir, steps);
                        rouille::Response::text(format!("{{}}"))
                    },
                    _ => rouille::Response::empty_404()
                )
            });
        }
        _ => println!("{}", matches.usage()),
    }
}
