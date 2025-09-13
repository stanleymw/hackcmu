(module
  ;;(import func move)   ;; Import the move function @@@@@@@@@FIX@@@@@@@@@@@

  (func $moveFn
    call move
    call move
    call move
    call move
  )

  ;; Run the function $moveFn by name
  (start $moveFn)
)