package fiber_he

import (
	"testing"
)

func TestFiberHe(t *testing.T) {
	result := FiberHe("works")
	if result != "FiberHe works" {
		t.Error("Expected FiberHe to append 'works'")
	}
}
