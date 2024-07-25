#import "@preview/cetz:0.2.2": canvas, plot, chart, draw, matrix, vector

#let style = (
  black: (stroke: black),
  blue: (stroke: blue),
  red : (stroke: red),
  mark : (stroke: none, fill: black),
  plane: (stroke: none, fill: rgb("#0003")),
  transparent: (stroke: none, fill: none),
)

=== Gradient descend

To understand how weights are adjusted in neuronal networks, its important to understand the mathematical foundation of gradient descent.
The simplest form of gradient descent is implemented by Newton's method.

$
x_(k+1) = x_k - (f'(x_k))/f(x_k)
$

Each step, the index is moved in the direction of the negative gradient, eventually approaching a local minima.
While conditions exist under which the method does not converge, it typically approaches a local minima rather quickly as shown in @fig-newton-method.

#raw(read("src/gradient.rs"), lang: "Rust")<code-newtons-method-rust>


// #image("../../bilder/ml/newton.svg")

#let f(x) = calc.pow(x, 2)
#let f_deriv(x) = 2.0 * x


#let x = -1.5;
#let next_x = -1.5;
#let tangents = ()
#let horizontals = ()
#let points = ()
#for _ in range(0, 4) {
  let y = f(x);
  points.push((x, y))
  next_x -= y / f_deriv(x); // Adjust x

  tangents.push(((x, y), (next_x, 0)))
  horizontals.push(((x, y), (x, 0)))

  x = next_x
}
#let _ = horizontals.remove(0)
#let _ = tangents.pop()

#let legend_item(t) = text(size: 9pt, t)
#let domain = (-1.6, 0.3)
#figure(
  kind: image,
  caption: [Descend of Newton's method for $f(x) = x^2$ starting from $x_0 =1.5$],
  canvas(length: 1cm, {
    plot.plot(
      name: "plot",
      size: (8, 6),
      x-tick-step: 1,
      y-tick-step: 1,
      y-label: $y=x^2$,
      x-grid: true,
      y-grid: true,
      legend: "legend.inner-north-east",
      legend-style: (item: (spacing: 0.2)),
      {
        plot.add(
          label: legend_item($f(x)$),
          style: style.black,
          domain: domain,
          x => calc.pow(x, 2)
        )
        plot.add(
          label: legend_item("tangents"),
          style: style.blue,
          ((0, 0), (0, 0)),
        )
        plot.add(
          label: legend_item("x values"),
          style: style.red,
          ((0, 0), (0, 0)),
        )
        for horizontal in horizontals {
          plot.add(
            style: style.red,
            horizontal
          )
        }
        for tangent in tangents {
          plot.add(
            style: style.blue,
            tangent
          )
        }
        plot.add(
          style: style.transparent,
          mark-style: style.mark,
          mark: "o",
          mark-size: 0.12,
          points
        )
        for idx in range(0, points.len()) {
          plot.add-anchor("pt" + str(idx), points.at(idx))
        }
      })
      for idx in range(0, points.len()) {
        draw.content("plot.pt" + str(idx), text(size: 8pt, $k=idx$), anchor: "south-west", padding: .1)
      }
    }
  )
)<fig-newton-method>

For machine learning however, there are typically many parameters in contrast to the single parameter that can be optimized with Newton's method.
Yet, the underlying mechanism doesn't change.
Instead, partial derivatives are calculated to form a gradient, which describes the multi-dimensional vector of the greatest slope at a given point. 
Adjusting the parameters by applying the negative gradient will move them towards a local minimum.
For two parameters, this can be visualized in three-dimensional space as shown in @fig-gradient-descend-3d.

#figure(
  kind: image,
  caption: [Visualization of gradient descend in three-dimensional space at different starting points],
  canvas(length: 1.3cm, {
    import draw: scale, set-transform, grid, line, mark
    import calc: pow, exp

    // Set up the transformation matrix
    set-transform(matrix.transform-rotate-dir((-1.3, 1.3, -1.3), (0, 1, .5)))
    scale(x: 1.3, z: -1)

    grid((-3,-3), (3,3), stroke: gray + .5pt)

    let fn(x, y) = exp(-pow(x * y, 2)) + exp(-pow((x - 1.1) * (y - 1), 2))
    let point(x, y) = (x, y, fn(x, y))

    // Draw a sine wave on the xy plane
    let wave(scale: 5, samples: 22) = {
      let points = ()
      for x in range(0, samples + 1) {
        let x_points = ()
        for y in range(0, samples + 1) {
          let x = x / samples * scale - scale / 2
          let y = y / samples * scale - scale / 2
          x_points.push(point(x, y))
        }
        points.push(x_points)
      }
      for x in range(0, samples) {
        for y in range(0, samples) {
          line(
            points.at(x).at(y),
            points.at(x).at(y + 1),
            points.at(x + 1).at(y + 1),
            points.at(x + 1).at(y),
            stroke: rgb("#000") + 0.5pt,
            fill: blue.darken(40%).transparentize(80%),
            close: true
          )
        }
      }

      let dx(x, y) = -2 * x + pow(y, 2) * exp(-pow(x, 2) * pow(y, 2)) - 2 * (x - 1.1) * pow(y - 1, 2) * exp(-pow(x - 1.1, 2) * pow(y - 1, 2))
      let dy(x, y) = -2 * pow(x, 2) * y * exp(-pow(x, 2) * pow(y, 2)) - 2 * pow(x - 1.1, 2) * (y - 1) * exp(-pow(x - 1.1, 2) * pow(y - 1, 2))

      let descend(point_x, point_y, downscale: 2, samples: 9) = {
        let z = fn(point_x, point_y)
        let points = ()
        points.push(((point_x, point_y, z + 0.1), (point_x, point_y, z + 0.03)))
        for idx in range(0, samples) {
          let new_x = point_x - dx(point_x, point_y) / (downscale + idx)
          let new_y = point_y - dy(point_x, point_y) / (downscale + idx)
          points.push((point(point_x, point_y), point(new_x, new_y)))
          point_x = new_x
          point_y = new_y
        }
        points
      }

      let points_green = descend(0.386, 0.40, downscale: 3, samples: 13)
      let points_yellow = descend(0.309, 1.00, downscale: 4, samples: 9)
      let points_red = descend(0.489, 1.01)

      let m(p, color) = {
        mark(p.at(0), p.at(1), symbol: "stealth", fill: color, stroke: 0.56pt, scale: 1.3)
      }
      

      let first_m(p, color) = {
        mark(p.at(0), p.at(1), symbol: ">", fill: color, stroke: 0.59pt, scale: 1.9)
      }

      first_m(points_yellow.remove(0), yellow)
      for p in points_yellow {
        m(p, yellow)
      }

      first_m(points_green.remove(0), green)
      for p in points_green {
        m(p, green)
      }

      first_m(points_red.remove(0), red)
      for p in points_red {
        m(p, red)
      }
    }


    wave()
  })
)<fig-gradient-descend-3d>

