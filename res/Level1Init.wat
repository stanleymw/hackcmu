(module
  (import "builtin" "move" (func $move))

  (func $main
    call $move
  )

  ;; Run the function $moveFn by name
  (start $main)
)