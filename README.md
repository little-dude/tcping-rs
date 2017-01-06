TCPing
======

tcping is a small utility that makes it easy to debug tcp connections.  A
server listens from incoming TCP connections on arbitrary ports, and one or
multiple clients try to open these connections and send "ping" messages every
seconds.

Example
-------

On the server side:

```
$ tcping server --address 127.0.0.1:500  
127.0.0.1:500: connection established from 127.0.0.1:59472
127.0.0.1:500: connection established from 127.0.0.1:59474
127.0.0.1:500: connection with 127.0.0.1:59472 is closed (received EOF).
127.0.0.1:500: connection with 127.0.0.1:59474 is closed (received EOF).
127.0.0.1:500: connection established from 127.0.0.1:59476
127.0.0.1:500: connection with 127.0.0.1:59476 is closed (received EOF).
127.0.0.1:500: connection established from 127.0.0.1:59478
127.0.0.1:500: connection with 127.0.0.1:59478 is closed (received EOF).
127.0.0.1:500: connection established from 127.0.0.1:59480
127.0.0.1:500: connection with 127.0.0.1:59480 is closed (received EOF).
127.0.0.1:500: connection established from 127.0.0.1:59482
127.0.0.1:500: connection with 127.0.0.1:59482 is closed (received EOF).
127.0.0.1:500: connection established from 127.0.0.1:59488
127.0.0.1:500: connection with 127.0.0.1:59488 is closed (received EOF).
127.0.0.1:500: connection established from 127.0.0.1:59490
127.0.0.1:500: connection with 127.0.0.1:59490 is closed (received EOF).
127.0.0.1:500: connection established from 127.0.0.1:59492
127.0.0.1:500: connection with 127.0.0.1:59492 is closed (received EOF).
127.0.0.1:500: connection established from 127.0.0.1:59494
127.0.0.1:500: connection with 127.0.0.1:59494 is closed (received EOF).
```

On the client side:

```
$  tcping client --address 127.0.0.1:500 --reconnect --count 10 --interval 0.1
127.0.0.1:59472 >>> 127.0.0.1:500 connection successful (1)
127.0.0.1:59474 >>> 127.0.0.1:500 connection successful (2)
127.0.0.1:59476 >>> 127.0.0.1:500 connection successful (3)
127.0.0.1:59478 >>> 127.0.0.1:500 connection successful (4)
127.0.0.1:59480 >>> 127.0.0.1:500 connection successful (5)
127.0.0.1:59482 >>> 127.0.0.1:500 connection successful (6)
127.0.0.1:59488 >>> 127.0.0.1:500 connection successful (7)
127.0.0.1:59490 >>> 127.0.0.1:500 connection successful (8)
127.0.0.1:59492 >>> 127.0.0.1:500 connection successful (9)
127.0.0.1:59494 >>> 127.0.0.1:500 connection successful (10)
===============================
success: 10, failed 0
===============================
```
