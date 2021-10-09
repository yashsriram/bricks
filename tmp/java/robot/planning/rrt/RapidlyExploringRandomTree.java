package robot.planning.rrt;

import math.Vec3;
import processing.core.PApplet;
import robot.sensing.ConfigurationSpace;

import java.util.*;

public class RapidlyExploringRandomTree {
    public static float GROWTH_LIMIT = 20f;
    public static float END_POINT_HINT_SIZE = 2f;
    public static boolean DRAW_TREE = true;

    final PApplet applet;
    final Vec3 startPosition;
    final Vec3 finishPosition;
    final Vertex root;

    public RapidlyExploringRandomTree(PApplet applet, Vec3 startPosition, Vec3 finishPosition) {
        this.applet = applet;
        this.startPosition = Vec3.of(startPosition);
        this.finishPosition = Vec3.of(finishPosition);
        this.root = Vertex.of(applet, startPosition);
    }

    private Vertex getNearestVertexFrom(final Vec3 position) {
        Stack<Vertex> fringe = new Stack<>();
        fringe.add(root);
        float minDistance = position.minus(root.position).norm();
        Vertex nearestVertex = root;

        while (fringe.size() > 0) {
            Vertex node = fringe.pop();
            float distance = position.minus(node.position).norm();
            if (distance < minDistance) {
                minDistance = distance;
                nearestVertex = node;
            }
            fringe.addAll(node.getChildren());
        }
        return nearestVertex;
    }

    private Vertex getFinishVertex() {
        Stack<Vertex> fringe = new Stack<>();
        fringe.add(root);
        Vertex finishVertex = null;

        while (fringe.size() > 0) {
            Vertex node = fringe.pop();
            if (node.position.equals(finishPosition)) {
                finishVertex = node;
                break;
            }
            fringe.addAll(node.getChildren());
        }
        return finishVertex;
    }

    private void generateNextNode(Vec3 newPosition, ConfigurationSpace configurationSpace) {
        Vertex nearestVertex = getNearestVertexFrom(newPosition);
        Vec3 growth = newPosition.minus(nearestVertex.position);
        if (growth.norm() > GROWTH_LIMIT) {
            newPosition = nearestVertex.position.plus(growth.normalize().scale(GROWTH_LIMIT));
        }
        if (configurationSpace.doesEdgeIntersectSomeObstacle(nearestVertex.position, newPosition)) {
            return;
        }
        nearestVertex.addChild(Vertex.of(applet, newPosition));
    }

    public void growTree(List<Vec3> newPositions, ConfigurationSpace configurationSpace) {
        for (Vec3 newPosition : newPositions) {
            // generate node at finish position with a small probability
            if (applet.random(1) <= 0.01) {
                generateNextNode(finishPosition, configurationSpace);
            }
            generateNextNode(newPosition, configurationSpace);
        }
    }

    public void draw() {
        if (DRAW_TREE) {
            // tree
            Stack<Vertex> fringe = new Stack<>();
            fringe.add(root);

            while (fringe.size() > 0) {
                Vertex node = fringe.pop();
                node.draw();
                fringe.addAll(node.getChildren());
            }
        }
        // start
        applet.pushMatrix();
        applet.fill(0, 0, 1);
        applet.noStroke();
        applet.translate(startPosition.x, startPosition.y, startPosition.z);
        applet.box(END_POINT_HINT_SIZE);
        applet.popMatrix();
        // finish
        applet.pushMatrix();
        applet.fill(0, 1, 0);
        applet.noStroke();
        applet.translate(finishPosition.x, finishPosition.y, finishPosition.z);
        applet.box(END_POINT_HINT_SIZE);
        applet.popMatrix();
    }

    public List<Vec3> search() {
        Vertex finishVertex = getFinishVertex();
        if (finishVertex != null) {
            List<Vec3> path = new ArrayList<>();
            path.add(0, finishVertex.position);
            Vertex node = finishVertex.parent;
            while (node != null) {
                path.add(0, node.position);
                node = node.parent;
            }
            return path;
        }
        PApplet.println("Could not find path to finish position");
        return Collections.singletonList(startPosition);
    }
}
