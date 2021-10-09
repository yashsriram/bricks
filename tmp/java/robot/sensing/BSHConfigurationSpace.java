package robot.sensing;

import math.Vec3;
import robot.input.SphericalAgentDescription;
import fixed.SphericalObstacle;
import processing.core.PApplet;

import java.util.ArrayList;
import java.util.List;

class BoundingSphere {
    final List<BoundingSphere> children = new ArrayList<>();
    final Vec3 center;
    final float radius;
    final Vec3 color;

    public BoundingSphere(Vec3 center, float radius, Vec3 color) {
        this.center = center;
        this.color = color;
        this.radius = radius;
    }
}

public class BSHConfigurationSpace extends ConfigurationSpace {
    public static boolean DRAW_BOUNDING_SPHERES = false;
    public static float BOUNDING_SPHERE_COMPRESSION_SLACK = 100f;
    final PApplet parent;
    final SphericalAgentDescription sphericalAgentDescription;
    final BoundingSphere root;

    public BSHConfigurationSpace(final PApplet parent, final SphericalAgentDescription sphericalAgentDescription, final List<SphericalObstacle> sphericalObstacles) {
        this.parent = parent;
        this.sphericalAgentDescription = sphericalAgentDescription;

        if (sphericalObstacles.size() == 0) {
            throw new IllegalArgumentException("Need at least one obstacle");
        }

        List<BoundingSphere> boundingSpheres = new ArrayList<>(sphericalObstacles.size());
        // Initialize bounding spheres as obstacles themselves
        for (SphericalObstacle o : sphericalObstacles) {
            boundingSpheres.add(new BoundingSphere(o.center, o.radius, Vec3.of(1)));
        }

        // Throw exception if a sphere is already bounded in another
        for (int i = 0; i < boundingSpheres.size() - 1; i++) {
            for (int j = i + 1; j < boundingSpheres.size(); j++) {
                BoundingSphere b1 = boundingSpheres.get(i);
                BoundingSphere b2 = boundingSpheres.get(j);
                if (doesOneBoundAnother(b1, b2)) {
                    throw new IllegalArgumentException("Bad obstacles, one of the obstacle is completely bounded in other");
                }
            }
        }

        int numParentBoundingSpheres = 0;
        int biggestGroupSize = 0;
        // Create bounding spheres tree data structure
        while (boundingSpheres.size() > 1) {
            // Loop invariant: At this point no sphere bounds no other spheres in the list boundingSpheres

            // Find closest two spheres
            int x = 0;
            int y = 1;
            float leastDistance = boundingSpheres.get(y).center.minus(boundingSpheres.get(x).center).norm();
            boolean breakOut = false;
            for (int i = 0; i < boundingSpheres.size() - 1; i++) {
                for (int j = i + 1; j < boundingSpheres.size(); j++) {
                    BoundingSphere b1 = boundingSpheres.get(i);
                    BoundingSphere b2 = boundingSpheres.get(j);
                    float distance = b2.center.minus(b1.center).norm();
                    if (distance < leastDistance) {
                        x = i;
                        y = j;
                        leastDistance = distance;
                    }
                    if (distance < BOUNDING_SPHERE_COMPRESSION_SLACK) {
                        breakOut = true;
                        break;
                    }
                }
                if (breakOut) {
                    break;
                }
            }

            // Find the smallest sphere bounding the above two spheres
            BoundingSphere child1 = boundingSpheres.get(x);
            BoundingSphere child2 = boundingSpheres.get(y);
            Vec3 o2_o1_dir = child2.center.minus(child1.center).normalizeInPlace();
            Vec3 e1 = child1.center.plus(o2_o1_dir.scale(-child1.radius));
            Vec3 e2 = child2.center.plus(o2_o1_dir.scale(child2.radius));
            Vec3 parentCenter = e1.plus(e2).scale(0.5f);
            float parentRadius = e1.minus(parentCenter).norm();

            // Create a parent bounding sphere, add children to parent, remove children from list
            BoundingSphere parentSphere = new BoundingSphere(
                    parentCenter,
                    parentRadius,
                    Vec3.of(parent.random(1), parent.random(1), parent.random(1)));
            parentSphere.children.add(child1);
            parentSphere.children.add(child2);
            boundingSpheres.remove(y);
            boundingSpheres.remove(x);

            // Check for additional spheres that are bounded by parent
            // Getting size now is important as we modify list in the for loop
            int numBoundingSpheres = boundingSpheres.size();
            // Iterating from backwards is important as we remove elements in the list in for loop
            for (int i = numBoundingSpheres - 1; i >= 0; i--) {
                BoundingSphere sphere = boundingSpheres.get(i);
                if (doesOneBoundAnother(parentSphere, sphere)) {
                    parentSphere.children.add(sphere);
                    boundingSpheres.remove(i);
                }
            }

            // Add parent to bounding spheres list
            boundingSpheres.add(parentSphere);
            if (parentSphere.children.size() > biggestGroupSize) {
                biggestGroupSize = parentSphere.children.size();
            }
            numParentBoundingSpheres++;

            PApplet.print("BSH creation, " + "biggest group = " + biggestGroupSize + ", # left " + boundingSpheres.size() + "\r");
        }

        this.root = boundingSpheres.get(0);
        PApplet.println();
        PApplet.println("BSH created, #parent spheres = " + numParentBoundingSpheres);
    }

