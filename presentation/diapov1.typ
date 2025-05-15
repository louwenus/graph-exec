#import "@preview/fletcher:0.5.7" as fletcher: diagram, node, edge
#import "config.typ": *
#show: template



#place(dy: 50pt, center + top)[
    = Paralélisme : Tomasulo, du hardware au thread
]

#place(
  dy: 0pt,
  center + bottom,
  box(
    height: 70%,
    figure(
      box(
        radius: 5pt,
        clip: true,
        image("assets/die_shot.jpg"),
      ),
      caption: text(size: 18pt)[Die shot of an Intel Mobile Pentium II (Dixon)],
    ),
  ),
)



#pagebreak()

== Somaire

#place(center + horizon)[
  #set align(left)
  + Petite histoire du parallélisme
  + Un peu de théorie
  + L'algorithme de Tomasulo
  + Tentative de portage
]

#pagebreak()

== Histoire

= 1) Les évolution du parallélisme en informatique


#pagebreak()

== Superscalaire: Détéction implicite

In order: Le CDC6600 (1965), Le pentium (1993)

Out of order: Pentium Pro (1995)

#pagebreak()

== Parallélisme logiciel: Explicite

SIMD : ILLIAC IV (1972), MMX (1996)

Multi-coeur : POWER4 (2001)


#pagebreak()

== Théorie

= 2) La difficile théorie

#pagebreak()

== Indécidabilité

=== Réduction a la terminaison de équivalence

On dispose de:

- `emule_termine(f,n)` : f termine en moins de n étapes
- `équiv(f,g)` : f et g sont ils équivalents sur tout entrée

#pseudocode-list(line-numbering: none)[
+ termine(f):
  + g:= n -> emule_termine(while True,n)
  + h:= n -> emule_termine(f,n)
  + non équiv(g,h)
]

#pagebreak()
== Un modèle tout mignon
#align(center+horizon,grid(
  columns: (50%,50%),
  diagram(spacing: 4em,
    node((0,-1),"Thread 1"),  edge("d","-|>"),
    node((1,-1),"Thread 2"), edge("d","-|>"),
    node((0,0),`W`),edge("d","-|>"), node((1,0),`W`),edge("d","-|>"),
    node((0,1),`R`),edge("d","-|>"),edge("ur","<|--"),
    node((1,1),`R`),edge("d","-|>"),edge("ul","<|--"),
  ),[]
))

#pagebreak()
== Un modèle tout mignon
#align(center+horizon,grid(
  columns: (50%,50%),
  diagram(spacing: 4em,
    node((0,-1),"Thread 1"),  edge("d","-|>"),
    node((1,-1),"Thread 2"), edge("d","-|>"),
    node((0,0),`W`),edge("d","-|>"), node((1,0),`W`),edge("d","-|>"),
    node((0,1),`R`),edge("d","-|>"),edge("ur","<|--"),
    node((1,1),`R`),edge("d","-|>"),edge("ul","<|--"),
  ),
  figure(
      box(
        radius: 5pt,
        clip: true,
        image("assets/intertwinned_green_lines.png"),
      ),
      caption: text(size: 18pt)[What it realy look likes],
    ),
  
  
))
#pagebreak()

== Incohérence du model mémoire

#place(horizon+center)[
#set align(left)

#grid(columns: (auto,30pt,auto),
[
#diagram(
  spacing: 4em,
  node((0,-1),"Thread 1"),  edge("d","-|>"),
  node((1,-1),"Thread 2"), edge("d","-|>"),
  node((0,0),`R`),edge("d","-|>"), node((1,0),`R`),edge("d","-|>"),
  node((0,1),`W`),edge("d","-|>"),edge("ur","--|>"),
  node((1,1),`W`),edge("d","-|>"),edge("ul","--|>"),
  
  
)],
[],
[#figure(
      box(
        radius: 5pt,
        clip: true,
        image("assets/ghost.jpg"),
      ),
      caption: text(size: 18pt)[],
    )],
  )

]

#pagebreak()
== Tomasulo
= 3) Tomasulo

#pagebreak()
== Tomasulo
#align(center + horizon)[
#diagram(
  node("Instruction decoder"), edge("dr",stroke:blue,"-|>"),
  node((1,1),"logical register"),edge("d","--|>"),edge("l",stroke:blue,"-|>"),
  node((0,1),"Reservation Station"),edge("<|-"),edge("d",stroke:blue,"-|>"),
      node((1,2),"Real registers"),
  node((0,2),"Execution unit"),edge("r","-|>")
)]

#pagebreak()
== Gestion des risques

- RAR: pas un risque
- WAW et RAW: register renaming
- WAR: Attente dans la station

#pagebreak()

== Et avec tout ça

= 4) Tentative de portage au multithreading

#pagebreak()

== Un marché du travail

Comment trouver des execution unit ?

File de tache partagées!

\
Mais ... comment faire une file de tache partagées?

#pagebreak()

= Annexes

TODO!!!
