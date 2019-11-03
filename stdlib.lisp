(def True true)
(def False false)
(def head (lambda (l) (index 0 l)))
(def tail (lambda (l) (drop 1 l)))
(def empty (lambda (l) (= 0 (len l))))
(def add (lambda (x y) (+ x y)))
(def sub (lambda (x y) (- x y)))
(def mul (lambda (x y) (* x y)))
(def div (lambda (x y) (/ x y)))
(def mod (lambda (x y) (% x y)))

(def or
  (lambda (x y)
    (if (quote x)
      (True)
      (if (quote y)
        (True)
        (False)))))

(def and
  (lambda (x y)
    (if (x)
      (if (y)
        (True)
        (False))
      (False))))

(def not
  (lambda (x)
    (if (x)
      (False)
      (True))))

(def rec-filter
     (lambda (f l acc)
       (if (empty l)
       (acc)
       (if (f (head l))
         (rec-filter f (tail l) (append (head l) acc))
         (rec-filter f (tail l) acc)))))

(def filter (lambda (f l) (rec-filter f l (list))))

(def rec-range
  (lambda (begin end l)
    (if (< begin end)
      (rec-range (+ 1 begin) end (append begin l))
      l)))

(def range
  (lambda (begin end)
    (rec-range begin end (list))))

(def reduce
  (lambda (f acc l)
    (if (empty l)
      acc
      (reduce f (f acc (head l)) (tail l)))))

(def sum
  (lambda (l)
    (reduce add 0 l)))

(def product
  (lambda (l)
    (reduce mul 1 l)))

(def rec-zip
  (lambda (l1 l2 acc)
    (if (or (empty l1) (empty l2))
      acc
      (rec-zip (tail l1) (tail l2) (append (list (head l1) (head l2)) acc)))))

(def zip
  (lambda (l1 l2)
    (rec-zip l1 l2 (list))))

(def collatz
  (lambda (x)
    (if (= 1 x)
      (list 1)
      (if (= 0 (mod x 2))
        (append x (collatz (/ x 2)))
        (append x (collatz (+ 1 (* 3 x))))))))

(def rec-fibonacci
  (lambda (max_n i acc_l)
    (if (= max_n i)
      acc_l
      (rec-fibonacci max_n (+ i 1) (append (sum (last_n 2 acc_l)) acc_l)))))

(def fibonacci
  (lambda (n)
    (rec-fibonacci n 0 (list 1))))

(def rec-isprime
  (lambda (x acc max_num)
    (if (>= acc max_num)
      True
      (if (= 0 (mod x acc))
        False
        (rec-isprime x (+ 1 acc) max_num)))))

(def isprime
  (lambda (x)
    (if (<= x 1)
      False
      (rec-isprime x 2 (+ 1 (sqrt x))))))

(def rec-map
  (lambda (f l a)
    (if (empty l)
      a
      (rec-map f (tail l) (append (f (head l)) a)))))

(def map
  (lambda (f l)
    (rec-map f l (list))))
