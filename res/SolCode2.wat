(module
  (import "builtin" "move" (func $move))
  (import "builtin" "turn_right" (func $turn_right))

  (func $move_fn
    call $move
    call $move
  )

  (func $turnRight_fn
    call $turn_right
  )
 
  (start $move_fn)
  (start $turnRight_fn)
  (start $move_fn)
  (start $turnRight_fn)
  (start $move_fn)
)