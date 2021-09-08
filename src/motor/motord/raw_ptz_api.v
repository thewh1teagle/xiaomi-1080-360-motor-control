module main

import dl
import os


pub enum MOTOR {
	pan = 0
	tilt = 1
}

pub enum Direction {
	forward = 1
	reverse = 0
}
pub enum HLimits {
	max_h = 73
	min_h = -73
	center_h = 86
	h_position = 0
}

struct Position {
	h_position int
	v_position int
}
pub enum Limits {
	max_v = 20
	min_v = -20
}

type NoPFunc = fn () 
type IntPFunc = fn (int)

struct RawPtzApi {
	motor_init fn()
	motor_exit fn()
	motor_h_dir_set fn(pos int)
	motor_h_position_get fn()
	motor_h_dist_set fn(pos int)
	motor_h_move fn()
	motor_h_stop fn()
	motor_v_dir_set fn(pos int)
	motor_v_position_get fn()
	motor_v_dist_set fn(pos int)
	motor_v_move fn()
	motor_v_stop fn()
}

fn load_raw_api() ? (RawPtzApi) {
	library_file_path := os.join_path(os.getwd(), dl.get_libname('libdevice_kit'))
	handle := dl.open_opt(library_file_path, dl.rtld_lazy) ?
	api := RawPtzApi {
		motor_init: NoPFunc( dl.sym_opt(handle, "motor_init") ? )
		motor_exit: NoPFunc( dl.sym_opt(handle, "motor_exit") ? )
		motor_h_dir_set: IntPFunc( dl.sym_opt(handle, "motor_h_dir_set") ? )
		motor_h_position_get: NoPFunc( dl.sym_opt(handle, "motor_h_position_get") ? )
		motor_h_dist_set: IntPFunc( dl.sym_opt(handle, "motor_h_dist_set") ? )
		motor_h_move: NoPFunc( dl.sym_opt(handle, "motor_h_move") ? )
		motor_h_stop: NoPFunc( dl.sym_opt(handle, "motor_h_stop") ? )
		motor_v_dir_set: IntPFunc( dl.sym_opt(handle, "motor_v_dir_set") ? )
		motor_v_position_get: NoPFunc( dl.sym_opt(handle, "motor_v_position_get") ? )
		motor_v_dist_set: IntPFunc( dl.sym_opt(handle, "motor_v_dist_set") ? )
		motor_v_move: NoPFunc( dl.sym_opt(handle, "motor_v_move") ? )
		motor_v_stop: NoPFunc( dl.sym_opt(handle, "motor_v_stop") ? )
	}
	return api	
}

fn (raw_api RawPtzApi) raw_pan(degrees f32, direction Direction) {
	raw_api.motor_h_dir_set(direction)
	raw_api.motor_h_position_get()
	raw_api.motor_h_dist_set(degrees)
	raw_api.motor_h_move()
	raw_api.motor_h_stop()
}

fn (raw_api RawPtzApi) raw_tilt(degrees f32, direction Direction) {
	raw_api.motor_v_dir_set(direction)
	raw_api.motor_v_position_get()
	raw_api.motor_v_dist_set(degrees)
	raw_api.motor_v_move()
	raw_api.motor_v_stop()
}