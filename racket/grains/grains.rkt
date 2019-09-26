#lang racket

(provide square total)

(define (square n)
  (arithmetic-shift 1 (- n 1)))

(define (total)
  (- (square 65) 1))
