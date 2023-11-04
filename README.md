# Problem de la Pyramide des Dames

> A défaut de trouver un meilleur nom

La problématique est de résoudre une pyramide de base 7 où chaque point peut contenir une pièce ou être vide. Le but du jeux est de finir avec une seule pièce sur la grille. La grille commence remplie de pièce et on commence la partie en retirant une pièce de son choix. Il faut ensuite réaliser des coups en "sautant" une pièce par dessus une autre à condition que l'espace derrière la pièce sautée est libre, la pièce sautée est retirée de la partie (le mode de capture est comme le jeux des dames). Il n'est pas possible de sauter une pièce et attérir hors de la grille.

! Dans un 1er temps la règle n'autorise pas les prises en diagonale !

Exemple de grille de départ :

```txt
      1
    1 1 1
  1 1 1 1 1
1 1 1 1 1 1 1
```

Exemple d'un coup :

```txt
      1                     1
    1 1 1       -->       1 1 1
  1 1 1 1 1             1 1 1 1 1
1 1 1 1 0 1 1         1 1 1 1 1 0 0 
```

## Objectifs

- Problèmes
  - [ ] Déterminer si le jeux est resolvable ou pas dans un version 4 par 7 sans coup en diagonale
  - [ ] Si pas solvable, avec les coups en diagonale
- Features techniques
  - [x] Afficher une grille du jeux
  - 