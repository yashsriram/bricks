package fixed;

import math.Vec3;
import processing.core.PApplet;
import processing.core.PShape;

public class SpriteSphericalObstacle extends SphericalObstacle {
    public final PShape shape;
    public final float normalizedSize;

    public SpriteSphericalObstacle(PApplet parent, Vec3 center, float radius, Vec3 color, PShape shape, float normalizedSize) {
        super(parent, center, radius, color);
        this.shape = shape;
        this.normalizedSize = normalizedSize;
    }

    public void draw() {
        parent.pushMatrix();
        parent.translate(center.x, center.y, center.z);
//        parent.noFill();
//        parent.stroke(color.x, color.y, color.z);
//        parent.sphere(radius);
        parent.scale(2 * radius / normalizedSize);
        parent.shape(shape);
        parent.popMatrix();
    }

}
