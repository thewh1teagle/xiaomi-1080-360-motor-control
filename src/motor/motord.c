#include <dlfcn.h>
#include <inttypes.h>
#include <sys/inotify.h>
#include <limits.h>
#include <unistd.h>
#include <stdio.h>
#include <pthread.h>
#include <string.h>
#include <stdlib.h>

#define EVENT_FILE "event" // file for controlling the motor (by writing to it)
#define STATUS_FILE "status" // file for status of motor, to know if max offset of direction

#define FORWARD 1
#define REVERSE 0
#define PAN 0
#define TILT 1

#define MAX_H 73       //Max steps horizontal
#define MIN_H -73
#define MAX_V 20        //Max steps vertical
#define MIN_V -20
#define CENTER_H 86     //Center pos horizontal
#define CENTER_V 20     //Center pos vertical


int H_POSITION = 0;
int V_POSITION = 0;



void (*motor_init)();
void (*motor_exit)();

void (*motor_h_dir_set)(int direction);
void (*motor_h_position_get)();
void (*motor_h_dist_set)(int steps);
void (*motor_h_move)();
void (*motor_h_stop)();

void (*motor_v_dir_set)(int direction);
void (*motor_v_position_get)();
void (*motor_v_dist_set)(int steps);
void (*motor_v_move)();
void (*motor_v_stop)();






void miio_motor_move(int motor, int direction, int steps) {
    /* use xiaomi function from shared libary for 
    controlling the motor */
    printf("[DEBUG] raw_motor_move %d %d %d \n", motor, direction, steps);
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
    FILE *fp;
    fp = fopen (pathname,"w");
    fclose(fp);
    /*
    watch file changes, if the file changed,
    start callback function in new thread
    */
    printf("[DEBUG] File event service started on file %s \n", pathname);
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



void write_motor_status(int status) {
    FILE *fp;
    fp = fopen(STATUS_FILE, "w");
    if (status == 1) {
        fprintf(fp, "1\n");
    } else {
        fprintf(fp, "0\n");
    }
    fclose(fp);
}

void motor_move(motor, direction, steps) {
    printf("[DEBUG] H_POSITION: %d \nV_POSITION: %d \n", H_POSITION, V_POSITION);
    switch (motor) {
    case PAN:
        if (direction == FORWARD) {
            if (H_POSITION + steps > MAX_H) {
                write_motor_status(1);
                printf("[DEBUG] MAX H! \n");
                return;
            } 
            write_motor_status(0);
            H_POSITION += steps;
        } else {
            if (H_POSITION - steps < MIN_H) {
                write_motor_status(1);
                printf("[DEBUG] MIN H! \n");
                return;
            }
            write_motor_status(0);
            H_POSITION -= steps;
        }
        break;
    
    case TILT:
        if (direction == FORWARD) {
            if (V_POSITION + steps > MAX_V) {
                write_motor_status(1);
                printf("[DEBUG] MAX V! \n");
                return;
            }
            write_motor_status(0);
            V_POSITION += steps;
        } else {
            if (V_POSITION - steps < MIN_V) {
                write_motor_status(1);
                printf("[DEBUG] MIN V! \n");
                return;
            } 
            write_motor_status(0);
            V_POSITION -= steps;
        break;
        }
    }
    miio_motor_move(motor, direction, steps);
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
    
    int motor, direction, steps = 0;

    if (strcmp(argv[0], "calibrate") == 0) {
        motor_calibrate();
    }
    else {
        if (strcmp(argv[0], "pan") == 0) {
            motor = PAN;
        } else {
            motor = TILT;
        }

        if (strcmp(argv[1], "forward") == 0) {
            direction = FORWARD;
        } else {
            direction = REVERSE;
        }
        
        steps = atoi(argv[2]);
        motor_move(motor, direction, steps);
    }
    free(argv);
}

void reset_motor() {
    H_POSITION = 0;
    V_POSITION = 0;
    write_motor_status(0);
}



void motor_calibrate() {
    printf("[DEBUG] Calibrating motor...\n");
    //Set internal position to MAX without moving to make sure the functions allow a max # of steps

    //calibrate horizontal axis first, right is 0. Move to center afterwards
    miio_motor_move(PAN, FORWARD, MAX_H + abs(MIN_H) + 10);
    miio_motor_move(PAN, REVERSE, CENTER_H);

    //calibrate vertical axis, down is 0. Move to center afterwards
    miio_motor_move(TILT, FORWARD, MAX_V + abs(MIN_V) + 4);
    miio_motor_move(TILT, REVERSE, CENTER_V);
    reset_motor();
}



void dl_load(void *handle) {
    *(void**)(&motor_init) = dlsym(handle, "motor_init");
    *(void**)(&motor_exit) = dlsym(handle, "motor_exit");


    *(void**)(&motor_h_dir_set) = dlsym(handle, "motor_h_dir_set");
    *(void**)(&motor_h_position_get) = dlsym(handle, "motor_h_position_get");
    *(void**)(&motor_h_dist_set) = dlsym(handle, "motor_h_dist_set");
    *(void**)(&motor_h_move) = dlsym(handle, "motor_h_move");
    *(void**)(&motor_h_stop) = dlsym(handle, "motor_h_stop");
 

    *(void**)(&motor_v_dir_set) = dlsym(handle, "motor_v_dir_set");
    *(void**)(&motor_v_position_get) = dlsym(handle, "motor_v_position_get");
    *(void**)(&motor_v_dist_set) = dlsym(handle, "motor_v_dist_set");
    *(void**)(&motor_v_move) = dlsym(handle, "motor_v_move");
    *(void**)(&motor_v_stop) = dlsym(handle, "motor_v_stop");
}





int main(int argc, char *argv[]) {
    void *handle;
    handle = dlopen("libdevice_kit.so", RTLD_LAZY);
    if (!handle) {
    /* fail to load the library */
        fprintf(stderr, "Error: %s\n", dlerror());
        return EXIT_FAILURE;
    } 
    dl_load(handle);
    motor_init();
    if (argc > 1) {
        if (strcmp(argv[1], "--calibrate") == 0) {
            motor_calibrate();
        }
    }
    reset_motor();

    file_event_service(EVENT_FILE,callback_motor);
    

    motor_exit();
    dlclose(handle);
    return EXIT_SUCCESS;
}