    private boolean doesOneBoundAnother(BoundingSphere b1, BoundingSphere b2) {
        float distanceBtwSpheres = b2.center.minus(b1.center).norm();
        return distanceBtwSpheres <= Math.abs(b1.radius - b2.radius);
    }

    private void drawRecursive(BoundingSphere sphere) {
        for (BoundingSphere bounded : sphere.children) {
            drawRecursive(bounded);
        }
        parent.pushMatrix();
        parent.translate(sphere.center.x, sphere.center.y, sphere.center.z);
        parent.stroke(sphere.color.x, sphere.color.y, sphere.color.z);
        parent.noFill();
        parent.sphere(sphere.radius);
        parent.popMatrix();
    }

    public void draw() {
        if (DRAW_BOUNDING_SPHERES) {
            drawRecursive(root);
        }
    }

    private boolean doesVertexIntersectThisSphere(final Vec3 p, final BoundingSphere node) {
        return p.minus(node.center).norm() <= node.radius + sphericalAgentDescription.radius;
    }

    private boolean doesEdgeIntersectThisSphere(final Vec3 p1, final Vec3 p2, final BoundingSphere node) {
        Vec3 pb_pa = p2.minus(p1);
        Vec3 pa_pc = p1.minus(node.center);
        float r = node.radius + sphericalAgentDescription.radius;
        float a = pb_pa.dot(pb_pa);
        float c = pa_pc.dot(pa_pc) - r * r;
        float b = 2 * pb_pa.dot(pa_pc);
        float discriminant = b * b - 4 * a * c;
        if (discriminant >= 0) {
            float t1 = (float) ((-b + Math.sqrt(discriminant)) / (2 * a));
            float t2 = (float) ((-b - Math.sqrt(discriminant)) / (2 * a));
            // Intersection with line segment only possible iff at least one of the solutions lies in [0, 1]
            return (0 <= t1 && t1 <= 1) || (0 <= t2 && t2 <= 1);
        }
        return false;
    }

    private boolean doesVertexIntersectSomeObstacleUnderThisBoundingSphere(final Vec3 p, final BoundingSphere node) {
        if (doesVertexIntersectThisSphere(p, node)) {
            // Intersects with this bounding sphere
            if (node.children.size() == 0) {
                // Leaf bounding sphere node i.e. actual obstacle
                return true;
            }

            // Not an actual obstacle => check for intersection with one of the children
            for (BoundingSphere child : node.children) {
                if (doesVertexIntersectSomeObstacleUnderThisBoundingSphere(p, child)) {
                    return true;
                }
            }
        }
        return false;
    }

    public boolean doesVertexIntersectSomeObstacle(final Vec3 p) {
        return doesVertexIntersectSomeObstacleUnderThisBoundingSphere(p, root);
    }

    private boolean doesEdgeIntersectSomeObstacleUnderThisBoundingSphere(final Vec3 p1, final Vec3 p2, final BoundingSphere node) {
        if (doesVertexIntersectThisSphere(p1, node)
                || doesVertexIntersectThisSphere(p2, node)
                || doesEdgeIntersectThisSphere(p1, p2, node)) {
            // Intersects with this bounding sphere
            if (node.children.size() == 0) {
                // Leaf bounding sphere node i.e. actual obstacle
                return true;
            }

            // Not an actual obstacle => check for intersection with one of the children
            for (BoundingSphere child : node.children) {
                if (doesEdgeIntersectSomeObstacleUnderThisBoundingSphere(p1, p2, child)) {
                    return true;
                }
            }
        }
        return false;
    }

    public boolean doesEdgeIntersectSomeObstacle(final Vec3 p1, final Vec3 p2) {
        return doesEdgeIntersectSomeObstacleUnderThisBoundingSphere(p1, p2, root);
    }
}
