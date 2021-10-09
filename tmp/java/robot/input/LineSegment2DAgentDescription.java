package robot.input;

import math.Vec3;

public class LineSegment2DAgentDescription {
    public final Vec3 startPose;
    public final Vec3 finishPose;
    public final float length;

    public LineSegment2DAgentDescription(Vec3 startPose, Vec3 finishPose, float length) {
        this.startPose = startPose;
        this.finishPose = finishPose;
        this.length = length;
    }
}
