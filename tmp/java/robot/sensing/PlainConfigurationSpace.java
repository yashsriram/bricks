package robot.sensing;

import math.Vec3;
import robot.input.SphericalAgentDescription;
import fixed.SphericalObstacle;
import processing.core.PApplet;

import java.util.ArrayList;
import java.util.List;

public class PlainConfigurationSpace extends ConfigurationSpace {
    final PApplet parent;
    final SphericalAgentDescription sphericalAgentDescription;
    final List<SphericalObstacle> sphericalObstacles;

    public PlainConfigurationSpace(PApplet parent, SphericalAgentDescription sphericalAgentDescription, List<SphericalObstacle> sphericalObstacles) {
        this.parent = parent;
        this.sphericalAgentDescription = sphericalAgentDescription;
        this.sphericalObstacles = sphericalObstacles;
    }

    public boolean doesVertexIntersectSomeObstacle(Vec3 p) {
        for (SphericalObstacle sphericalObstacle : sphericalObstacles) {
            if (p.minus(sphericalObstacle.center).norm() <= sphericalObstacle.radius + sphericalAgentDescription.radius) {
                return true;
            }
        }
        return false;
    }

    private boolean doesVertexIntersectThisSphere(final Vec3 p, final SphericalObstacle node) {
        return p.minus(node.center).norm() <= node.radius + sphericalAgentDescription.radius;
    }

    public boolean doesEdgeIntersectSomeObstacle(Vec3 p1, Vec3 p2) {
        for (SphericalObstacle sphericalObstacle : sphericalObstacles) {
            if (doesVertexIntersectThisSphere(p1, sphericalObstacle) || doesVertexIntersectThisSphere(p2, sphericalObstacle)) {
                return true;
            }
            Vec3 pb_pa = p2.minus(p1);
            Vec3 pa_pc = p1.minus(sphericalObstacle.center);
            float r = sphericalObstacle.radius + sphericalAgentDescription.radius;
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
