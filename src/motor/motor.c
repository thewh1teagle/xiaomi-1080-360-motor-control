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


int _motor_move(int motor, int direction, int steps) {
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

int _motor_left(int steps) { return _motor_move(PAN, FORWARD, steps); }
int _motor_right(int steps) { return _motor_move(PAN, REVERSE, steps); }
int _motor_up(int steps) { return _motor_move(TILT, FORWARD, steps); }
int _motor_down(int steps) { return _motor_move(TILT, REVERSE, steps); }




void move(char *direction, int steps) {
    if (strcmp(direction,"left") == 0) {
        _motor_left(steps);
    }
    else if (strcmp(direction,"right") == 0) {
        _motor_right(steps);
    }
    else if (strcmp(direction,"up") == 0) {
        _motor_up(steps);
    }
    else if (strcmp(direction,"down") == 0) {
        _motor_down(steps);
    }
}


void callback_motor() {
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

    char *positions[] = split(str, ",");
    ///split params for h and v
    h = atoi(positions[0]);
    v = atoi(positions[1]);
    free(positions);
}


