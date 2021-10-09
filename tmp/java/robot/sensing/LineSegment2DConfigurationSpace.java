package robot.sensing;

import math.Vec3;
import robot.input.LineSegment2DAgentDescription;
import fixed.SphericalObstacle;
import processing.core.PApplet;

import java.util.ArrayList;
import java.util.List;

public class LineSegment2DConfigurationSpace extends ConfigurationSpace {
    final PApplet parent;
    final LineSegment2DAgentDescription description;
    final List<SphericalObstacle> sphericalObstacles;
    public final float orientationScale;

    public LineSegment2DConfigurationSpace(PApplet parent, LineSegment2DAgentDescription description, List<SphericalObstacle> sphericalObstacles, float orientationScale) {
        this.parent = parent;
        this.description = description;
        this.sphericalObstacles = sphericalObstacles;
        this.orientationScale = orientationScale;
    }

    public boolean doesVertexIntersectSomeObstacle(final Vec3 pose) {
        Vec3 p1Position = Vec3.of(pose);
        p1Position.x = 0;
        Vec3 p2Position = Vec3.of(pose);
        p2Position.x = 0;
        Vec3 halfLength = Vec3.of(0, (float) (Math.sin(pose.x / orientationScale) * description.length / 2), (float) (Math.cos(pose.x / orientationScale) * description.length / 2));
        p1Position.minusInPlace(halfLength);
        p2Position.plusInPlace(halfLength);

        for (SphericalObstacle sphericalObstacle : sphericalObstacles) {
            if (p1Position.minus(sphericalObstacle.center).norm() <= sphericalObstacle.radius
                    || p2Position.minus(sphericalObstacle.center).norm() <= sphericalObstacle.radius) {
                return true;
            }
            Vec3 pb_pa = p2Position.minus(p1Position);
            Vec3 pa_pc = p1Position.minus(sphericalObstacle.center);
            float r = sphericalObstacle.radius;
            float a = pb_pa.dot(pb_pa);
            float c = pa_pc.dot(pa_pc) - r * r;
            float b = 2 * pb_pa.dot(pa_pc);
            float discriminant = b * b - 4 * a * c;
            if (discriminant >= 0) {
                float t1 = (float) ((-b + Math.sqrt(discriminant)) / (2 * a));
                float t2 = (float) ((-b - Math.sqrt(discriminant)) / (2 * a));
                // Intersection with line segment only possible iff at least one of the solutions lies in [0, 1]
                if ((0 <= t1 && t1 <= 1) || (0 <= t2 && t2 <= 1)) {
                    return true;
                }
            }
        }

        return false;
    }

    public boolean doesEdgeIntersectSomeObstacle(Vec3 p1, Vec3 p2) {
        return false;
    }

    public boolean doesEdgeIntersectSomeObstacleStricter(Vec3 pose1, Vec3 pose2) {
        // Considers a bounding sphere of the line centered at center of line and radius as length / 2
        // Checks for collision of bounding sphere and obstacles
        // This always returns true when there is a collision
        // But there can be a case where there is no true collision but this return true
        // A loss occurs in that case
        Vec3 center1 = Vec3.of(pose1);
        center1.x = 0;
        Vec3 center2 = Vec3.of(pose2);
        center2.x = 0;

//        parent.pushMatrix();
//        parent.stroke(0, 0, 1);
//        parent.noFill();
//        parent.translate(center1.x, center1.y, center1.z);
//        parent.sphere(description.length / 2);
//        parent.popMatrix();
//
//        parent.pushMatrix();
//        parent.stroke(0, 1, 0);
//        parent.noFill();
//        parent.translate(center2.x, center2.y, center2.z);
//        parent.sphere(description.length / 2);
//        parent.popMatrix();

        for (SphericalObstacle sphericalObstacle : sphericalObstacles) {
            if (center1.minus(sphericalObstacle.center).norm() <= sphericalObstacle.radius + description.length / 2
                    || center2.minus(sphericalObstacle.center).norm() <= sphericalObstacle.radius + description.length / 2) {
                return true;
            }
            Vec3 pb_pa = center2.minus(center1);
            Vec3 pa_pc = center1.minus(sphericalObstacle.center);
            float r = sphericalObstacle.radius + description.length / 2;
            float a = pb_pa.dot(pb_pa);
            float c = pa_pc.dot(pa_pc) - r * r;
            float b = 2 * pb_pa.dot(pa_pc);
            float discriminant = b * b - 4 * a * c;
            if (discriminant >= 0) {
                float t1 = (float) ((-b + Math.sqrt(discriminant)) / (2 * a));
                float t2 = (float) ((-b - Math.sqrt(discriminant)) / (2 * a));
                // Intersection with line segment only possible iff at least one of the solutions lies in [0, 1]
                if ((0 <= t1 && t1 <= 1) || (0 <= t2 && t2 <= 1)) {
                    return true;
                }
            }
        }

        return false;
    }

    public void draw() {
    }
}
