#include <unistd.h>
#include <inttypes.h>
#include <sys/inotify.h>
#include <limits.h>
#include <unistd.h>
#include <stdio.h>
#include <pthread.h>
#include <string.h>
#include <stdlib.h>

#include "utils.h"
#include "motor.h"


extern void motor_init();
extern void motor_exit();

int main(void) {
    motor_init(); // miio function
    file_event_service(EVENT_FILE, callback_motor); // start event listener
    motor_exit(); // miio function
    return 0;
}