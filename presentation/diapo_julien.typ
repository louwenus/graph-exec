#import "@preview/fletcher:0.5.7" as fletcher: diagram, node, edge
#import "@preview/cetz:0.3.4"
#import "@preview/cetz-plot:0.1.1": plot

#import "config.typ": *
#show: template


#place(dy: 50pt, center + top)[
  = Plus court chemin dans un cross en parapente
]

#place(
  dy: 0pt,
  center + bottom,
  box(
    height: 60%,
    figure(
      box(
        radius: 5pt,
        clip: true,
        image("assets/suisse-compressed.jpg"),
      ),
      caption: text(size: 18pt)[Parapentes au Cousimbert, Suisse],
    ),
  ),
)

#pagebreak()

== Sommaire

#place(center + horizon)[
  #set align(left)
  + Introduction
  + Modélisation du problème
  + Résolution par un algorithme naïf
  + Approximation des solutions
]

#pagebreak()

== Introduction

= 1) Introduction


#pagebreak()

== Introduction

#place(center + horizon)[
  #figure(
    image("assets/track.jpg"),
    caption: [Balises de la #underline[Bornes to Fly] 2025],
  )
]

#pagebreak()

== Introduction

#place(center + horizon)[
  === Problématique
  Comment modéliser et optimiser la recherche du plus court chemin dans un graphe temporel dans le cadre d'un cross en parapente ?
]

#pagebreak()

== Modélisation

= 2) Modélisation du problème

#pagebreak()

== Modélisation

#place(center + horizon)[
  #set align(left)

  On modélise la carte par un graphe orienté pondéré $G(S, A, Delta t)$ tel que:
  - Les sommets $S$ représentent les points d'intérêts (sommet, balise, col, décollage...)
  - Deux sommets $(a, b)$ sont reliés s'il est possible d'aller de $a$ à $b$.
  - $Delta t: A -> NN$ associe à chaque arête un temps de parcours.
]

#pagebreak()

== Modélisation

#place(center + horizon)[
  #grid(
    columns: (50%, 0%, 50%),
    image("assets/track-graph.jpg"),
    [$-->$],
    [
      #image("assets/track-graph.jpg")
      #place(
        dy: -100%,
        dx: 46.5pt,
        image("assets/track-graph-model-no-back.svg"),
      )
    ],
  )
]

#pagebreak()

== Modélisation

#let example(
  radius: 40pt,
  n1,
  n2,
  n3,
  n4,
  n5,
  a1,
  a2,
  a3,
  a4,
  a5,
  a6,
  a7,
) = diagram(
  cell-size: 150pt,
  node-stroke: 2pt,
  node((0, 0), n1, radius: radius),
  node((1, 0.2), n2, radius: radius),
  node((0.3, 0.9), n3, radius: radius),
  node((2.3, 0.9), n4, radius: radius),
  node((1.2, 1.4), n5, radius: radius),

  arrow((0, 0), (1, 0.2), label: a1, label-side: center),
  arrow((0, 0), (0.3, 0.9), label: a2, label-side: center),
  arrow((1, 0.2), (2.3, 0.9), label: a3, label-side: center),
  arrow((1, 0.2), (0.3, 0.9), label: a4, label-side: center),
  arrow((0.3, 0.9), (2.3, 0.9), label: a5, label-side: center),
  arrow((0.3, 0.9), (1.2, 1.4), label: a6, label-side: center),
  arrow((1.2, 1.4), (2.3, 0.9), label: a7, label-side: center),
)

#example(
  [],
  [],
  [],
  [],
  [],
  [#text(fill: orange)[$ 10"min" $]],
  [$ 5"min" $],
  [$ 15"min" $],
  [$ 5"min" $],
  [$ 10"min" $],
  [$ 10"min" $],
  [$ 5"min" $],
)

#pagebreak()

