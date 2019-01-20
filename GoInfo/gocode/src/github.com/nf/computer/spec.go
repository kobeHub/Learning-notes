package computer

type Spec struct {  // exported struct
  Maker string    // exported
  model string    // unexported
  Price float64   // exported
}
