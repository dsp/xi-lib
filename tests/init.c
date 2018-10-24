#include <stdio.h>
#include <pthread.h>
#include <unistd.h>
#include <xilib.h>

void callme(const char* msg, uint32_t len) {
    printf("%s\n", msg);
}

void * xi_thread(void *user_data) {
    XiHandle *xi = (XiHandle *) user_data;
    xi_start(xi);
    usleep(50000); // 50 ms
    xi_shutdown(xi);
    return NULL;
}

int main(int argc, char ** argv) {
    pthread_t xi_thread_id;
    XiHandle * xi = xi_create(callme);
    pthread_create(&xi_thread_id, NULL, xi_thread, (void *) xi);
    xi_send_message(xi, "{\"method\": \"new_view\"}", sizeof("{\"method\": \"new_view\"}"));
    pthread_join(xi_thread_id, NULL);
    xi_free(xi);
    return 0;
}
