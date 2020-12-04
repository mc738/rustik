# Rustik

Rustik is a small library/messaging protocol for sending data between processes over a network.

## Headers

There are 4 types of header:

* Handshake
    * A handshake, specifying the frames to be sent.
    * Size: 32 bytes
* Handshake Response
    * A response to handshake, saying the connection is ready and send frames.
    * Size: 8 bytes
* Frame
    * Sent with a frame.
    * Size: 8 bytes
* Frame Response
    * Sent to confirm a frame has been received.
    * Size: 8 bytes

### Handshake

```
__|_0_|_1_|_2_|_3_|_4_|_5_|_6_|_7_|
00|FROM           |F_SIZE |F_COUNT|
08|COR_ID                 |FLAGS  |
16|CHK_SUM  
32|
```
#### Key

1. FROM - the sender address 
    * 4 bytes
2. F_SIZE - the frame size
    * 2 bytes
    * Converts to `u16`
3. F_COUNT - total number of frames
    * 2 bytes
    * Converts to `u16`
4. COR_ID 6 - correlation id
    * 6 bytes
5. FLAGS - message and connection flags.
    * 2 bytes
5. CHK_SUM - `md5` check some of data
    * 16 bytes

### Handshake Response

```
__|_0_|_1_|_2_|_3_|_4_|_5_|_6_|_7_|
00|COR_ID                 |FLAGS  |
```
#### Key

1. COR_ID 6 - correlation id
    * 6 bytes
2. FLAGS - tba
    * 2 bytes

### Frame

```
__|_0_|_1_|_2_|_3_|_4_|_5_|_6_|_7_|
00|COR_ID                 |F_NO   |
```
#### Key

1. COR_ID 6 - correlation id
    * 6 bytes
2. F_NO - the fame number
    * 2 bytes
    * Converts to `u16`

### Frame Response

```
__|_0_|_1_|_2_|_3_|_4_|_5_|_6_|_7_|
08|COR_ID                 |F_NO   |
```
#### Key

1. COR_ID 6 - correlation id
    * 6 bytes
2. F_NO - the fame number
    * 2 bytes
    * Converts to `u16`

#### Notes

The frame response header is the same as the related frame header.    
