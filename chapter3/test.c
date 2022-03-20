// TAS
// 入力されたポインpの指す値がtrueならReturn
// falseならメモリの値をtrueに設定してfalseをReturn
bool test_and_set(bool *p) {
  if (*p) {
    return true;
  } else {
    *p = true;
    return false;
  }
}