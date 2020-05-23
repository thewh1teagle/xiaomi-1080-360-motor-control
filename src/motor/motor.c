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
#define MAX_LEFT 66
#define MAX_RIGHT 66
#define MAX_DOWN 13
#define MAX_UP 20
#define EVENT_FILE "value"





void file_event_service(char *pathname, void (*callback_function)()) {
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



char *readFile(char *filename) {
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



char **split(char string[], char *sep) {
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





int motor_move(int motor, int direction, int steps) {
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
    
    return 0;
}

int motor_left(int steps) { return motor_move(PAN, FORWARD, steps); }
int motor_right(int steps) { return motor_move(PAN, REVERSE, steps); }
int motor_up(int steps) { return motor_move(TILT, FORWARD, steps); }
int motor_down(int steps) { return motor_move(TILT, REVERSE, steps); }




void move(char *direction, int steps) {
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


void callback_motor() {
    char *contents = readFile(EVENT_FILE);
    char **argv = split(contents, " ");
    free(contents);
    int steps = atoi(argv[1]);
    char *direction = argv[0];
    free(argv);
    move(direction, steps);
}


int main(void) {
    motor_init();
    file_event_service(EVENT_FILE, callback_motor);
    motor_exit();
    return 0;
}

