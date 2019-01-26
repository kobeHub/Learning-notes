/*Use the buffered channel to implementate
the work pool and simulate the process.*/
package main


import (
  "fmt"
  "sync"
  "time"
  "math/rand"
)


// define the Job to be process
type Job struct {
  id int
  randomno int    // The number to be process
}

// define the output Result type
type Result struct {
  job Job
  sumofdigits int
}

// Initialize the input and output channels with buffer cap 10
var (
  jobs = make(chan Job, 10)
  results = make(chan Result, 10)
)

// Process the input Jobs
// actually add all the digits of number
func digits(number int) int {
  sum := 0
  no := number
  for no != 0 {
    digit := no % 10
    sum += digit
    no /= 10
  }
  time.Sleep(1 * time.Second)   // sleep to simulate the work time_colon
  return sum
}

// Create the Wait group to wait all the Goroutines
func worker(wg *sync.WaitGroup) {
  for job := range jobs {
    output := Result{job, digits(job.randomno)}
    results <- output
  }
  wg.Done()
}


// Create the worker pool to manage the worker Goroutines
func createWorkerPool(noOfWorkers int) {
  var wg sync.WaitGroup
  for i := 0; i < noOfWorkers; i++ {
    wg.Add(1)
    go worker(&wg)
  }
  wg.Wait()
  close(results)
}

// Allocate jobs for workers
func allocate(noOfJobs int) {
  for i := 0; i < noOfJobs; i++ {
    randomno := rand.Intn(999)
    job := Job{i, randomno}
    jobs <- job
  }
  close(jobs)
}

// Print all the results
func result(done chan bool) {
  for result := range results {
    fmt.Printf("Job id %d, input random int %d, sum of digits %d\n",
                result.job.id, result.job.randomno, result.sumofdigits)
  }
  done <- true
}

func main() {
  startTime := time.Now()
  noOfJobs := 100
  go allocate(noOfJobs)
  done := make(chan bool)
  go result(done)
  noOfWorkers := 10
  createWorkerPool(noOfWorkers)
  <-done

  endTime := time.Now()
  residual := endTime.Sub(startTime)
  fmt.Println("Run time:", residual.Seconds(), "s")
}
