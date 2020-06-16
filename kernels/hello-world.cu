#include<stdio.h>
#include<stdlib.h>

extern "C" __global__ void hello_world(void) {
    printf("Hello World! from thread [%d,%d] on GPGPU\n", threadIdx.x, blockIdx.x);
}
