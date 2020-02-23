#include <stdio.h>
#include <unistd.h>
#include <inttypes.h>


#define MAX_LEFT 66
#define MAX_RIGHT 66
#define MAX_DOWN 13
#define MAX_UP 20


int mmotor(int motor, int direction, int steps) {
    // motor = 1,2
    // direction 1,2
    motor_init();
    switch (motor) {
    case 1:
        motor_h_dir_set(direction);
        motor_h_position_get();
        motor_h_dist_set(steps);
        motor_h_move();
        motor_h_stop();    
        break;
    case 2:
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


void motor_up(int steps) {mmotor(2, 1, steps);}
void motor_down(int steps) {mmotor(2, 0, steps);}
void motor_left(int steps) {mmotor(1, 1, steps);}
void motor_right(int steps) {mmotor(1, 0, steps);}

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