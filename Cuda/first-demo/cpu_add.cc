#include <iostream>
#include <cmath>

inline void add(int n, float *a, float *b) {
  for (int i = 0; i < n; i++) {
    b[i] += a[i];
  }
}

int main() {
  const int N = 1 << 20;
  float *a = new float[N];
  float *b = new float[N];

  for (int i = 0; i < N; i++) {
    a[i] = 0.1f;
    b[i] = 0.2f;
  }

  add(N, a, b);
  // Check errors
  // all elements of b should be 0.3f
  float max_error = 0.0f;
  for (int i = 0; i < N; i++) {
    max_error = fmax(max_error, fabs(b[i] - 0.3f));
  }
  std::cout << "Max error using cpu: " << max_error << std::endl;

  delete[] a;
  delete[] b;
}
