#include <unistd.h>
#include <inttypes.h>
#include <sys/inotify.h>
#include <limits.h>
#include <unistd.h>
#include <stdio.h>
#include <pthread.h>
#include <string.h>
#include <stdlib.h>
#include "motor.h"

int h = 0;
int v = 0;


extern void motor_h_dir_set(int direction);
extern void motor_h_position_get();
extern void motor_h_dist_set(int steps);
extern void motor_h_move();
extern void motor_h_stop();

extern void motor_v_dir_set(int direction);
extern void motor_v_position_get();
extern void motor_v_dist_set(int steps);
extern void motor_v_move();
extern void motor_v_stop();


int raw_motor_move(int motor, int direction, int steps) {
    /* use xiaomi function from shared libary for 
    controlling the motor */

    switch (motor) {
        case PAN:
            motor_h_dir_set(direction);
            motor_h_position_get();
            motor_h_dist_set(steps);
            motor_h_move();
            motor_h_stop();    
            break;
        case TILT:
            motor_v_dir_set(direction);
            motor_v_position_get();
            motor_v_dist_set(steps);
            motor_v_move();
            motor_v_stop();
            break;
    }
    
    return 0;
}

int raw_motor_left(int steps){return raw_motor_move(PAN, FORWARD, steps);}
int raw_motor_right(int steps){return raw_motor_move(PAN, REVERSE, steps);}
int raw_motor_up(int steps){return raw_motor_move(TILT, FORWARD, steps);}
int raw_motor_down(int steps){ return raw_motor_move(TILT, REVERSE, steps);}




void motor_move(char *direction, int steps) {
    /* control the motor 
    args:
     - diretion - <left | right | up | down 
     - steps - int
     */

    if (strcmp(direction,"left") == 0) {
        raw_motor_left(steps);
    }
    else if (strcmp(direction,"right") == 0) {
        raw_motor_right(steps);
    }
    else if (strcmp(direction,"up") == 0) {
        raw_motor_up(steps);
    }
    else if (strcmp(direction,"down") == 0) {
        raw_motor_down(steps);
    }
}


void callback_motor() {
    /* 
    this function will called every time 
    the event file will modify.
    after that we read the value from file and then 
    controlling the motor with those values */

    char *contents = readFile(EVENT_FILE);
    char **argv = split(contents, " ");
    free(contents);
    int steps = atoi(argv[1]);
    char *direction = argv[0];
    free(argv);
    move(direction, steps);
}



void motor_calibrate() {
    //Set internal position to MAX without moving to make sure the functions allow a max # of steps
    h = MAX_H;
    v = MAX_V;

    //calibrate horizontal axis first, right is 0. Move to center afterwards
    motor_right(MAX_H);
    h = 0;
    motor_left(CENTER_H);

    //calibrate vertical axis, down is 0. Move to center afterwards
    motor_down(MAX_V);
    v = 0;
    motor_up(CENTER_V);
}


void store_pos(int h, int v) {
    /* 
    store position for motor in position_file 
    */
    FILE *fp;
    fp = fopen(POSITION_FILE, "w");
    fprintf(fp, "%d,%d", h, v);
    fclose(fp);
}

//Load current pos from file
void load_pos() {
    FILE *fp;
    char str[POS_LEN];

    fp = fopen(POSITION_FILE, "r");
    if (fp == NULL){
        printf("No position found, calibrating...");
        motor_calibrate();
        load_pos();
        return;
    }
    fgets(str, POS_LEN, fp);

    fclose(fp);


    // split params for h and v
    char *positions[] = split(str, ",");
    h = atoi(positions[0]);
    v = atoi(positions[1]);
    free(positions);
}


