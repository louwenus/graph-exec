#import "@preview/fletcher:0.5.7" as fletcher
#import "@preview/lovelace:0.3.0": *
// #import "@preview/shadowed:0.2.0": shadowed

// Colors
#let blue = rgb("#003366")
#let red = rgb("#D81E5B")
#let orange = rgb("#F0544F")
#let cream = rgb("#FDF0D5")
#let grey = rgb("#C6D8D3")


// Page template
#let template(doc) = [
  #set page(
    paper: "presentation-4-3",
    header: context [
      #set text(15pt, fill: white)
      #place(
        dx: -10pt,
        dy: 0pt,
        box(fill: blue, height: 40pt, width: 100% + 20pt)[ ],
      )
    ],
    footer: context [
      #set align(right)
      #set text(
        15pt,
        fill: rgb("#ffffff"),
        font: "Open Sans",
      )
      #place(
        dx: -10pt,
        box(fill: blue, height: 100%, width: 100% + 20pt)[
          #grid(
            columns: (33.3%, 33.3%, 33.4%),
            rows: 100%,
            align: center + horizon,
            [
              Louwen FRICOUT, n°47817
            ],
            [Tomasulo: du hardware au thread],
          )
          #place(
            dx: -5pt,
            right + horizon,
            counter(page).display(
              "1 / 1",
              both: true,
            ),
          )
        ],
      )
    ],
    footer-descent: 60%,
    margin: (x: 10pt),
  )

  #set text(size: 25pt)

  #show heading.where(level: 2): it => place(
    dy: -63pt,
    text(fill: white, font: "Open Sans", weight: "medium")[#it],
  )

  #show heading.where(level: 1): it => place(
    center + horizon,
    text()[#it],
  )

  // Disable Figure 1 in captions
  #show figure.caption: it => [#it.body]

  #doc
]


#let arrow = fletcher.edge.with(marks: "-|>")

#let divider = line(length: 90%, stroke: gray)

#let card(title, body, width: 80%, color: orange) = [
  #set align(center)
  #box(
    width: width,
    radius: 5pt,
    clip: true,
    [
      #set align(left)
      #grid(
        rows: (35pt, auto),
        columns: 100%,
        box(
          fill: color,
          width: 100%,
          height: 100%,
          inset: (x: 10pt),
          place(
            horizon,
            text(fill: white, weight: "medium", font: "Open Sans")[
              #title
            ],
          ),
        ), box(
          fill: color.lighten(60%),
          width: 100%,
          inset: (top: 10pt, bottom: 15pt, x: 10pt),
          [
            #body
          ],
        )
      )
    ],
  )
]
