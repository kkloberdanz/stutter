(let (f (lambda (x) (+ 1 x))) (f 2))

(let (f (lambda (x y) (+ x y))) (f 4 5))

(def head (lambda (l) (index 0 l)))

(def tail (lambda (l) (drop 1 l)))
