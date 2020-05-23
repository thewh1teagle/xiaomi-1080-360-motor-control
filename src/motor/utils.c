#include <unistd.h>
#include <inttypes.h>
#include <sys/inotify.h>
#include <limits.h>
#include <unistd.h>
#include <stdio.h>
#include <pthread.h>
#include <string.h>
#include <stdlib.h>


char **split(char string[], char *sep) {
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