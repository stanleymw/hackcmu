(module
    ;;(import func move)   @@@@@@@@@@@@@@FIX
    ;;(import func turn_right)   @@@@@@@@@@@@@@FIX

    (func $move_fn
        call move
    )

    (func $turnRight_fn
    call turn_right
    )

    (func $turnLeft_fn
    call turn_right
    call turn_right
    call turn_right
    )

    (func moveNTimes (param $n i32)
        (local $i i32)  ;; the looping variable
        (local.set $i (i32.const 0))  ;; initialize i to 0

        (loop $_loop
            call move_fn

            ;; Will add one to i
            local.get $i
            local.const 1
            i32.add  ;; i+1 will be on the stack
            local.set $i    ;; pop this stack value and set i to it

            (local.get $n)   ;; load parameter
            i32.lt_s  
            br_if $my_loop ;; if the statement is true then the execution will return to the start of the loop
        )
    )

    (func $main
        (local $i i32)  ;; the looping variable
        (local.set $i (i32.const 32))  ;; initialize i to 0

        (loop $_loop
            (local.get $i)
            call moveNTimes  ;; consume
            call turnLeft_fn

            ;; Will add one to i
            local.get $i
            local.const 1
            i32.sub  ;; i-1 will be on the stack
            local.set $i    ;; pop this stack value and set i to it

            i32.const 0
            i32.gt_s  
            br_if $my_loop ;; if the statement is true then the execution will return to the start of the loop
        )
    )
    
    (start $main)
)