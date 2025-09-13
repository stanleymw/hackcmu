(module
  (import "builtin" "move" (func $move))
  (import "builtin" "turn_right" (func $turn_right))

  (func $move_fn
    call $move
  )
 
  (start $move_fn)
)