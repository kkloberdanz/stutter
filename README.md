# Stutter
Implementation for the Stutter Programming Language

Stutter is a functional, lisp-like language

An example of Stutter syntax can be seen below with the standard library implementation of quicksort
```lisp
(def quicksort
  (lambda (mylist)
    (if (empty mylist)
      (list)
      (cat (quicksort (filter (lambda (x) (< x (head mylist))) (tail mylist)))
           (list (head mylist))
           (quicksort (filter (lambda (x) (>= x (head mylist)))
                              (tail mylist)))))))

λ (quicksort (list 2 3 5 4 3 2 1 100 -1))
(-1 1 2 2 3 3 4 5 100)

```

Or below is how one could implement the collatz conjecture (https://en.wikipedia.org/wiki/Collatz_conjecture)
```lisp
(def collatz
  (lambda (x)
    (if (= 1 x)
      (list 1)
      (if (= 0 (mod x 2))
        (cat (list x) (collatz (/ x 2)))
        (cat (list x) (collatz (+ 1 (* 3 x))))))))

λ (collatz 100)
(100 50 25 76 38 19 58 29 88 44 22 11 34 17 52 26 13 40 20 10 5 16 8 4 2 1)
```

One can even to differential calculus with Stutter
```lisp
(def differential
  (lambda (f x)
    (/
      (- (f (+ x 0.001))
          (f x))
      0.001)))

λ (differential (lambda (x) (pow x 3.0)) 5.0)
75.01500100002545
```

Stutter is interactive, and includes a REPL for experimentation
```lisp
λ (filter isprime (range 0 15))
(3 5 7 11 13)
```

With Stutter being a functional language, it is well suited for solving mathematical problems, such as those found on https://projecteuler.net/
```lisp
λ (sum (filter even (filter (lambda (x) (< x 4000000)) (fibonacci 1000))))
4613732
```
