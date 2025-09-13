(module
  (import "builtin" "move" (func $move))
  (import "builtin" "turn_right" (func $turnRight))

  (func $moveForwardTwice
    call $move
    call $move
  )

  (start $moveForwardTwice)
)