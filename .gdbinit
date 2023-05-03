file target/thumbv6m-none-eabi/debug/tracker
# connect to OpenOCD on TCP port 3333
target remote 127.0.0.1:2345

# print demangled function/variable symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault

# *try* stopping at the user entry point (it might be gone due to inlining)
break main
break src/main.rs:63 

tui enable

monitor reset halt

continue
