(module
  (import "builtin" "move" (func $move))

  (func $move_fn
    call $move
    call $move
    call $move
    call $move
  )

  ;; Run the function $moveFn by name
  (start $move_fn)
)