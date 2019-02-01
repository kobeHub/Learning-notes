// package errors to implements functions to manipulate errors.
package errors

// return an error that formats as the given text.
func New(s string) error {
  return &errorString{s}
}

// errorString is a trivial implementation of error.
type errorString struct {
  s string
}

// implements the error interface, so the variable
// can assigned to error type.
func (e *errorString) Error() string {
  return e.s
}

