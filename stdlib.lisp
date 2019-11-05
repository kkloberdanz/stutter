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
      (acc)
        (if (f (head l))
          (rec-filter f (tail l) (append (head l) acc))
          (rec-filter f (tail l) acc)))))
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
        (append x (collatz (/ x 2)))
        (append x (collatz (+ 1 (* 3 x))))))))

(def fibonacci
  (lambda (n)
    (let (rec-fibonacci
      (lambda (max-n counter acc)
        (if (= max-n counter)
          acc
          (rec-fibonacci
            max-n
            (+ 1 counter)
            (append (sum (last_n 2 acc)) acc)))))
      (rec-fibonacci n 0 (list 1)))))

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
