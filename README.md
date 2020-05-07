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
(def deriv
  (lambda (f x)
    (/
      (- (f (+ x 0.001))
          (f x))
      0.001)))

λ (deriv (lambda (x) (pow x 3.0)) 5.0) ;;; derivative of x^3 where x = 5
75.01500100002545
```

Stutter represents integers with arbetrary precision, so you can do HUGE calculations
```lisp
λ (* (pow 2 256) 115792089237316195423570985008687907853269984665640564039457584007913129639936 (pow 2 3000))
16494713321620350741993496885467939816357268231600417049851139500495161700422753138745619312734532899769399639859564344346707703038337765362028352463050892142817376642659631769406531981236939492922882898922691027668705220270672142169779584404709380210195240739141315707098903742871343194605777737106680060131995521390233213277144803525131645301526466134890948461800583723667846148527655028861109487200549103456486611729021614446599534482430167479941690445523700547433413601218424532337021429721207722454661962332775900337804264855201393785854635389404176669028291992269607299897818566277692086873782286899826226443701644992485084075468415071380507509262644345189285869872937801552926941885902689729628391307391790210311786929910974684343210128412710635415831843815750515037182596327876404272565225965818062628068300670548129897297723878575734275659663231693703806350392112861843199104523186400803827857928433252865895146539480888443753397647798080253830660596227252772158407408106372437086277027124153680304956748163818772598046707205583149203650029442564096
λ
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
