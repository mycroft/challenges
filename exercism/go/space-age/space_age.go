package space

type Planet string

func Age(seconds float64, name Planet) float64 {
	var orbital float64

	switch name {
	case "Mercury":
		orbital = 0.2408467
	case "Venus":
		orbital = 0.61519726
	case "Earth":
		orbital = 1.0
	case "Mars":
		orbital = 1.8808158
	case "Jupiter":
		orbital = 11.862615
	case "Saturn":
		orbital = 29.447498
	case "Uranus":
		orbital = 84.016846
	case "Neptune":
		orbital = 164.79132
	}

	return seconds / (orbital * 365.25 * 86400)
}
