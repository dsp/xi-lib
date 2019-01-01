#include <stdio.h>
#include <pthread.h>
#include <unistd.h>
#include <xilib.h>
#include <string.h>

void callme(const char* msg, uint32_t len) {
    printf("received: %s\n", msg);
}

void * xi_receiver_thread(void *user_data) {
    XiHandle *xi = (XiHandle *) user_data;
    xi_start_receiver(xi);
    return NULL;
}

void * xi_core_thread(void *user_data) {
    XiHandle *xi = (XiHandle *) user_data;
    xi_start_core(xi);
    return NULL;
}

int main(int argc, char ** argv) {
    pthread_t xi_core_thread_id, xi_receiver_thread_id;
    XiHandle * xi = xi_create(callme);

    pthread_create(&xi_core_thread_id, NULL, xi_core_thread, (void *) xi);
    pthread_create(&xi_receiver_thread_id, NULL, xi_receiver_thread, (void *) xi);

    const char * msg = "{\"method\": \"client_started\", \"params\": {\"config_dir\":null, \"client_extras_dir\": null}}";
    xi_send_message(xi, msg, strlen(msg) + 1);
    
    const char * msg2 = "{\"id\": 1, \"method\": \"new_view\", \"params\": {\"file_path\": null}}";
    xi_send_message(xi, msg2, strlen(msg2) + 1);

    pthread_join(xi_receiver_thread_id, NULL);
    pthread_join(xi_core_thread_id, NULL);

    xi_free(xi);
    return 0;
}
