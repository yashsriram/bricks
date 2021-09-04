### sanity check
- [ ] Small, simple, clear, useful, usable, independent, composable problems. No big problem. Big problem -> Compose(smaller problems).
- [ ] Decently thought about and provable (guarantees (correctness, time, space usage, ...), bounds, ...) solutions.
- [ ] Starting should consist mostly of what I already know, don't wait too much (it will be endless process otherwise).
    - The notion of independent starts, saaga deyyali ante entayna saaga deyyochu 0-1 lopala, but you want to get to 1, you are already at 1.
- [ ] Scalable (give bounds), Parallelize (when possible).
- [ ] Visualizable.
- [ ] Testable, get lots of good testcases.
    - Try to avoid "natural looking" motion generation problems.
- [ ] Think of physical manifestations, larger effects on yourself, society, ecosystem, everything and everyone.
- [ ] Teachable to others, learnable by others.
- [ ] Implement it in Rust and other good langagues (try to avoid C++).

### roadmap
- [ ] consume <- motion-planning repo
    - [ ] source code
    - [ ] demos
- [ ] Demos and tests until now
- [x] wireframe viz: lines, color, 3D Perspective FPS camera, minimal, speed (stress test)
- `State`, `StateSpace`
    - [x] Traits
    - [x] RectangularSpace
    - [ ] CircularSpace
    - [ ] CuboidSpace
    - [ ] SphereSpace
- `Graph`
    - [x] On a `StateSpace`
    - [x] Vertex = `StateSpace::State` +  `Set<VertexIdx>`
    - [x] To mesh
- `TreeSearch` on `Graph`
    - [x] start, stop, goal, max idxs
    - [x] handle stop not reached -> reachable subgraph explored
    - [x] TreeSearch = Open least cost on fringe + Propagate to unexplored adjacencies and add them to fringe
    - [x] Propagate trait = search state + cost priority
    - [x] CostPriority = Ord on cost + open min cost first + NaN cost is INF + NAN cost = NAN cost
    - [x] Searching for a stop = may finds path to stop + may some other vertices => so same search can be used to find paths to multiple vertices
    - [x] Multiple searches on a graph = State per search
    - [x] Parallelizable searches
    - [x] Large graph - small area search is inexpensive - uses sparse seach state using hashmaps
    - [x] Control initial alloc size of tree and fringe
    - [x] Get path to a goal, get path to stop, store start, stop, max idxs
    - [x] Remove Clone trait bound on vertex search state by merging Propagate and CostPriority
    - [x] Vis
    - [ ] BFS, UCS, A\* search states
- PRM
    - [x] Create a `Graph<StateSpace>`
    - [x] Sampling from `StateSpace`
    - [x] Connecting `Vertices<State>` using dist() trait fn and edge len
    - [x] Multi (agent) searchable from `Graph`
    - [ ] Spatial data structure
    - [ ] Growable
    - [ ] Modifiable (vertex and edge culling)
- actual problem to geometric search problem formulation
- meshes, transform, drawing options
- geometric entity description
    - [ ] Sphere
    - [ ] Line
- single state intersections (instantaneous)
    - [ ] sphere sphere
    - [ ] line-seg sphere
    - [ ] line-seg line-seg
- interval state intersections (ccd)
    - [ ] sphere sphere
    - [ ] line-seg sphere
    - [ ] line-seg line-seg
- problem description as composition of simple things like geometric entity, start, goal, option<obstacles>,
- RRT
    - [ ] sampling
    - [ ] growable
    - [ ] searches
- RRT\*
    - [ ] sampling
    - [ ] growable
    - [ ] searches
- Crowds
    - [ ] Boids
    - [ ] TTC

#### viz tools
graphics-viz, rust libs create/contribute.
1. Visualizer as alternatives for rviz and other robot vis utility tools.
1. Small interesting problems.
    1. Polynomial project improvement using (lin + non-lin) opts.
    1. Tesselation based vis of 3d functions.
    1. Use 2d vis to illustrate diff randomness.
1. FK, IK (backward kineamtics) animation.
1. Fourier bots
1. Speedup stuff e.g. Parallelization.

#### pp, rust libs create/contribute
- Churning motion-planning, escape-room and stick-solo repos; i.e. extracting a lib crate and bin crates from these repos.
- Path planning, Path planning with fancy abstractions (joint spaces) = motion planning, Path planning with times at vertices = trajectory planning?
- Methods
    - Direct straight line path from A to B
    - Gradient descent
    - Monte carlo sampling
    - PRM/(DFS, BFS, UCS, A\*, weighted-A\*)
    - RRT
    - RRT^\*
    - function approximator based policy (optimization, ML, DL, RL, ...)
    - Reflecting ripple search
- Motion planning, conf space obstacles
- Discriminators
    1. Global opt vs Local opt
    1. Can detect conf space obstacles vs cannot detect conf space obstacles
- Tasks as Nd space search
    1. Locomotion tasks (circle, diffdrive, line, bicycle, etc...)
    1. Kinematic chain (serial manipulator) e.g. Inverse kinematics
    1. Rocket (gravity wells + orbits + transfers)
    1. Ship (non-holonomic + water sim + replanning)
    1. Submarine (non-holonomic 3d)
    1. Car (non-holonomic 3d + road)
    1. Airplane/Drone (fluid dynamics)
- Multi agent planning, crowds, Multi-robot coordination.
    1. ttc
    1. RVO
    1. boids
    1. Forces
- Planning in Dynamic obstacles.
- Planning with unknown things using sensors.
- Combining classical, ML, DL, RL and any L planning.
- Modules
    1. Modeling problems into conf space
    1. Sampling conf space
    1. Connecting Vertices (continuous collision detection, spatial data structures for faster required operations)
    1. Graph search (graph structures, algorithms)
    1. Speedup stuff e.g. Parallelization.

#### sen
- sensing crate, rust libs create/contribute.
1. No noise sensor models and algos.
1. Noisy sensor models and algos.
1. Sensing static obstacles.
1. Sensing dynamic obstacles.
1. Speedup stuff e.g. Parallelization.
1. Combining classical, ML, DL, RL and any L sensing.

#### dyna
- dynamics crate, models of moving.
1. Speedup stuff e.g. Parallelization.

#### ctrl
- control crate, controls dyna models.
1. Speedup stuff e.g. Parallelization.
