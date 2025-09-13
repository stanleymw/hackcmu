(module
  (import "builtin" "move" (func $move))

  (func $move_fn
    call $move
  )
 
  (start $move_fn)
)