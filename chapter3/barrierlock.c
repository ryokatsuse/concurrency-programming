
void barrier(volatile int *cnt, int max) {
  __sync_fetch_add_add(cnt, 1);
  while(*cnt < max);
}

volatile int num = 10;

void *worker(void *arg) {
  barrier(&num, 10);
  // 何らかの処理
}

int main(int argc, char *argv[]) {
  pthread_t th[10];
  for (int i = 0; i < 10; i++)
  {
    if(pthread_create(&th[i], NULL, worker, NULL) != 0) {
      perror("pthread_create"); return -1;
    }
    return 0;
  }
}