#example(
  [$ 1000"m" $],
  [$ 700"m" $],
  [$ 800"m" $],
  [$ 800"m" $],
  [$ 900"m" $],
  [$ 10"min" \ -200m $],
  [#place(dx: -50pt, $ 5"min" \ -100m $)],
  [$ 15"min" \ -300m $],
  [$ 5"min" \ +200m $],
  [$ 10"min" \ -50m $],
  [#place(dy: 30pt, dx: -25pt, $ 10"min" \ +200m $)],
  [#place(dy: 30pt, dx: 25pt, $ 5"min" \ -50m $)],
)

#pagebreak()

== Modélisation

#place(center + horizon)[
  === Première contrainte
  L'altitude du parapente doit rester supérieure à l'altitude du sommet

  \

  $G=(S, h, A, Delta t, Delta h)$ \
  #box()[
    #set align(left)
    - $h: S -> NN$ donne l'altitude de chaque sommet.
    - $Delta h: A -> NN$ associe à chaque transition son gain/perte en altitude.
  ]
]

#pagebreak()

== Modélisation

#place(center + horizon)[
  === Deuxième contrainte
  L'aéorologie évolue au cours de la journée

  \

  $G=(S, h, A, Delta t, Delta h)$ \
  #box()[
    #set align(left)
    - $Delta h: (A times NN) -> NN$ associe à chaque transition son gain/perte en altitude à un instant donné.
  ]
]

#pagebreak()

== Modélisation

#place(
  center + horizon,
  grid(rows: (50%, 50%), card(
      width: 90%,
      [Chemin possible],
      [Un chemin $c=(x_1, ..., x_n)$ est possible si \
        $
          forall i in [|2, n|], \ h(x_1) + Delta h(x_1, x_2) + dots.h.c + Delta h(x_(i-1), x_i) >= h(x_i)
        $],
    ), card(
      width: 90%,
      color: blue,
      [Solution partielle],
      [$(t, a)$ est une solution partielle pour un sommet $v$ si il existe un chemin possible du départ jusqu'à $v$ qui prend une durée $t$ et arrive avec une altitude $a$],
    )),
)

#pagebreak()

== Algorithme naïf

= 3) Résolution par un algorithme naïf

#pagebreak()

== Algorithme naïf

Il n'est pas possible d'utiliser l'algorithme de Djikstra

#place(center + horizon)[
  #diagram(
    cell-size: 150pt,
    node-stroke: 2pt,
    node((0, 0), "h=2", radius: 40pt),
    node((1, 0), "h=1", radius: 40pt),
    node((1, 1), "h=1", radius: 40pt),
    node((2, 0), "h=0", radius: 40pt),

    arrow((-1 / 2, 0), (0, 0), stroke: red),
    arrow((0, 0), (1, 0), label: [0s, $Delta h=1$]),
    arrow(
      (0, 0),
      (1, 1),
      label: place(dx: -140pt, dy: 20pt, box(width: 200pt)[1s, $Delta h=0$]),
      stroke: red,
    ),
    arrow((1, 1), (1, 0), label: [1s, $Delta h=0$], stroke: red),
    arrow((1, 0), (2, 0), label: [1s, $Delta h=2$], stroke: red),
    arrow((2, 0), (2 + 1 / 2, 0), stroke: red),
  )
]

#pagebreak()

== Algorithme naïf

#place(center + horizon)[
  #pseudocode-list(line-numbering: none)[
    + $G=(S, h, A, Delta t, Delta h)$
    + résultat $= +oo$
    + *explorer*($s$, $C$, durée, altitude):
      + *si* $s =$ arrivée *alors:*
        + $"résultat" = min("résultat", "durée")$
      + *sinon:*
        + $C = C union {s}$
        + *pour tout* sommet $t$ voisin du sommet $s$ accessibles:
          + *si* $t in.not C$ *alors:*
            + explorer($t$, $C$, $"durée" + Delta t(s, t)$, $"altitude" - Delta h(s, t)$)
    + *explorer*(décollage, ${}$, $0$, $h("décollage")$)
  ]
  #divider
  Complexité: $O(|S|!)$
]

#pagebreak()

== Algorithme naïf

#place(center + horizon)[
  Peut-on faire mieux ?

  Les versions déterministes de ces deux problèmes sont : \
  Étant donné une durée $T$, existe-t-il un chemin de durée inférieure à $T$ ?

  #h(10pt)

  Ces problèmes sont NP-complets.
]

#pagebreak()

== Approximation

#place(center + horizon)[
  = 4) Approximation des solutions
]

#pagebreak()

== Approximation

