package main

import (
	"image/color"
	"math"
)

type Ray struct {
	origin    Vec3
	direction Vec3
}

func (r Ray) At(t float64) Vec3 {
	return r.origin.Add(r.direction.Multiply(t))
}

func (r Ray) HitSphere(center Vec3, radius float64) float64 {
	oc := r.origin.Subtract(center)
	a := r.direction.Dot(r.direction)
	b := 2.0 * oc.Dot(r.direction)
	c := oc.Dot(oc) - (radius * radius)
	discriminant := (b * b) - 4*a*c
	if discriminant < 0 {
		return -1.0
	} else {
		return (-b - math.Sqrt(discriminant)) / (2.0 * a)
	}

}

func (r Ray) Color() color.RGBA {
	t := r.HitSphere(Vec3{0, 0, -1}, 0.5)
	if t > 0.0 {
		n := r.At(t).Subtract(Vec3{0, 0, -1}).Unit()
		return color.RGBA{uint8(n.x * 0xff), uint8(n.y * 0xff), uint8(n.z * 0xff), 0xff}
	}

	t = 0.5 * (r.direction.Unit().y + 1.0)
	return color.RGBA{uint8(float64(0xff) - float64(0xff)*0.5*t), uint8(float64(0xff) - float64(0xff)*0.3*t), 0xff, 0xff}
}
