(module
  ;;(import func move)   @@@@@@@@@@@@@@FIX
  ;;(import func turnRight)   @@@@@@@@@@@@@@FIX

  (func $move_fn
    call move
  )

  (func $turnRight_fn
    call turnRight
  )

  (func $turnLeft_fn
    call turnRight
    call turnRight
    call turnRight
  )

  (func $main
    call move_fn
    call move_fn
    call turnLeft_fn
    call move_fn
    call move_fn
    call turnRight_fn
    call move_fn
    call move_fn
    call turnRight_fn
    call move_fn
    ;; Robot starts reversing direction and will proceed towards the goal
    call turn_right_fn
    call turn_right_fn
    call move_fn
    call turnLeft_fn
    call move_fn
    call move_fn
    call turnRight_fn
    call move_fn
    call move_fn
    call turn_left
    call move_fn
    call move_fn
    call move_fn
  )
 
  (start $main)
)