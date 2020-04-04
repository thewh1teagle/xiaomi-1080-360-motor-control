
const NAME: &str = "control";
const DEFAULT_LIBRARY_PATH: &str = "./mocks";
const DEFAULT_DATABASE_PATH: &str = "./control.db";
const DEFAULT_HOST_PORT: &str = "0.0.0.0:8888";

const _MAX_X: u8 = 172;
const _MAX_Y: u8 = 40;
const _CENTER_X: u8 = 86;
const _CENTER_Y: u8 = 20;


#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
use dlopen::wrapper::{Container, WrapperApi};

use std::fs;
use std::path;



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

#[derive(EnumString, EnumVariantNames, PartialEq, Display, Debug, Copy, Clone)]
#[strum(serialize_all = "kebab_case")]
pub enum Action {
    Pan,
    Tilt,
}
#[derive(EnumString, EnumVariantNames, PartialEq, Display, Debug, Copy, Clone)]
#[strum(serialize_all = "kebab_case")]
pub enum Direction {
    Forward,
    Reverse,
}

type StepCount = u8;


extern crate unqlite;
use unqlite::{UnQLite, KV};


pub struct PTZService {
    api: Container<PTZApi>,
    store: UnQLite,
}



impl PTZService {
    pub fn new(libraries_path: path::PathBuf, database_path: path::PathBuf) -> PTZService {
        let mut device_kit_path = libraries_path.clone();
        device_kit_path.push("libdevice_kit.so");
        let device_kit_path_str = device_kit_path.into_os_string().into_string().unwrap();
        
        let api: Container<PTZApi> = unsafe { Container::load(&device_kit_path_str) }
            .unwrap_or_else(|_| panic!("cloud not load '{}' library", device_kit_path_str));

        let database_path_str = database_path.into_os_string().into_string().unwrap();
        let store = UnQLite::create(database_path_str);
        if !store.kv_contains("initialized") {
            store.kv_store("initialized", [1]).unwrap();
            store.kv_store("current_x", [0]).unwrap();
            store.kv_store("current_y", [0]).unwrap();
        }

        PTZService { api: api, store: store }
    }

    pub fn init(&mut self) {
        unsafe { self.api.motor_init() }
    }

    pub fn calibrate(&mut self) {
        self.store.kv_store("current_x", [_MAX_X]).unwrap();
        self.store.kv_store("current_y", [_MAX_Y]).unwrap();

        self.right(_MAX_X);
        self.store.kv_store("current_x", [0]).unwrap();
        self.left(_CENTER_X);

        self.down(_MAX_Y);
        self.store.kv_store("current_y", [0]).unwrap();
        self.up(_CENTER_Y);
    }

    pub fn move_(&mut self, action: Action, dir: Direction, steps: StepCount) {
        println!("ptz move action={} dir={} steps={}", action, dir, steps);

        self.init();
        match action {
            Action::Pan => {
                let current_x = self.store.kv_fetch("current_x").unwrap()[0];
                unsafe { self.api.motor_h_dir_set(dir as i32) }
                unsafe { self.api.motor_h_dist_set(steps as i32) }
                unsafe { self.api.motor_h_move() }
        
                if dir == Direction::Forward {
                    self.store.kv_store("current_x", [current_x + steps]).unwrap();
                } else {
                    self.store.kv_store("current_x", [current_x - steps]).unwrap();
                }
            },
            Action::Tilt => {
                unsafe { self.api.motor_v_dir_set(dir as i32) }
                unsafe { self.api.motor_v_dist_set(steps as i32) }
                unsafe { self.api.motor_v_move() }
        
                let current_y = self.store.kv_fetch("current_y").unwrap()[0];
                if dir == Direction::Forward {
                    self.store.kv_store("current_y", [current_y + steps]).unwrap();
                } else {
                    self.store.kv_store("current_y", [current_y - steps]).unwrap();
                }
            },
        }
        self.stop();
    }

    pub fn left(&mut self, mut steps: StepCount) {
        println!("ptz left steps={}", steps);

        let current_x = self.store.kv_fetch("current_x").unwrap()[0];
        if current_x + steps > _MAX_X {
            steps = _MAX_X - current_x;
        }

        self.move_(Action::Pan, Direction::Forward, steps);
    }

