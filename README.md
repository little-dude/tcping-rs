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
$ tcping -s 127.0.0.1:1234 -s 127.0.0.1:4321
listening started on 127.0.0.1:1234, ready to accept
listening started on 127.0.0.1:4321, ready to accept
127.0.0.1:1234: connection established from 127.0.0.1:53156
127.0.0.1:1234 <<< 127.0.0.1:53156 ping 0
127.0.0.1:1234 >>> 127.0.0.1:53156 ping 0
127.0.0.1:1234 <<< 127.0.0.1:53156 ping 1
127.0.0.1:1234 >>> 127.0.0.1:53156 ping 1
127.0.0.1:1234 <<< 127.0.0.1:53156 ping 2
127.0.0.1:1234 >>> 127.0.0.1:53156 ping 2
127.0.0.1:1234 <<< 127.0.0.1:53156 ping 3
127.0.0.1:1234 >>> 127.0.0.1:53156 ping 3
127.0.0.1:1234 <<< 127.0.0.1:53156 ping 4
127.0.0.1:1234 >>> 127.0.0.1:53156 ping 4
127.0.0.1:1234: connection with 127.0.0.1:53156 is closed (received EOF).
```

On the client side:

```
$ tcping -c 127.0.0.1:1234
Establishing session to 127.0.0.1:1234
127.0.0.1:53156 >>> 127.0.0.1:1234: ping 0
127.0.0.1:53156 <<< 127.0.0.1:1234: ping 0
127.0.0.1:53156 >>> 127.0.0.1:1234: ping 1
127.0.0.1:53156 <<< 127.0.0.1:1234: ping 1
127.0.0.1:53156 >>> 127.0.0.1:1234: ping 2
127.0.0.1:53156 <<< 127.0.0.1:1234: ping 2
127.0.0.1:53156 >>> 127.0.0.1:1234: ping 3
127.0.0.1:53156 <<< 127.0.0.1:1234: ping 3
127.0.0.1:53156 >>> 127.0.0.1:1234: ping 4
127.0.0.1:53156 <<< 127.0.0.1:1234: ping 4
^C
```
