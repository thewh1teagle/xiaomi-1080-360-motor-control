#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dlfcn.h>



int main(void) {
    void *handle;

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

    handle = dlopen("./libdevice_kit.so", RTLD_LAZY);

    if (!handle) {
        /* fail to load the library */
        fprintf(stderr, "Error: %s\n", dlerror());
        return EXIT_FAILURE;
    }

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

    motor_init();
    motor_h_dir_set(1);
    motor_h_position_get();
    motor_h_dist_set(10);
    motor_h_move();
    motor_h_stop();   
    motor_exit();
    dlclose(handle);
    return EXIT_SUCCESS;
}