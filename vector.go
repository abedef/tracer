package main

import (
	"fmt"
	"math"
)

type Vec3 struct {
	x float64
	y float64
	z float64
}

func (v Vec3) Negative() Vec3 {
	return Vec3{x: -v.x, y: -v.y, z: -v.z}
}

func (v Vec3) Add(u Vec3) Vec3 {
	return Vec3{x: v.x + u.x, y: v.y + u.y, z: v.z + u.z}
}

func (v Vec3) Subtract(u Vec3) Vec3 {
	return Vec3{x: v.x - u.x, y: v.y - u.y, z: v.z - u.z}
}

func (v Vec3) Multiply(scalar float64) Vec3 {
	return Vec3{x: v.x * scalar, y: v.y * scalar, z: v.z * scalar}
}

func (v Vec3) Divide(scalar float64) Vec3 {
	return Vec3{x: v.x / scalar, y: v.y / scalar, z: v.z / scalar}
}

func (v Vec3) Squares() float64 {
	return (v.x * v.x) + (v.y * v.y) + (v.z * v.z)
}

func (v Vec3) Magnitude() float64 {
	return math.Sqrt(v.Squares())
}

func (v Vec3) String() string {
	return fmt.Sprint(v.x, v.y, v.z)
}

func (v Vec3) Dot(u Vec3) float64 {
	return u.x*v.x + u.y*v.y + u.z*v.z
}

func (v Vec3) Cross(u Vec3) Vec3 {
	return Vec3{
		x: u.y*v.z - u.z*v.y,
		y: u.z*v.x - u.x*v.z,
		z: u.x*v.y - u.y*v.x,
	}
}

func (v Vec3) Unit() Vec3 {
	return v.Divide(v.Magnitude())
}
