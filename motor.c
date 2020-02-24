#include <stdio.h>
#include <unistd.h>
#include <inttypes.h>

#define FORWARD 0
#define BACKWARD 1
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
int motor_right(int steps) { return motor_move(PAN, BACKWARD, steps); }
int motor_up(int steps) { return motor_move(TILT, FORWARD, steps); }
int motor_down(int steps) { return motor_move(TILT, BACKWARD, steps); }

int main() {
    int up_p, down_p, left_p, right_p = 0; // motor position
    motor_up(2);
    while(1) {
        int a = service();
        if (a) {
            break;
        }
    }
    return 0;
}

int service() { // for now it's for testing max position 
    system ("/bin/stty raw");
    if (getchar() == '\033') { // if the first value is esc
    getchar(); // skip the [
    switch(getchar()) { // the real value
        case 'A':
            motor_up(5);
            break;
        case 'B':
            motor_down(5);
            break;
        case 'C':
            motor_right(5);
            break;
        case 'D':
            motor_left(5);
            break;
        }
    }
    else {
        return 0;
    }
}
