(module
    (import "builtin" "move" (func $move))

    (func $move_fn
        call $move
    )

    (func $moveNTimes (param $n i32)
        (local $i i32)  ;; the looping variable
        (local.set $i (i32.const 0))  ;; initialize i to 0

        (loop $_loop
            call $move_fn
            ;; Will add one to i
            local.get $i
            i32.const 1
            i32.add  ;; i+1 will be on the stack
            local.set $i    ;; pop this stack value and set i to it

            local.get $i
            local.get $n
            i32.lt_s  ;; will check if i is less than n
            br_if $_loop ;; if the statement is true then the execution will return to the start of the loop
        )
    )

    (func $main
        i32.const 16
        call $moveNTimes  ;; will consume the value 16 from the stack
    )
    
    (start $main)
)