#card(
  [
    Critère de domination
  ],
  [
    Soit $(t_1, a_1), (t_2, a_2)$ deux solutions partielles pour un sommet $v$.

    $(t_1, a_1)$ domine $(t_2, a_2)$ et on note $(t_1, a_1) succ.curly.eq (t_2, a_2)$ \
    si $t_1 <= (1 + epsilon) t_2$ et $a_1 > a_2$
  ],
)

\

*Algorithme de djikstra modifié:*
- On autorise l'algorithme à passer par un sommet déjà visité, en ne conservant la nouvelle solution partielle seulement si elle n'est pas dominée par une solution partielle déjà existante.

#pagebreak()

== Approximation


#let approx(body) = {
  pseudocode-list(line-numbering: none)[
    + *approx1*(G, altitudes):
      + solutions = (${(0, 0)}$, ${}$, ..., ${}$)
      + tas = NouveauTasMin((0, altitude[0], 0, 0))
      + *Tant que* $exists (d, h, s, p) in "tas"$:
        + *Si* $s = "arrivée"$:
          + *Renvoyer* $(p, d)$
        + *Pour chaque* $(v, Delta t, Delta h)$ voisin de $s$ accessibles en altitude:
          + d = d + $Delta t$
          + h = h + $Delta h$
          + *Si* $(d, h)$ n'est pas dominé par une solution de solutions$[v]$:
            + #body
            + solutions$[v]$ = solutions$[v]$ $union (d, h)$
            + tas.ajouter($(d, h, v, p + v)$)
  ]
}

#place(dy: -20pt, center + horizon)[
  #approx()[]
  #place(dy: 10pt, divider)
  #place(dy: 25pt, center)[
    // $->$ C'est une $epsilon$-approximation, mais toujours $O(abs(S)!)$
    Toujours $O(abs(S)!)$
  ]
]

#pagebreak()

== Approximation

#grid(
  columns: 100%,
  rows: (60%, 40%),
  place(
    center + horizon,
    card(
      [
        Majoration du nombre de solutions partielles
      ],
      [
        Notons $T_"max"$ le temps de trajet maximum pour atteindre un sommet $s$.

        En ne conservant uniquement les solutions partielles non dominées par une autre, on a $ abs("solutions"[s]) <= log_(1+epsilon)(T_max) $
      ],
    ),
  ), place(
    center + horizon,
    card(
      [
        Majoration du nombre d'opérations sur le tas
      ],
      [
        $
          O(abs(S) abs("solutions"[s])) &= O(abs(S) log_(1+epsilon)(T_max)) \ &= O(abs(S) log(T_"max") / epsilon)
        $
      ],
      color: blue,
    ),
  )
)

#pagebreak()

== Approximation

#place(center + horizon)[
  #box(height: 115%)[
    #set text(size: 24pt)
    #approx()[
      #text(fill: orange)[
        #sym.triangle.r Retirer les solutions dominées par $(d, h)$ dans solutions$[v]$ \
        #h(18pt) et leurs chemins dans le tas]
    ]
  ]
]

#pagebreak()

== Approximation

#place(center + horizon)[
  #let complexity(body) = {
    text(fill: red, body)
  }
  #box(height: 115%)[
    #set text(size: 24pt)
    #approx()[
      #text(fill: orange)[
        #sym.triangle.r Retirer les solutions dominées par $(d, h)$ dans solutions$[v]$ \
        #h(18pt) et leurs chemins dans le tas]
    ]
  ]
  #place(
    dy: -110%,
    dx: 77%,
    complexity()[$ N = abs(S) log(T_"max") / epsilon $],
  )
  #place(
    dy: -80%,
    dx: 40%,
    complexity()[$#scale(x: -100%, sym.arrow.t.curve) O(N)$],
  )
  #place(
    dy: -70%,
    dx: 90%,
    complexity()[$#scale(x: -100%, sym.arrow.b.curve) O(abs(A))$],
  )
  #place(
    dy: -24%,
    dx: 78%,
    complexity()[$#scale(x: -100%, sym.arrow.t.curve) O(N log(N))$],
  )
]

#pagebreak()

== Approximation

