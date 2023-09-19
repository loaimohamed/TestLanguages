(print "What's your name")

(defvar *name* (read))

(defun hello-you (*name*)
    (format t "Hello ~a! ~%" *name*)
)

(setq *PRINT-CASE* :capitalize)

(hello-you *name*)
