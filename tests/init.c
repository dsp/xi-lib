#include <xilib.h>

int main(int argc, char ** argv) {
    XiHandle * xi = xi_init();
    xi_shutdown(xi);
    return 0;
}
