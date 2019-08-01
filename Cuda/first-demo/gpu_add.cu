#include <iostream>
#include <cmath>


// CUDA kernel function using GPU
__global__
void add(int n, float *a, float *b) {
  int index = threadIdx.x;
  int stride = blockDim.x;
  
  for (int i = index; i < n; i += stride) {
    b[i] += a[i];
  }
}

int main() {
  const int N = 1 << 20;
  float *a, *b;
  cudaMallocManaged(&a, N*sizeof(float));
  cudaMallocManaged(&b, N*sizeof(float));

  // Init arrays from host
  for (int i = 0; i < N; i++) {
    a[i] = 0.1f;
    b[i] = 0.2f;
  }

  // Run kernel function using tripe angle bracket
  add<<<1, 256>>>(N, a, b);

  // Wait for GPU results before hosts process
  cudaDeviceSynchronize();

  // Check errors
  // all elements of b should be 0.3f
  float max_error = 0.0f;
  for (int i = 0; i < N; i++) {
    max_error = fmax(max_error, fabs(b[i] - 0.3f));
  }
  std::cout << "Max error using GPU: " << max_error << std::endl;

  cudaFree(a);
  cudaFree(b);
}