#grid(
  rows: (50%, 50%),
  columns: 100%,
  place(
    center + horizon,
    card(
      [
        Complexité
      ],
      [
        $
          O((abs(S) log(T_"max") / epsilon)^2 times abs(A) times
            log(abs(S) log(T_"max") / epsilon))
        $
      ],
      color: blue,
    ),
  ),
  place(
    center + horizon,
    card(
      [
        Qualité de l'approximation
      ],
      [
        La solution en pire cas est $(1+epsilon)^abs(S) #sym.approx 1 + epsilon abs(S)$ plus grande que la solution optimale $->$ erreur de $epsilon abs(S)$
      ],
      color: orange,
    ),
  )
)

#pagebreak()

== Approximation - deuxième contrainte


#place(
  center + horizon,
  [
    #card(
      [
        Discrétisation du temps
      ],
      [
        - On divise la journée en $k$ subdivision régulière.
        - Une solution partielle peut dominer seulement les solutions partielles dans la même subdivision.
        - On a donc au maximun $k$ fois plus de solutions partielles par sommet.
      ],
      color: orange,
    )

    #cetz.canvas({
      import cetz.draw: *

      plot.plot(
        size: (20, 7),
        x-format: plot.formats.multiple-of.with(factor: 1, symbol: "h"),
        x-tick-step: 1,
        y-tick-step: 1,
        y-min: -4.5,
        y-max: 4.5,
        x-min: 7.5,
        x-max: 19.5,
        y-label: $Delta h$,
        x-label: [#move(dx: 35pt, dy: -12pt, [heure])],
        axis-style: "left",
        legend: (16, 8),
        legend-style: (stroke: none),
        {
          plot.add(
            (
              (8, -4),
              (9, -3.5),
              (10, -2.5),
              (11, -1.5),
              (12, 0),
              (13, 1.5),
              (14, 4),
              (15, 4),
              (16, 2),
              (17, 1),
              (18, -1),
              (19, -3),
            ),
            line: "spline",
            style: (stroke: red),
            label: $Delta h "continue"$,
          )
          plot.add(
            (
              (8, -3.75),
              (9, -3),
              (10, -2),
              (11, -0.75),
              (12, 0.75),
              (13, 2.75),
              (14, 4),
              (15, 3),
              (16, 1.5),
              (17, 0),
              (18, -2),
              (19, -2),
            ),
            line: "hv",
            style: (stroke: blue),
            label: $Delta h "discrétisé"$,
          )
        },
      )
    })
  ],
)


== Résultats

#place(center + horizon)[
  Avec suppression des solutions partielles dominées
  #text(
    size: 20pt,
    table(
      columns: 4,
      inset: 10pt,
      align: center + horizon,
      [Nombre de sommets],
      [Solution exacte],
      [$epsilon = 0.1$],
      [$epsilon = 0.0005$],

      [$1000$], [$244$ms], [$4$ms - $10.8%$], [$208$ms - $0%$],
      // [$1000$], [$244$ms - $2520$], [$4$ms - $2794$ - $10.8%$], [$208$ms - $2520$],
      [$10000$], [$2857$s], [$1.49$s - $27.7%$], [$85.3$s - $0.2%$],
      // [$10000$], [$2857$s - $22346$], [$1.49$s - $28547$], [$85.3$s - $22388$],
    ),
  )
  Sans suppression des solutions partielles dominées
  #text(
    size: 20pt,
    table(
      columns: 4,
      inset: 10pt,
      align: center + horizon,
      [Nombre de sommets],
      [Solution exacte],
      [$epsilon = 0.1$],
      [$epsilon = 0.0005$],

      [$1000$ ], [$20$ms], [$7$ms - $9.8%$], [$20$ms - $0%$],
      // [$1000$ ], [$20$ms - $2093$], [$7$ms - $2299$], [$20$ms - $2093$],
      [$10000$ ],
      [$207.7$s],
      [$4.57$s - $7.2%$],
      [$33.9$s - $0.2%$],
      // [$10000$ ],
      // [$207.7$s - $22062$],
      // [$4.57$s - $23665$],
      // [$33.9$s - $22094$],
    ),
  )
]

#pagebreak()

== Conclusion

#place(center + horizon)[
  #set align(left)
  // - Solution exacte trop longue à calculer
  // - Solution approchée plus rapide mais \
  // avec une grosse marge d'erreur

  - Pistes de reflexions:
    - Heuristique $A^*$
]

#pagebreak()

== Annexe - preuve NP-complétude

