#include <unistd.h>
#include <inttypes.h>
#include <sys/inotify.h>
#include <limits.h>
#include <unistd.h>
#include <stdio.h>
#include <pthread.h>
#include <string.h>
#include <stdlib.h>



#define FORWARD 1
#define REVERSE 0
#define PAN 0
#define TILT 1

#define MAX_H 172       //Max steps horizontal
#define MAX_V 40        //Max steps vertical
#define CENTER_H 86     //Center pos horizontal
#define CENTER_V 20     //Center pos vertical
#define POS_LEN 7       //Format: xxx,yyy
#define MAXPATHLEN 200  //Max Len for CALIBRATED = 1;full path


#define EVENT_FILE "event" // file for controlling the motor (by writing to it)
#define POSITION_FILE "position" // file for store the position

int H_POS = 0;
int V_POS = 0;
int CALIBRATED = 0;



char **split(char string[], const char *sep) {
    /*
    get string pointer and sep , return array of splited string 
    don't forget to free the return pointer 
    */
    char *token = strtok(string, sep);
    char **argv = calloc(1, sizeof(char*));
    
    int i = 0;
    while (token != NULL) {
        argv = realloc(argv, sizeof(argv) + sizeof(char*));
        argv[i] = calloc(strlen(token), sizeof(char));
        strcpy(argv[i++], token);
        token = strtok(NULL, sep);
    }
    return argv;
}

char *readFile(char *filename) {
    /* 
    get contents of file
    don't forget to free the buffer
    */
    char * buffer = 0;
    long length;
    FILE * f = fopen (filename, "r");
    if (f) {
        fseek (f, 0, SEEK_END);
        length = ftell (f);
        fseek (f, 0, SEEK_SET);
        buffer = malloc (length);
        if (buffer) {
            fread (buffer, 1, length, f);
        }
        fclose (f);
    }
    return buffer;
}

void file_event_service(char *pathname, void (*callback_function)()) {
    /*
    watch file changes, if the file changed,
    start callback function in new thread
    */
    printf("File event service started on file %s \n", pathname);
    int BUF_LEN = (10 * (sizeof(struct inotify_event) + _PC_NAME_MAX + 1));

    char buf[BUF_LEN];
    int inotify_fd = 0;
    struct inotify_event *event = NULL;

    inotify_fd = inotify_init();
    inotify_add_watch(inotify_fd, pathname, IN_ALL_EVENTS);
    while (1) {
        int n = read(inotify_fd, buf, BUF_LEN);
        char* p = buf;
        while (p < buf + n) {
            event = (struct inotify_event*)p;
            uint32_t mask = event->mask;
            if (mask & IN_CLOSE_WRITE) {
                pthread_t thread_id; 
                pthread_create(&thread_id, NULL, callback_function, NULL); 
                pthread_join(thread_id, NULL); 
            }
            p += sizeof(struct inotify_event) + event->len;
        }
    }
}







// Xiaomi motor horizontal control functions - from lib folder
extern void motor_h_dir_set(int direction);
extern void motor_h_position_get();
extern void motor_h_dist_set(int steps);
extern void motor_h_move();
extern void motor_h_stop();

// Xiaomi motor vertical control functions - from lib folder
extern void motor_v_dir_set(int direction);
extern void motor_v_position_get();
extern void motor_v_dist_set(int steps);
extern void motor_v_move();
extern void motor_v_stop();


void raw_motor_move(int motor, int direction, int steps) {
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
    
}

void raw_motor_left(int steps) {
    if (H_POS + steps > MAX_H) { // if position is more than max position than get the rest (and maybe nothing)
        steps = MAX_H-H_POS;
    }
    raw_motor_move(PAN, FORWARD, steps);
}
void raw_motor_right(int steps) {
    if (H_POS - steps < 0) {
        steps = H_POS;
    }
    raw_motor_move(PAN, REVERSE, steps);
}
void raw_motor_up(int steps) {
    if (V_POS + steps > MAX_V) {
        steps = MAX_V - V_POS;
    }
    raw_motor_move(TILT, FORWARD, steps);
}
void raw_motor_down(int steps) {
    if (V_POS - steps < 0) {
        steps = V_POS;
    } 
    raw_motor_move(TILT, REVERSE, steps);
}




void motor_move(char *direction, int steps) {
    
    /* control the motor 
    args:
     - diretion - <left | right | up | down 
     - steps - int
     */

    printf("Calibrated: %d \n", CALIBRATED);

    printf("moving %s %d steps \n", direction, steps);

    if (strcmp(direction,"left") == 0) {
        raw_motor_left(steps);
        V_POS += steps;
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
    motor_move(direction, steps);
}



void motor_calibrate() {
    printf("Calibrating motor... \nPlease wait.\n");
    //Set internal position to MAX without moving to make sure the functions allow a max # of steps
    H_POS = MAX_H;
    V_POS = MAX_V;

    //calibrate horizontal axis first, right is 0. Move to center afterwards
    motor_move("right", MAX_H);
    H_POS = 0;
    motor_move("left", CENTER_H);

    //calibrate vertical axis, down is 0. Move to center afterwards
    motor_move("down", MAX_V);
    V_POS = 0;
    motor_move("up", CENTER_V);
    CALIBRATED = 1;
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
    char **positions = split(str, ",");
    H_POS = atoi(positions[0]);
    V_POS = atoi(positions[1]);
    free(positions);
}




// Xiaomi functions from lib folder
extern void motor_init();
extern void motor_exit();

int main(void) {
    motor_init(); // miio function
    motor_calibrate();
    file_event_service(EVENT_FILE, callback_motor); // start event listener
    motor_exit(); // miio function
    return 0;
}