(module
   (import "builtin" "move" (func $move))
  (import "builtin" "turn_right" (func $turn_right))

  (func $move_fn
    call $move
  )

  (func $turnRight_fn
    call $turn_right
  )

    (func $turnLeft_fn
    call $turn_right
    call $turn_right
    call $turn_right
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
        (local $i i32)  ;; the looping variable
        (local.set $i (i32.const 8))  ;; initialize i to 0

        (loop $_loop
            (local.get $i)
            call $moveNTimes  ;; consume
            call $turnLeft_fn

            ;; Will add one to i
            local.get $i
            i32.const 1
            i32.sub  ;; i-1 will be on the stack
            local.set $i    ;; pop this stack value and set i to it
			local.get $i
            i32.const 0
            i32.gt_s  
            br_if $_loop ;; if the statement is true then the execution will return to the start of the loop
        )
    )
    
    (start $main)
)

