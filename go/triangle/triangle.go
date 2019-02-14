package triangle

type Kind int

const (
	Equ Kind = iota // equilateral
	Iso             // isosceles
	Sca             // scalene
	NaT             // not a triangle
)

func KindFromSides(a, b, c float64) Kind {
	switch {
	case a < 0 || b < 0 || c < 0:
		return NaT
	case a >= b+c || b >= a+c || c >= a+b:
		return NaT
	case a == b && b == c:
		return Equ
	case a == b || a == c || b == c:
		return Iso
	default:
		return Sca
	}
}