(module
  (import "builtin" "move" (func $move))
  (import "builtin" "turn_right" (func $turn_right))

  (func $main
    call $move
    call $move
  )

  (start $main)
)