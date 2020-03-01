#ifndef __DEVICE_KIT__
#define __DEVICE_KIT__

void motor_init();
void motor_h_dir_set(int dir);
void motor_h_position_get();
void motor_h_dist_set(int steps);
void motor_h_move();
void motor_h_stop();
void motor_v_dir_set(int dir);
void motor_v_position_get();
void motor_v_dist_set(int steps);
void motor_v_move();
void motor_v_stop();
void motor_exit();

/*
TODO: Document these symbols

00001eec T mijia_devicekit_exit
00001e58 T mijia_devicekit_init
000022f0 T mijia_led_control
00002548 T mijia_led_setpriority
00001f14 T mijia_motor_control
000151d4 B motor_h_drv
000030ac T motor_h_position_set
00002d60 T motor_h_start
000151c8 B motor_v_drv
00002b9c T motor_v_position_set
00002f78 T motor_v_start
0000383c T key_exit
00003814 T key_init
00003850 T key_value
00003918 T led_exit
00003858 T led_init
00003934 T led_set_value
*/

#endif