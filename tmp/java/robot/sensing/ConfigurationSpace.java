package robot.sensing;

import math.Vec3;

public abstract class ConfigurationSpace {
    public abstract boolean doesVertexIntersectSomeObstacle(Vec3 p);

    public abstract boolean doesEdgeIntersectSomeObstacle(Vec3 p1, Vec3 p2);

    public abstract void draw();
}