    pub fn right(&mut self, mut steps: StepCount) {
        println!("ptz right steps={}", steps);
        let current_x = self.store.kv_fetch("current_x").unwrap()[0];
        if (current_x as i8 - steps as i8) < 0 {
            steps = current_x;
        }

        self.move_(Action::Pan, Direction::Reverse, steps);
    }

    pub fn up(&mut self, mut steps: StepCount) {
        println!("ptz up steps={}", steps);
        let current_y = self.store.kv_fetch("current_y").unwrap()[0];
        if current_y + steps > _MAX_Y {
            steps = _MAX_Y - current_y;
        }

        self.move_(Action::Tilt, Direction::Forward, steps);
    }

    pub fn down(&mut self, mut steps: StepCount) {
        println!("ptz down steps={}", steps);
        let current_y = self.store.kv_fetch("current_y").unwrap()[0];
        if (current_y as i8 - steps as i8) < 0 {
            steps = current_y;
        }

        self.move_(Action::Tilt, Direction::Reverse, steps);
    }

    pub fn goto(&mut self, x: u8, y: u8) {
        println!("ptz goto x={} y={}", x, y);
        let current_x = self.store.kv_fetch("current_x").unwrap()[0];
        let current_y = self.store.kv_fetch("current_y").unwrap()[0];

        if x > current_x {
            self.left(x - current_x);
        } else if x < current_x {
            self.right(current_x - x);
        }
    
        if y > current_y {
            self.up(y - current_y);
        } else if y < current_y {
            self.down(current_y - y);
        }
    }

    pub fn stop(&mut self) {
        unsafe { self.api.motor_h_stop() }
        unsafe { self.api.motor_v_stop() }
    }

    pub fn save(&mut self, index: u8) {
        let current_x = self.store.kv_fetch("current_x").unwrap()[0];
        let current_y = self.store.kv_fetch("current_y").unwrap()[0];
        self.store.kv_store(format!("saved_pos_{}", index), [current_y.clone(), current_y.clone()]).unwrap();
        print!("saved [{:?}, {:?}] to index {:?}\n", current_x, current_y, index);
    }

    pub fn restore(&mut self, index: u8) {
        let target_pos = self.store.kv_fetch(format!("saved_pos_{}", index)).unwrap();
        self.goto(target_pos[0], target_pos[1]);
        print!("restored {:?} from index {:?}\n", target_pos, index);
    }
}

#[macro_use]
extern crate rouille;
use std::net::{SocketAddr, ToSocketAddrs};

