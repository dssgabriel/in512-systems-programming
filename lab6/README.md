# Optimization de la consommation : Multiplication de matrices

Nous allons travailler sur la multiplication de matrices denses.
C’est un algorithme classique qui est utilisé dans de nombreux domaines comme
l’apprentissage automatique, l’analyse de données, les moteurs 3D, les
simulations physiques, etc.

Ici nous travaillerons avec des matrices carrées de taille `n x n`.
Le produit de matrices `C = A x B` est défini par l'équation :
```
           n
	c_ij = Σ (a_ik * b_kj)
	      k=1
```
Avec `i` l’indice de ligne et `j` l’indice de colonne de la matrice `C`.

## Représentation des matrices en Rust

Pour représenter les matrices en Rust nous allons utiliser la structure
suivante :
```rs
type Element = f64;

#[derive(Debug, PartialEq)]
pub struct Matrix {
    n: usize,
    values: Vec<Element>,
}
```

- `Element` est le type des valeurs à l’intérieur de la matrice, ici des
nombres à virgule flottante stockés sur 64 bits (l’équivalent des
`double` en C).
- `n` représente la taille d’un côté de la matrice.
- `values` est un vecteur qui contient les éléments de la matrice.

Implémenter pour la structure `Matrix` le constructeur 
`fn new(n: usize, values: Vec<Element>) -> Self`.
Attention à bien vérifier grace à une assertion que le vecteur `values`
contient bien `n²` valeurs.

Conseil : Ne pas hésiter à implémenter des tests automatiques pour
valider l'implémentation de chaque méthode tout au long du TP.

## Implémentation des traits Index et IndexMut

Une matrice est une structure à deux dimensions, mais on va la
stocker sur le vecteur `values` à une seule dimension. Il y a plusieurs
ordres de stockage possibles. Ici, on va stocker les matrices dans
l’ordre des lignes (_row major order_).

Nous souhaitons cependant pouvoir accéder aux éléments en utilisant les
indices ligne `i` et colonne `j`.

Implémenter les traits `std::ops::Index<(usize, usize)>` et
`std::ops::IndexMut<(usize, usize)>` pour la structure `Matrix` de
manière à ce que l’on puisse accéder aux éléments de la matrice avec la
syntaxe suivante :
```rs
#[test]
fn indexes() {
    let mut m = Matrix::new(2, vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(m[(0, 0)], 1.0);
    assert_eq!(m[(1, 0)], 3.0);

    m[(1,0)] = 5.0;
    assert_eq!(m[(1, 0)], 5.0);
}
```

## Génération de matrices remarquables

1. Implémenter un deuxième constructeur `fn identity(n:usize) -> Self` qui
   retourne la matrice identité de dimension `n`.
2. Implémenter un troisième constructeur `fn random(n:usize) -> Self` qui
   retourne une matrice de dimension `n` dont les éléments sont tirés
   aléatoirement d’une distribution uniforme sur l'intervalle `[-1, 1]`.

## Installation de l'outil perf

Dans la suite du TP nous allons utiliser l'outil `perf` pour faire diverses
mesurer de performance. Pour installer l'outil `perf` sous Ubuntu ou Debian
vous pouvez exécuter les commandes suivantes :
```
sudo apt-get install --reinstall linux-tools-common linux-tools-generic linux-tools-`uname -r`
echo "-1" | sudo tee /proc/sys/kernel/perf_event_paranoid
```

Attention : Sur certaines machines il n'est pas possible d'installer l'outil
`perf`. Le cas échéant, utilisez la commande `time` et au lieu de mesurer
l'énergie consommée / le nombre de cache-misses, vous mesurerez le temps
d'exécution.

## Multiplication matricielle naïve

1. Implémenter une méthode `fn multiply(a: &Matrix, b: &Matrix) -> Matrix`
   qui retourne le résultat de la multiplication matricielle de `a` par `b`.
   Écriver un code simple sans essayer à ce stade d’optimiser la fonction.
   Rajouter un ou plusieurs tests pour s'assurer que le résultat est correct.

2. Modifiez le fichier `src/main.rs` de manière à pouvoir réaliser le produit
   de deux matrices aléatoires dont la taille est passée en ligne de commande.

3. Utilisez l’outil `perf` pour estimer grâce aux compteurs RAPL la
   consommation énérgétique en Joules de votre programme pour une matrice de
   dimension 256x256.
   ```
   cargo build
   perf stat -e power/energy-pkg/,power/energy-ram/ ./target/debug/matrixmult 256
   ```
   Conseil : Lorsque vous faites des mesures, assurez vous que les valeurs
   mesurées sont significatives et reproductibles. Pour cela vous pouvez
   répéter la mesure plusieurs fois et vérifier que l'écart type est petit par
   rapport à la valeur mesurée.

4. Compiler le programme en utilisant les optimisations du compilateur
   et mesurer l’énergie consommée à nouveau.
   ```
   cargo build --release
   perf stat -e power/energy-pkg/,power/energy-ram/ ./target/debug/matrixmult naive 256
   ```
   Qu’observe-t-on ? Comment peut-on l’expliquer ?

5. Testez la version optimisée avec des dimensions plus grandes :
   512, 768, 1024, 1280, 1536, 1792.
   Tracez la courbe de temps d’exécution et de consommation en fonction de `n`.

