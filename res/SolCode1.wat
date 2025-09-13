(module
  (import "builtin" "move" (func $move))

  (func $move_fn
    call $move
    call $move
    call $move
  )

  ;; Run the function $move_fn by name
  (start $move_fn)
)