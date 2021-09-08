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
void motor_h_dir_set(int direction){
    printf("[TEST] motor_h_dir_set [VAL] %d\n", direction);
};
void motor_h_position_get(){
    printf("[TEST] motor_h_position_get\b");
};
void motor_h_dist_set(int steps){
    printf("[TEST] motor_h_dist_set [VAL] %d\n", steps);
};
void motor_h_move(){
    printf("[TEST] motor_h_move\n");
};
void motor_h_stop(){
    printf("[TEST] motor_h_stop\n");
};   
void motor_v_dir_set(int direction){
    printf("[TEST] motor_v_dir_set [VAL] %d\n", direction);
};
void motor_v_position_get(){
    printf("[TEST] motor_v_position_get\n");
};
void motor_v_dist_set(int steps){
    printf("[TEST] motor_v_dist_set [VAL] %d\n", steps);
};
void motor_v_move(){
    printf("[TEST] motor_v_move\n");
};
void motor_v_stop(){
    printf("[TEST] motor_v_stop\n");
};  