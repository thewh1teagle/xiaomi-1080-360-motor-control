#include <stdio.h>
#include "device_kit.h"

void motor_init() { printf("motor_init()\n"); }
void motor_h_dir_set(int dir) { printf("motor_h_dir_set(dir=%d)\n", dir); }
void motor_h_position_get() { printf("motor_h_position_get()\n"); }
void motor_h_dist_set(int steps) { printf("motor_h_dist_set(steps=%d)\n", steps); }
void motor_h_move() { printf("motor_h_move()\n"); }
void motor_h_stop() { printf("motor_h_stop()\n"); }
void motor_v_dir_set(int dir) { printf("motor_v_dir_set(dir=%d)\n", dir); }
void motor_v_position_get() { printf("motor_v_position_get()\n"); }
void motor_v_dist_set(int steps) { printf("motor_v_dist_set(steps=%d)\n", steps); }
void motor_v_move() { printf("motor_v_move()\n"); }
void motor_v_stop() { printf("motor_v_stop()\n"); }
void motor_exit() { printf("motor_exit()\n"); }
