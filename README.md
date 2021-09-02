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

### sanity check
- [ ] Small, simple, clear, independent problems rather than a big problem. Increment problem as we go.
- [ ] Useful, usable, rigourously understood solutions (proofs, guarantees (correctness, time, space usage, ...), bounds, ...).
- [ ] Starting should consist mostly of what I already know, don't wait too much (it will be endless process otherwise).
    - The notion of independent starts, saaga deyyali ante entayna saaga deyyochu 0-1 lopala, but you want to get to 1, you are already at 1.
- [ ] Reusable, composable.
- [ ] Scalable (give bounds), Parallelize (when possible).
- [ ] Visualizable.
- [ ] Testable, get lots of good testcases.
    - Try to avoid "natural looking" motion generation problems.
- [ ] Think of physical manifestations, larger effects on yourself, society, ecosystem, everything and everyone.
- [ ] Teachable to others, learnable by others.
- [ ] Implement it in Rust and other good langagues (try to avoid C++).

- viz
    - [x] lines
    - [x] color
    - [x] camera
    - [x] mutability
    - [x] prune pbr plugin
    - [x] speed (stress test)
- [ ] consume <- motion-planning repo
    - [ ] source code
    - [ ] demos
- actual problem to geometric search problem formulation
- meshes, transform, drawing options
- geometric entity description
    - [ ] Sphere
    - [ ] Line
- problem description
    - [ ] geometric entity
    - [ ] start
    - [ ] finish
    - [ ] option<obstacles>
- single state intersections (instantaneous)
    - [ ] sphere sphere
    - [ ] line-seg sphere
    - [ ] line-seg line-seg
- interval state intersections (ccd)
    - [ ] sphere sphere
    - [ ] line-seg sphere
    - [ ] line-seg line-seg
- [ ] Graph searches<Vertex<_, _>, CostFn(Vec<f32>, Vec<f32>) -> f32>
- PRM
    - [ ] sampling
    - [ ] growable
    - [ ] multi (agent) searchable
    - [ ] modifiable (vertex attributes, vertex and edge culling)
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
