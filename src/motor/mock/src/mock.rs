#[no_mangle]
pub extern "C" fn motor_init() {println!("[MOCK] motor_init called!")}
#[no_mangle]
pub extern "C" fn motor_exit() {println!("[MOCK] motor_exit called!")}
#[no_mangle]
pub extern "C" fn motor_h_dir_set(direction: i32) {println!("[MOCK] motor_h_dir_set called!")}
#[no_mangle]
pub extern "C" fn motor_h_position_get() {println!("[MOCK] motor_h_position_get called!")}
#[no_mangle]
pub extern "C" fn motor_h_dist_set(steps: i32) {println!("[MOCK] motor_h_dist_set called!")}
#[no_mangle]
pub extern "C" fn motor_h_move() {println!("[MOCK] motor_h_move called!")}
#[no_mangle]
pub extern "C" fn motor_h_stop() {println!("[MOCK] motor_h_stop called!")}
#[no_mangle]
pub extern "C" fn motor_v_dir_set(direction: i32) {println!("[MOCK] motor_v_dir_set called!")}
#[no_mangle]
pub extern "C" fn motor_v_position_get() {println!("[MOCK] motor_v_position_get called!")}
#[no_mangle]
pub extern "C" fn motor_v_dist_set(steps: i32) {println!("[MOCK] motor_v_dist_set called!")}
#[no_mangle]
pub extern "C" fn motor_v_move() {println!("[MOCK] motor_v_move called!")}
#[no_mangle]
pub extern "C" fn motor_v_stop() {println!("[MOCK] motor_v_stop called!")}