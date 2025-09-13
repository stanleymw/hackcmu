(module
  (import "builtin" "move" (func $move))
  
  (func $main
    (local $var1 i32) ;; Create a local variable $var1
    (local $var2 i32) ;; Create a local variable $var2 
    i32.const 40  ;; Load the constant 40 on the stack
    (local.set $var1)  ;; Will pop the value 40 from the stack and store it into $var1
    (local.set $var2 (i32.const 28)) ;; Directly load 28 into then local variable $var2 

    ;; Load the local variables onto the stack
    (local.get $var1)
    (local.get $var2)  
    i32.add

    drop    ;; remove the last portion 
  )
  
  (start $main)
)