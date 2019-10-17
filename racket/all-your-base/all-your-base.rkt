#lang racket

(provide rebase)

(define (list-to-num list-digits in-base)
  (foldl (lambda (elem total)
           (+ (* total in-base) elem))
         0
         list-digits))

(define (num-to-list num out-base)
  (if (= num 0)
      `()
      (cons (remainder num out-base) (num-to-list (quotient num out-base) out-base))))

(define (rebase list-digits in-base out-base)
  (if (or (<= in-base 0) (<= out-base 0))
      #f
      (reverse (num-to-list (list-to-num list-digits in-base) out-base))))