Réduction depuis PARTITION: \
Soit $E subset NN$ un ensemble, existe-t-il $S subset E$ tel que $ sum_(i in S) i = sum_(i in.not S) i $

#pagebreak()

== Annexe - preuve NP-complétude

Contrainte d'altitude avec $E = {1, 2, 3}, S= sum_(i in E) i$, \

#place(
  center + horizon,
  diagram(
    node-stroke: 2pt,
    node((0, 1), $h=S / 2$, radius: 40pt),
    node((1, 0), radius: 20pt),
    node((1, 2), radius: 20pt),
    node((2, 0), radius: 20pt),
    node((2, 2), radius: 20pt),
    node((3, 0), radius: 20pt),
    node((3, 2), radius: 20pt),
    node((4, 0), radius: 20pt),
    node((4, 2), radius: 20pt),
    node((5, 1), $h=0$, radius: 40pt),

    arrow((1, 0), (1, 2), bend: 10deg),
    arrow((1, 2), (1, 0), bend: 10deg),
    arrow((2, 0), (2, 2), bend: 10deg),
    arrow((2, 2), (2, 0), bend: 10deg),
    arrow((3, 0), (3, 2), bend: 10deg, stroke: red),
    arrow((3, 2), (3, 0), bend: 10deg),
    arrow((4, 0), (4, 2), bend: 10deg),
    arrow((4, 2), (4, 0), bend: 10deg),

    arrow((1, 0), (2, 0), label: (1, 0), stroke: red),
    arrow((2, 0), (3, 0), label: (2, 0), stroke: red),
    arrow((3, 0), (4, 0), label: (3, 0)),
    arrow((1, 2), (2, 2), label: (0, 1)),
    arrow((2, 2), (3, 2), label: (0, 2)),
    arrow((3, 2), (4, 2), label: (0, 3), stroke: red),

    arrow((0, 1), (1, 0), stroke: red),
    arrow((0, 1), (1, 2)),
    arrow((4, 0), (5, 1)),
    arrow((4, 2), (5, 1), stroke: red),
  ),
)

#pagebreak()

== Annexe - preuve approximation

Soit $P_"opt" = ("départ", ..., u, v, ..., "arrivée")$ un chemin optimal.

$forall s in P_"opt", exists (t, a) in$ solutions$[s]$, $t <= t_"opt" (1+epsilon)^k$ et $a >= a_"opt"$,

_Initialisation_: Le sommet de départ est unique \
_Hérédité_: Supposons la propriété vraie pour $u$

$exists (t_u, a_u) in$ solutions$[u]$ tel que $t_u <= t_(u,"opt") (1+epsilon)^k$ et $a_u >= a_(u,"opt")$


$t_v = t_u + Delta t(u, v) &<= (1 + epsilon)^k t_(u,"opt") + Delta t(u, v) \ &<= (1+epsilon)^k (t_(u,"opt") + Delta t(u, v)) \
&<= (1+epsilon)^k t_(v,"opt")$

$a_v = a_u + Delta h(u, v) >= a_(u,"opt") + Delta h(u, v) >= a_(v,"opt")$

#pagebreak()
== Annexe - preuve approximation

Si $(t_v, a_v)$ est dominé dans solutions$[v]$:

$exists (t_"exist", a_"exist")$ tel que $t_"exist" <= (1+epsilon) t_v$ et $a_"exist" > a_v$

Donc $t_"exist" <= (1 + epsilon)^(k+1) t_(v,"opt")$ et $a_"exist" >= a_(v,"opt")$
#pagebreak()

== Annexe - preuve complexité

#place(center + horizon)[
  $
    RR^+ = [0, 1 [ union [(1+epsilon)^0, (1+epsilon)^(1)[ union [(1+epsilon)^1, (1+epsilon)^(2)[ union dots.h.c
  $

  \

  Soient $(t_1, a_1)$ et $(t_2, a_2)$ tels que $t_1, t_2 in [(1+epsilon)^k, (1 +epsilon)^(k+1)[$ et $a_1 >= a_2$

  Alors $t_2 (1 + epsilon) >= (1+epsilon)^(k+1) >= t_1$

  \

  Donc $(t_1, a_1) succ.curly.eq (t_2, a_2)$

  Donc $abs("solutions"[s]) = O(log_(1 + epsilon)(T_"max")) = O(log(T_"max") / epsilon)$
]


