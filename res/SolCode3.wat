(module
  (import "builtin" "move" (func $move))
  (import "builtin" "turn_right" (func $turn_right))

  (func $move_fn
    call move
  )

  (func $turnRight_fn
    call $turn_right
  )

  (func $turnLeft_fn
    call $turn_right
    call $turn_right
    call $turn_right
  )

  (func $main
    call $move_fn
    call $move_fn
    call $move_fn
    call $move_fn
    call $move_fn
    call $move_fn
    call $turnLeft_fn
    call $move_fn
    call $move_fn
    call $move_fn
    call $move_fn
    call $turnRight_fn
    call $move_fn
    call $move_fn
    call $move_fn
    call $turnRight_fn
    call $move_fn
    ;; Robot starts reversing direction and will proceed towards the goal
    call $turnRight_fn
    call $turnRight_fn
    call $move_fn
    call $turnLeft_fn
    call $move_fn
    call $move_fn
    call $move_fn
    call $turnRight_fn
    call $move_fn
    call $move_fn
    call $move_fn
    call $turn_left
    call $move_fn
    call $move_fn
    call $move_fn
    call $move_fn
    call $move_fn
    call $move_fn
    call $move_fn
  )
 
  (start $main)
)