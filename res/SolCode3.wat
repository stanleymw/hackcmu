(module
  ;;(import func move)   @@@@@@@@@@@@@@FIX
  ;;(import func turn_right)   @@@@@@@@@@@@@@FIX

  (func $move_fn
    call move
  )

  (func $turnRight_fn
    call turn_right
  )

  (func $turnLeft_fn
    call turn_right
    call turn_right
    call turn_right
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