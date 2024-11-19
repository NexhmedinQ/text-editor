## Introduction
Welcome (to anyone that may actually read this) to my journey writing a simple text editor from scratch. It's not really from scratch but I wasn't
really willing to go lower level than SDL2. I plan for this README to be mostly a development log for myself to keep notes on things I need to change
but I will be also giving some context into my decision making. Since this devlog is starting a little late I will note down some context before I
continue. 

### Aims and Context
The primary aim in making this text editor is getting back into Rust since I haven't really developed anything in it since university. A lot
of the code may not be idiomatic Rust but I will try to polish up when I have some actual features and am more confident 
in my code. 

For context, this text editor will only support ASCII and my primary goals are to implement some core text editor features and a performant data
structure implementation. This will likely not be a full, feature rich text editor my starting goal is to get to a point where I can code my text
editor with my text editor (at least a single file at a time). I have already implemented some basic stuff, I now have a static screen displaying
a page of a file, some data structure code and a cursor that moves around. 

### 13/11/24
My aim has been to get started on implementing text insertion. My data structure for storing the text in memory is a piece chain
which uses a doubly linked list under the hood and is inspired by https://www.catch22.net/tuts/neatpad/piece-chains/. Now the main issue 
I have is that it would be very inefficient to create new spans for each character that gets added despite this being the simplest solution. 
This would lead to higher memory usage considering a span is represented by 
```struct Span {
    newlines: u32,
    is_append: bool,
    start_index: usize,
    end_index: usize,
}``` plus the fact that we need to maintain a front and back pointer. As more and more edits occur the performance of the data structure will also take a hit considering we need to iterate through the linked list of spans. Also, in future when we want to implement redo and undo this will result in
one character at a time being removed or added. So we need a better solution. My thought is to batch the inserts so that we only insert into the 
data stucture when a new word in its entirety has been typed out. I'm not quite sure if this is the correct approach though considering I do want
to add text highlighting later so the highlighting for new input will only occur after the user adds some sort of whitespace or moves the 
cursor. An alternative would be to keep track of the span we're adding the word to and modify it as each letter is added although then we'll start
getting into all sorts of headaches with shared ownership so lets keep it simple for now. 

While implementing text insertion I realised how bare bones my doubly linked list is so we're going to need to address that first. Unfortunately, making the doubly linked list safe to the outside view while still giving me the features I want is a lot of work. So for now I'm going to expose a completely unsafe
API and rewrite the implementation later. This completely goes against Rusts philosophy and I may as well have written it in C++ but I want to see some functionality before spending time trying to fight the compiler with a feature rich doubly linked list implementation. 
