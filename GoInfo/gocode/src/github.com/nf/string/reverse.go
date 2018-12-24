package string

func Reverse(s string) string {
  b := []rune(s)    // The string shall be transfer to rune to use for unicode
  for i := 0; i < len(b)/2; i++ {
    j := len(b) - i - 1
    b[i], b[j] = b[j], b[i]
  }
  return string(b)
}
