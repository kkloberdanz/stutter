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
(def sqrt (lambda (x) (pow x 0.5)))
(def last_n (lambda (n l) (drop (- (len l) n) l)))
(def even (lambda (x) (= 0 (mod x 2))))
(def odd (lambda (x) (= 1 (mod x 2))))
(def last (lambda (l) (index (- (len l) 1) l)))

(def or
  (lambda (x y)
    (if (x)
      (True)
      (if (y)
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

(def filter
  (lambda (f l)
    (let (rec-filter (lambda (f l acc)
      (if (empty l)
        acc
        (let (x (head l)) (xs (tail l))
          (if (f x)
            (rec-filter f xs (append x acc))
            (rec-filter f xs acc))))))
      (rec-filter f l (list)))))

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

(def zip
  (lambda (l1 l2)
    (let (rec-zip (lambda (l1 l2 acc)
      (if (or (empty l1) (empty l2))
        acc
        (rec-zip
          (tail l1)
          (tail l2)
          (append (list (head l1) (head l2)) acc)))))
      (rec-zip l1 l2 (list)))))

(def collatz
  (lambda (x)
    (if (= 1 x)
      (list 1)
      (if (= 0 (mod x 2))
        (cat (list x) (collatz (/ x 2)))
        (cat (list x) (collatz (+ 1 (* 3 x))))))))

(def fibonacci
  (lambda (n)
    (let (rec-fibonacci
      (lambda (acc)
        (if (< n (len acc))
          (tail acc)
          (if (> (len acc) 2)
            (rec-fibonacci (append (sum (last_n 2 acc)) acc))
            (rec-fibonacci (append 1 acc))))))
      (rec-fibonacci (list)))))

(def isprime
  (lambda (x)
    (if (<= x 1)
      False
      (let (rec-isprime (lambda (x acc max_num)
        (if (>= acc max_num)
          True
          (if (= 0 (mod x acc))
            False
            (rec-isprime x (+ 1 acc) max_num)))))
        (rec-isprime x 2 (+ 1 (sqrt x)))))))

(def map
  (lambda (f l)
    (let (rec-map (lambda (f l a)
      (if (empty l)
        a
        (rec-map f (tail l) (append (f (head l)) a)))))
    (rec-map f l (list)))))

(def quicksort
  (lambda (mylist)
    (if (empty mylist)
      (list)
      (cat (quicksort (filter (lambda (x) (< x (head mylist))) (tail mylist)))
           (list (head mylist))
           (quicksort (filter (lambda (x) (>= x (head mylist)))
                              (tail mylist)))))))

(def f
  (lambda (x)
    (let (y (head (list 3))) (z (add 5 6))
      (let (w 8)
        (+ w x y z)))))
