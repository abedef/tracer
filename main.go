package main

import (
	"flag"
	"fmt"
	"image"
	"image/png"
	"os"
	"time"
)

func main() {
	startTime := time.Now()

	renderWidth := flag.Int("width", 256, "width of rendered image")
	renderHeight := flag.Int("height", 256, "height of rendered image")
	flag.Parse()

	aspectRatio := float64(*renderWidth) / float64(*renderHeight)

	viewportHeight := 2.0
	viewportWidth := aspectRatio * viewportHeight
	focalLength := 1.0

	camera := Camera{Vec3{0, 0, 0}}

	horizontal := Vec3{viewportWidth, 0, 0}
	vertical := Vec3{0, viewportHeight, 0}
	lowerLeftCorner := camera.origin.Subtract(Vec3{0, 0, focalLength}).Subtract(vertical.Divide(2)).Subtract(horizontal.Divide(2))

	upLeft := image.Point{0, 0}
	lowRight := image.Point{*renderWidth, *renderHeight}

	img := image.NewRGBA(image.Rectangle{upLeft, lowRight})

	for j := *renderHeight - 1; j >= 0; j-- {
		fmt.Fprintf(os.Stderr, "\rScanlines remaining: %-8v", j)
		for i := 0; i < *renderWidth; i++ {
			u := float64(i) / (float64(*renderWidth) - 1)
			v := float64(j) / (float64(*renderHeight) - 1)

			ray := Ray{camera.origin, lowerLeftCorner.Add(horizontal.Multiply(u)).Add(vertical.Multiply(v)).Subtract(camera.origin)}
			img.Set(i, j, ray.Color())
		}
	}

	elapsedTime := time.Since(startTime)
	fmt.Fprintf(os.Stderr, "\rRender complete in %vs\n", elapsedTime.Seconds())

	f, _ := os.Create("image.png")
	png.Encode(f, img)
}
