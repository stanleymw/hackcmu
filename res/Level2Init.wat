(module
  (import "builtin" "move" (func $move))
  (import "builtin" "turn_right" (func $turnRight))

  (func $move_fn
    call $move
  )
 
  (start $move_fn)
)