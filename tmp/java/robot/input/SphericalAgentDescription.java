package robot.input;

import math.Vec3;

public class SphericalAgentDescription {
    public final Vec3 startPosition;
    public final Vec3 finishPosition;
    public final float radius;

    public SphericalAgentDescription(Vec3 startPosition, Vec3 finishPosition,  float radius) {
        this.startPosition = Vec3.of(startPosition);
        this.finishPosition = Vec3.of(finishPosition);
        this.radius = radius;
    }
}
