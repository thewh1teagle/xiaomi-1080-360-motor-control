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




int main(void) {
    motor_init();
    file_event_service(EVENT_FILE, callback_motor);
    motor_exit();
    return 0;
}