6. Quelles sont les caractéristiques de votre machine ? Utiliser les outils
   suivants :
    - `lstopo` pour examiner la hiérarchie mémoire.
    - `cat /proc/cpuinfo` ou `lscpu` pour avoir les caractéristiques du
      processeur.

## Cache blocking

Comme nous avons vu en cours, la multiplication de matrices peut présenter
des problèmes d'accès au cache et à la mémoire. En effet, l'accès à la deuxième
matrice n'est pas dans l'ordre des lignes mais dans l'ordre des colonnes.
Les éléments d'une colonne ne sont donc pas contigus en mémoire.

Pour vérifier si notre implémentation souffre de problèmes de localité mémoire,
on va à nouveau utiliser l'outil `perf`.

1. Mesurons tout d'abord le nombre d'accès mémoire au dernier niveau de cache
   pour la version naïve avec la commande suivante:
   ```
   perf stat -e LLC-loads,LLC-stores ./target/release/matrixmult 1280
   ```
   - LLC-loads mesure le nombre de lectures depuis le dernier niveau de cache.
   - LLC-stores mesure le nombre d'écritures sur le dernier niveau de cache.

2. Implémenter maintenant une version bloquée de la multiplication matricielle
   `fn multiply_blocked(a: &Matrix, b: &Matrix) -> Matrix`.
   Définissez une constante `BLOCK` dans la classe `Matrix` pour stocker la
   taille du bloc (on peut la fixer à 64).
   Conseil : Vérifier bien que la dimension de vos matrices est un multiple de
   la taille du bloc.

3. Mesurer le nombre d'accès mémoires au dernier niveau de cache (L3) pour
   la version bloquée.

4. Mesurer la consommation énergétique et le temps d'exécution pour la
   version bloquée et comparer aux mesures de la version naïve ?

5. Que peut-on conclure ? Pourquoi la version bloquée se révèle plus efficace ?

## Parallélisation

Si notre processeur possède plusieurs cœurs de calcul, nous pouvons paralléliser
l'algorithme de manière à le rendre encore plus efficace.
Pour cela nous allons nous appuyer sur la bibliothèque (https://docs.rs/rayon/1.5.1/rayon/)[Rayon].

1. Implémenter une version parallèle du produit matriciel,
   `fn multiply_rayon(a: &Matrix, b: &Matrix) -> Matrix` :
   - On conseille de paralléliser en découpant selon les lignes de `a` et `c`
   (la matrice résultat).
   - `rayon` ajoute les méthodes `par_chunks(&self, chunk_size: usize)` et
   `par_chunks_mut(&mut self, chunk_size: usize)` aux itérateurs. Ces deux
   méthodes retournent des itérateurs *parallèles*. Les opérations sur les
   itérateurs seront distribuées sur différents threads. Vous pouvez donc
   utiliser ces deux méthodes pour répartir un calcul sur les lignes de `c`
   et les lignes de `a`.
   - Pour itérer de manière synchronisé sur les lignes des deux itérateurs
   vous pouvez utiliser (https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)[la méthode `zip`].

2. Mesurer l'énergie consommée par l'implémentation parallèle. Comparer les
   mesures aux expériences précédentes. Que peut-on conclure ?

3. Rajouter du cache-blocking dans l'implémentation parallèle et
   mesurer l'effet sur la consommation.

## Pour aller plus loin

Le produit de matrices que nous avons implémenté est efficace mais il est
possible de pousser les optimisations encore plus loin. Voici quelques
références et pistes, si ce travail vous intéresse :
- Le compilateur actuel Rust ne réussit pas à vectoriser correctement le
  produit de matrices. Néanmoins il est possible d'utiliser des
  (https://doc.rust-lang.org/beta/core/arch/)[appels intrinsèques] pour
  vectoriser manuellement et tirer parti des instructions SIMD du processeur.

Conseil : La vectorisation automatique est un des points faibles de Rust. En
raison de vérifications plus poussées, comme les débordement de tableaux, Rust
n'arrive pas toujours à bien vectoriser une boucle. Au contraire des langages
comme le C ou le Fortran, offrent moins de garanties sur la correction mémoire,
mais vectorisent généralement mieux le code.

- Plutôt qu'utiliser des techniques de blocking, qui doivent être paramétrisées
par une taille de bloc fixe; il est possible d'implementer la multiplication
matricielle pour préserver la localité indépendamment de l'échelle.
C'est ce qu'on appelle en anglais un (https://dspace.mit.edu/bitstream/handle/1721.1/80568/43558192-MIT.pdf)[algorithme _cache-oblivious_].
Pour la multiplication de matrices, un tel algorithme peut être obtenu
en réordonnant les éléments selon l'ordre donné par la (https://fr.wikipedia.org/wiki/Courbe_de_Lebesgue)[courbe de Lebesgue].
Cela permet d'obtenir des (https://github.com/rayon-rs/rayon/blob/master/rayon-demo/src/matmul/mod.rs)[implémentations très efficaces]
pour des matrices dont la dimension est une puissance de deux.

- La parallélisation que nous vous avons proposé se révèle très efficace.
Néanmoins il est possible d'aller encore plus vite en utilisant une
décomposition en blocs et la multiplication proposée par (https://fr.wikipedia.org/wiki/Algorithme_de_Strassen)[Strassen].

## Crédits

- Illustation d’ordre des lignes adapté de l’image de
(https://commons.wikimedia.org/wiki/File:Row_and_column_major_order.svg)[Cmglee],
en CC BY-SA 4.0.
