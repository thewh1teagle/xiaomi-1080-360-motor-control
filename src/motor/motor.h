#define FORWARD 1
#define REVERSE 0
#define PAN 0
#define TILT 1

#define MAX_H 172       //Max steps horizontal
#define MAX_V 40        //Max steps vertical
#define CENTER_H 86     //Center pos horizontal
#define CENTER_V 20     //Center pos vertical
#define POS_LEN 7       //Format: xxx,yyy
#define MAXPATHLEN 200  //Max Len for full path


#define EVENT_FILE "event" // file for controlling the motor (by writing to it)
#define POSITION_FILE "position" // file for store the position



void raw_motor_move(int motor, int direction, int steps);
void raw_motor_left(int steps);
void raw_motor_right(int steps);
void raw_motor_up(int steps);
void raw_motor_down(int steps);
void callback_motor();
void motor_move(char *direction, int steps);
void store_pos(int h, int v);
void load_pos();
void motor_calibrate();
void callback_motor();

int h = 0;
int v = 0;
