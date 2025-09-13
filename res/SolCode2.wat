(module
  (import "builtin" "move" (func $move))
  (import "builtin" "turn_right" (func $turn_right))

  (func $move_fn
    call $move
  )

  (func $turnRight_fn
    call $turn_right
  )

  (func $main
    call $move_fn
    call $move_fn
    call $move_fn
    call $turn_right
    call $move_fn
    call $move_fn
  )
 
  (start $main)
)