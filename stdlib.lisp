(let (f (lambda (x) (+ 1 x))) (f 2))

(let (f (lambda (x y) (+ x y))) (f 4 5))


(let (l (list 1 2 3)) (+ 1 2))

(let (head (lambda (l) (index 0 l))) (head (list 1 2 3 4 5)))
(let (tail (lambda (l) (drop 1 l))) (tail (list 1 2 3 4 5)))
