## Introduction
Welcome (to anyone that may actually read this) to my journey writing a simple text editor from scratch. It's not really from scratch but I wasn't
really willing to go lower level than SDL2. I plan for this README to be mostly a development log for myself to keep notes on things I need to change
but I will be also giving some context into my decision making. Since this devlog is starting a little late I will note down some context before I
continue. 

### Aims and Context
The primary aim in making this text editor is getting back into Rust since I haven't really developed anything in it since university. A lot
of the code may not be idiomatic Rust as I get back into it but I will try to polish up when I have some actual features and am more confident 
in my code. 

For context, this text editor will only support ASCII and my primary goals are to implement some core text editor features and a performant data
structure implementation. This will likely not be a full, feature rich text editor my starting goal is to get to a point where I can code my text
editor with my text editor (at least a single file at a time). I have already implemented some basic stuff, I now have a static screen displaying
a page of a file, some data structure code and a cursor that moves around. 