(module
  (import "builtin" "move" (func $move))

  (func $IncrementAndPrint (param $input i32) ;; create the IncrementAndPrint function using the correct signature
    local.get input
    i32.const 1
    i32.add
  )

  (func $main 
    i32.const 5   ;; will push 5 on the stack
    call $IncrementAndPrint   ;; will consume the value on the stack and call $IncrementAndPrint
  )

  (start $main)   
)
