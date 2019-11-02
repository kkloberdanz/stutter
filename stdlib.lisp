(def head (lambda (l) (index 0 l)))
(def tail (lambda (l) (drop 1 l)))
(def empty (lambda (l) (= 0 (len l))))

(def rec-filter
     (lambda (f l acc)
       (if (empty l)
       (acc)
       (if (f (head l))
         (rec-filter f (tail l) (append (head l) acc))
         (rec-filter f (tail l) acc)))))

(def filter (lambda (f l) (rec-filter f l (list))))
