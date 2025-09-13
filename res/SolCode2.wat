(module
  ;;(import func move)   @@@@@@@@@@@@@@FIX
  ;;(import func turnRight)   @@@@@@@@@@@@@@FIX

  (func $move_fn
    call move
    call move
  )

  (func $turnRight_fn
    call turnRight
  )
 
  (start $move_fn)
  (start $turnRight_fn)
  (start $move_fn)
  (start $turnRight_fn)
  (start $move_fn)
)