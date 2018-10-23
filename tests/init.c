#include <stdio.h>
#include <xilib.h>

void callme(const char* msg, uint32_t len) {
    printf("%s\n", msg);
}

int main(int argc, char ** argv) {
    XiHandle * xi = xi_init(callme);
    xi_send_message(xi, "foo", sizeof("foo"));
    xi_shutdown(xi);
    return 0;
}
