(module
  (import "builtin" "move" (func $move))
  (import "builtin" "turn_right" (func $turn_right))

  (func $moveForwardTwice
    call $move
    call $move
  )

  (start $moveForwardTwice)
)