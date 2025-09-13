(module
  (import "builtin" "move" (func $move))
  
  (func $main
    ;; an example loop that will iterate through the integers from 1 to 5
    (local $i i32)  ;; the looping variable
    (local.set $i (i32.const 0))  ;; initialize i to 0

    (loop $_loop
        ;; Will add one to i
        local.get $i
        local.const 1
        i32.add  ;; i+1 will be on the stack
        local.set $i    ;; pop this stack value and set i to it

        i32.const 5
        i32.lt_s  ;; will check if i is less than 5
        br_if $my_loop ;; if the statement is true then the execution will return to the start of the loop
    )
  )
  (start $main)
)
