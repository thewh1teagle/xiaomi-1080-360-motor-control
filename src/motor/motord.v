module main

import os
import dl


type NoPFunc = fn () 
type IntPFunc = fn (int)

fn main() {



	// EVENT_FILE := "event" // file for controlling the motor (by writing to it)
	// STATUS_FILE := "status" // file for status of motor, to know if max offset of direction

	// FORWARD := 1
	// REVERSE := 0
	// PAN := 0
	// TILT := 1

	// MAX_H := 73      //Max steps horizontal
	// MIN_H := -73
	// MAX_V := 20        //Max steps vertical
	// MIN_V := -20
	// CENTER_H := 86     //Center pos horizontal
	// CENTER_V := 20     //Center pos vertical


	// H_POSITION := 0
	// V_POSITION := 0

	library_file_path := os.join_path(os.getwd(), dl.get_libname('libdevice_kit'))
	handle := dl.open_opt(library_file_path, dl.rtld_lazy) ?
	// eprintln('handle: ${ptr_str(handle)}')
	motor_init := NoPFunc( dl.sym_opt(handle, "motor_init") ? )
	motor_exit := NoPFunc( dl.sym_opt(handle, "motor_exit") ? )
	motor_h_dir_set := IntPFunc( dl.sym_opt(handle, "motor_h_dir_set") ? )
	motor_h_position_get := NoPFunc( dl.sym_opt(handle, "motor_h_position_get") ? )
	motor_h_dist_get := IntPFunc( dl.sym_opt(handle, "motor_h_dist_set") ? )
	motor_h_move := NoPFunc( dl.sym_opt(handle, "motor_h_move") ? )
	motor_h_stop := NoPFunc( dl.sym_opt(handle, "motor_h_stop") ? )
	motor_v_dir_set := IntPFunc( dl.sym_opt(handle, "motor_v_dir_set") ? )
	motor_v_position_get := NoPFunc( dl.sym_opt(handle, "motor_v_position_get") ? )
	motor_v_dist_set := IntPFunc( dl.sym_opt(handle, "motor_v_dist_set") ? )
	motor_v_move := NoPFunc( dl.sym_opt(handle, "motor_v_move") ? )
	motor_v_stop := NoPFunc( dl.sym_opt(handle, "motor_v_stop") ? )



	// eprintln('f: ${ptr_str(motor_init)}')
	motor_init()
}

