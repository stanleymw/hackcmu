(module
  ;;(import func move)   @@@@@@@@@@@@@@FIX
  ;;(import func turn_right)   @@@@@@@@@@@@@@FIX

  (func $move_fn
    call move
    call move
  )

  (func $turnRight_fn
    call turn_Right
  )
 
  (start $move_fn)
  (start $turnRight_fn)
  (start $move_fn)
  (start $turnRight_fn)
  (start $move_fn)
)