#include <stdio.h>


/* 
this file using for test this code on your pc instead of to test it on the camera
*/

void motor_init() {
    printf("[TEST] motor init started\n");
}
void motor_exit() {
    printf("[TEST] motor exit\n");
}
void motor_h_dir_set(int direction){};
void motor_h_position_get(){};
void motor_h_dist_set(int steps){};
void motor_h_move(){};
void motor_h_stop(){};   

void motor_v_dir_set(int direction){};
void motor_v_position_get(){};
void motor_v_dist_set(int steps){};
void motor_v_move(){};
void motor_v_stop(){};   