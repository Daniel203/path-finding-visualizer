# Path Finding Visualizer
A website that allows you to visualize some path finding algorithms.

Try it out using this link: [https://daniel203.github.io/path-finding-visualizer/](https://daniel203.github.io/path-finding-visualizer/)


### Description
It's a website written in rust using the [yew](https://yew.rs/) crate. <br>
The main goal is to create a visual demonstration of some path-finding algorithms and 
also, of course, to practice rust and yew. <br>
It has a simple UI, it is easy to use, and the interface is nice and intuitive (at least I hope).

### Installation
1. Clone the repo
``` bash
git clone https://github.com/Daniel203/path-finding-visualizer.git 
cd path-finding-visualizer
```

2. Setup yew
``` bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

3. Run the server
``` bash
trunk serve
```
 4. On your browser go to [http://localhost:8080](http://localhost:8080/) or [http://127.0.0.1:8080/](http://127.0.0.1:8080/)


### Implemented algorithms
Path finding algorithms: 
- [x] BFS (Breadth First Search)
- [x] DFS (Depth First Search)
- [x] A*
- [x] A* Search

Maze generation algorithms: 
- [x] Binary tree
- [x] DFS
- [x] Recursive division


### TODO
- [ ] Implement more algorithms for path finding
- [ ] Implement more algorithms for maze generation
- [ ] Implement an animation speed selector
- [ ] Fix some bugs in the UI 
