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
            local.const 1
            i32.add  ;; i+1 will be on the stack
            local.set $i    ;; pop this stack value and set i to it

            (local.get $n)   ;; load parameter
            i32.lt_s  
            br_if $_loop ;; if the statement is true then the execution will return to the start of the loop
        )
    )

    (func $main
        ;; Take shortcuts; value = 1520
        i32.const 1520
        $moveNTimes  ;; will consume the value 255 from the stack
    )
    
    (start $main)
)