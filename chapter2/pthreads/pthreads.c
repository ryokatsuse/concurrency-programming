#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define NUM_THREADS 10

// スレッド用関数
void *thread_func(void *arg) {
    int id = (int)arg;
    for (int i = 0; i < 5; i++) {
        printf("id = %d, i = %d\n", id, i);
        sleep(1);
    }

    return "finished!"; // 返り値
}

