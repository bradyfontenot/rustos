# Implement the ttywrite utility. 
## **Your implementation should:**

- [ ] set all flags from command-line
- [ ] read from stdin if no input file passed. 
- [ ] write the input data to the passed in serial device. 
  - [ ] IF: the -r flag is set, send the data as it is (RAW)
  - [ ] ELSE: use XModem to send data.
- [ ] Print the number of bytes sent on a successful transmission.


# 
# 
## **Figure it Out:**

- [x] how do I access opt?
- [ ] how do i read from stdin?
- [ ] how do i pass data to serial device (TTYPort)
- [x] how do I read/set settings on TTYPort


## Flags
- [x] tty_path
- [ ] raw
- [ ] input:        default = stdin(not set)
- [x] baud_rate:    default = 115200    BaudRate
- [x] timeout:      default = 10        u64
- [x] char_width:   default = 8         charsize
- [x] tty_path:     default = 
- [x] flow_control: default = none
- [x] stop_bits:    default = 1         StopBits
