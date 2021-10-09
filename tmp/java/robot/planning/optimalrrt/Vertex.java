package robot.planning.optimalrrt;

import math.Vec3;
import processing.core.PApplet;

import java.util.ArrayList;
import java.util.List;

class Vertex {
    private static int nextId = 1;

    private static int getNextId() {
        int currId = nextId;
        nextId++;
        return currId;
    }

    private final PApplet applet;
    final int id;
    final Vec3 position;
    float costFromStart;

    private final List<Vertex> children = new ArrayList<>();
    Vertex parent = null;

    static Vertex of(PApplet parent, Vec3 position, float distanceFromStart) {
        return new Vertex(parent, position, distanceFromStart);
    }

    public Vertex(PApplet applet, Vec3 position, float costFromStart) {
        this.applet = applet;
        this.id = getNextId();
        this.position = Vec3.of(position);
        this.costFromStart = costFromStart;
    }

    void draw() {
        applet.pushMatrix();
        applet.stroke(0, 0, 1);
        applet.point(position.x, position.y, position.z);
        applet.popMatrix();
        for (Vertex child : children) {
            applet.line(position.x, position.y, position.z,
                    child.position.x, child.position.y, child.position.z);
        }
    }

    void addChild(Vertex child) {
        children.add(child);
        child.parent = this;
    }

    void removeChild(Vertex child) {
        int indexToRemove = -1;
        for (int i = 0; i < children.size(); i++) {
            if (children.get(i).id == child.id) {
                indexToRemove = i;
                break;
            }
        }
        // orphan the child
        children.get(indexToRemove).parent = null;
        children.remove(indexToRemove);
    }

    List<Vertex> getChildren() {
        return children;
    }

    @Override
    public String toString() {
        return "Vertex{" +
                "id=" + id +
                '}';
    }

}
