  #include <stdio.h>
#include <unistd.h>
#include <inttypes.h>
#include <string.h>


#define FORWARD 1
#define REVERSE 0
#define PAN 0
#define TILT 1
#define MAX_LEFT 66
#define MAX_RIGHT 66
#define MAX_DOWN 13
#define MAX_UP 20

int motor_move(int motor, int direction, int steps) {
    motor_init();
    switch (motor) {
        case 0:
            motor_h_dir_set(direction);
            motor_h_position_get();
            motor_h_dist_set(steps);
            motor_h_move();
            motor_h_stop();    
            break;
        case 1:
            motor_v_dir_set(direction);
            motor_v_position_get();
            motor_v_dist_set(steps);
            motor_v_move();
            motor_v_stop();
            break;
    }
    motor_exit();
    return 0;
}

int motor_left(int steps) { return motor_move(PAN, FORWARD, steps); }
int motor_right(int steps) { return motor_move(PAN, REVERSE, steps); }
int motor_up(int steps) { return motor_move(TILT, FORWARD, steps); }
int motor_down(int steps) { return motor_move(TILT, REVERSE, steps); }


int main(int argc, char **argv) {

    if (argc < 3 ) {
        char filename[10];
        strcpy(filename, argv[0]);
        printf("Usage: \n%s <left|right|up|down> <steps>\n", filename);
        exit(1);
    } 
    
    char direction[10];
    strcpy(direction, argv[1]);
    int steps = atoi(argv[2]);

    if (strcmp(direction,"left") == 0) {
        motor_left(steps);
    }
    else if (strcmp(direction,"right") == 0) {
        motor_right(steps);
    }
    else if (strcmp(direction,"up") == 0) {
        motor_up(steps);
    }
    else if (strcmp(direction,"down") == 0) {
        motor_down(steps);
    }
}