extern crate clap;
use clap::{crate_version, App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new(NAME)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(crate_version!())
        .arg(
            Arg::with_name("libraries-dir")
                .short("L")
                .long("libraries-dir")
                .value_name("DIR_PATH")
                .help("Set path to the camera libraries directory")
                .env("MIJIA_LIB_PATH")
                .default_value(DEFAULT_LIBRARY_PATH),
        )
        .arg(
            Arg::with_name("database-path")
                .short("D")
                .long("database-path")
                .value_name("PATH")
                .help("Set path to the camera libraries")
                .env("CONTROL_DATABASE_PATH")
                .default_value(DEFAULT_DATABASE_PATH),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            App::new("ptz")
                .setting(AppSettings::SubcommandRequiredElseHelp)
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
                        .arg(Arg::with_name("steps").index(3).required(true)),
                )
                .subcommand(
                    App::new("goto").arg(
                        Arg::with_name("x").index(1).required(true),
                    ).arg(
                        Arg::with_name("y").index(2).required(true),
                    ),
                )
                .subcommand(
                    App::new("left").arg(Arg::with_name("steps").index(1).required(true)),
                )
                .subcommand(
                    App::new("right").arg(Arg::with_name("steps").index(1).required(true)),
                )
                .subcommand(
                    App::new("up").arg(Arg::with_name("steps").index(1).required(true)),
                )
                .subcommand(
                    App::new("down").arg(Arg::with_name("steps").index(1).required(true)),
                )
                .subcommand(
                    App::new("save").arg(Arg::with_name("index").index(1).required(true)),
                )
                .subcommand(
                    App::new("restore").arg(Arg::with_name("index").index(1).required(true)),
                )
        )
        .subcommand(
            SubCommand::with_name("server")
                .about("start web server")
                .arg(Arg::with_name("host").short("h").help("set target host"))
                .arg(
                    Arg::with_name("listen")
                        .default_value(DEFAULT_HOST_PORT)
                        .short("p")
                        .help("set target host:port"),
                ),
        )
        .get_matches();
    let lib_path = fs::canonicalize(matches.value_of("libraries-dir").unwrap().to_string()).expect("Libraries PATH is not correct.");
    let db_path = fs::canonicalize(matches.value_of("database-path").unwrap().to_string()).expect("Database PATH is not correct.");

    let mut ptz = PTZService::new(lib_path.clone(), db_path.clone());
    match matches.subcommand() {
        ("ptz", Some(matches)) => match matches.subcommand() {
            ("calibrate", Some(_)) => {
                ptz.calibrate();
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
            ("goto", Some(matches)) => {
                let target_x: u8 = matches.value_of("x").unwrap().parse().unwrap();
                let target_y: u8 = matches.value_of("y").unwrap().parse().unwrap();
                ptz.goto(target_x, target_y);
            }
            ("left", Some(matches)) => {
                let steps: StepCount = matches.value_of("steps").unwrap().parse().unwrap();
                ptz.left(steps);
            }
            ("right", Some(matches)) => {
                let steps: StepCount = matches.value_of("steps").unwrap().parse().unwrap();
                ptz.right(steps);
            }
            ("up", Some(matches)) => {
                let steps: StepCount = matches.value_of("steps").unwrap().parse().unwrap();
                ptz.up(steps);
            }
            ("down", Some(matches)) => {
                let steps: StepCount = matches.value_of("steps").unwrap().parse().unwrap();
                ptz.down(steps);
            }
            ("save", Some(matches)) => {
                let index: u8 = matches.value_of("index").unwrap().parse().unwrap();
                ptz.save(index);
            }
            ("restore", Some(matches)) => {
                let index: u8 = matches.value_of("index").unwrap().parse().unwrap();
                ptz.restore(index);
            }
            _ => unreachable!(),
        },
        ("server", Some(matches)) => {
            let addrs: Vec<_> = matches
                .value_of("listen")
                .unwrap()
                .to_socket_addrs()
                .expect("error: unable to listen address")
                .collect();
            let addr: SocketAddr = *addrs.first().unwrap();

            println!("server: listening on {}", addr);

            rouille::start_server(addr, move |request| {
                router!(request,
                    (GET) (/ptz/move/{action: Action}/{dir: Direction}/{steps: StepCount}) => {
                        PTZService::new(lib_path.clone(), db_path.clone()).move_(action, dir, steps);
                        rouille::Response::text("bip bop!\n").with_status_code(200)
                    },

                    (GET) (/ptz/goto/{x: u8}/{y: u8}) => {
                        PTZService::new(lib_path.clone(), db_path.clone()).goto(x, y);
                        rouille::Response::text("bip bop!\n").with_status_code(200)
                    },

                    (GET) (/ptz/left/{steps: StepCount}) => {
                        PTZService::new(lib_path.clone(), db_path.clone()).left(steps);
                        rouille::Response::text("bip bop!\n").with_status_code(200)
                    },

                    (GET) (/ptz/right/{steps: StepCount}) => {
                        PTZService::new(lib_path.clone(), db_path.clone()).right(steps);
                        rouille::Response::text("bip bop!\n").with_status_code(200)
                    },

                    (GET) (/ptz/up/{steps: StepCount}) => {
                        PTZService::new(lib_path.clone(), db_path.clone()).up(steps);
                        rouille::Response::text("bip bop!\n").with_status_code(200)
                    },

                    (GET) (/ptz/down/{steps: StepCount}) => {
                        PTZService::new(lib_path.clone(), db_path.clone()).down(steps);
                        rouille::Response::text("bip bop!\n").with_status_code(200)
                    },

                    (GET) (/ptz/save/{index: u8}) => {
                        PTZService::new(lib_path.clone(), db_path.clone()).save(index);
                        rouille::Response::text("bip bop!\n").with_status_code(200)
                    },

                    (GET) (/ptz/restore/{index: u8}) => {
                        PTZService::new(lib_path.clone(), db_path.clone()).restore(index);
                        rouille::Response::text("bip bop!\n").with_status_code(200)
                    },

                    _ => rouille::Response::text("bad request.\n").with_status_code(400),
                )
            });
        }
        _ => unreachable!(),
    }
}
