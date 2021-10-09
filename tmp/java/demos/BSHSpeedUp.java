package demos;

import camera.QueasyCam;
import fixed.SphericalObstacle;
import math.Vec3;
import processing.core.PApplet;
import robot.acting.SphericalAgent;
import robot.input.SphericalAgentDescription;
import robot.planning.multiagentgraph.MultiAgentGraph;
import robot.sensing.BSHConfigurationSpace;
import robot.sensing.ConfigurationSpace;
import robot.sensing.PlainConfigurationSpace;

import java.util.ArrayList;
import java.util.List;

public class BSHSpeedUp extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;
    public static final int SIDE = 200;

    final Vec3 startPosition = Vec3.of(0, SIDE * 0.9f, SIDE * -0.9f);
    Vec3 finishPosition = Vec3.of(0, SIDE * -0.9f, SIDE * 0.9f);
    final Vec3 minCorner = Vec3.of(0, -SIDE, -SIDE);
    final Vec3 maxCorner = Vec3.of(0, SIDE, SIDE);

    SphericalAgentDescription sphericalAgentDescription;
    SphericalAgent sphericalAgent;
    List<SphericalObstacle> sphericalObstacles = new ArrayList<>();
    ConfigurationSpace bshConfigurationSpace;
    ConfigurationSpace plainConfigurationSpace;
    MultiAgentGraph graph;

    QueasyCam cam;

    static boolean DRAW_OBSTACLES = true;
    static String DATA_STRUCTURE = "";
    static long DATA_STRUCTURE_CREATION_TIME = 0;
    static long EDGE_CULLING_TIME = 0;

    public void settings() {
        size(WIDTH, HEIGHT, P3D);
    }

    public void setup() {
        surface.setTitle("Processing");
        colorMode(RGB, 1.0f);
        rectMode(CENTER);
        noStroke();

        cam = new QueasyCam(this);
        for (int i = -SIDE; i < SIDE; i = i + 4) {
            for (int j = -SIDE; j < SIDE; j = j + 4) {
                if (Math.abs(i * i * i + j) < 10) {
                    continue;
                }
                Vec3 center = Vec3.of(0, j, i);
                if (center.minus(startPosition).norm() < 10) {
                    continue;
                }
                if (center.minus(finishPosition).norm() < 10) {
                    continue;
                }
                sphericalObstacles.add(new SphericalObstacle(
                        this,
                        center,
                        1f,
                        Vec3.of(1, 0, 1)
                ));
            }
        }
        DRAW_OBSTACLES = false;

        sphericalAgentDescription = new SphericalAgentDescription(
                startPosition,
                finishPosition,
                0.5f
        );
        // configuration spaces
        long start = millis();
        plainConfigurationSpace = new PlainConfigurationSpace(this, sphericalAgentDescription, sphericalObstacles);
        long plain = millis();
        bshConfigurationSpace = new BSHConfigurationSpace(this, sphericalAgentDescription, sphericalObstacles);
        long bsh = millis();
        PApplet.println("Plain config space creation time: " + (plain - start) + " ms");
        PApplet.println("BSH config space creation time: " + (bsh - plain) + " ms");

        // graph
        graph = new MultiAgentGraph(this, startPosition, finishPosition);
        // spherical agent
        sphericalAgent = new SphericalAgent(this, sphericalAgentDescription, bshConfigurationSpace, minCorner, maxCorner, 20f, Vec3.of(1));

        // vertex sampling
        graph.generateVertices(sphericalAgent.samplePoints(7000), bshConfigurationSpace);
        resetBSH();
    }

    private void resetPlain() {
        DATA_STRUCTURE = "Plain";
        sphericalAgent = new SphericalAgent(this, sphericalAgentDescription, plainConfigurationSpace, minCorner, maxCorner, 20f, Vec3.of(1));
        long configSpace = millis();
        graph.clearAdjacenciesOnlyUseInBSHSpeedUp();
        graph.generateAdjacencies(30, plainConfigurationSpace);
        long edge = millis();
        EDGE_CULLING_TIME = edge - configSpace;
    }

    private void resetBSH() {
        DATA_STRUCTURE = "BSH";
        sphericalAgent = new SphericalAgent(this, sphericalAgentDescription, bshConfigurationSpace, minCorner, maxCorner, 20f, Vec3.of(1));
        long configSpace = millis();
        graph.clearAdjacenciesOnlyUseInBSHSpeedUp();
        graph.generateAdjacencies(30, bshConfigurationSpace);
        long edge = millis();
        EDGE_CULLING_TIME = edge - configSpace;
    }

    public void draw() {
        if (keyPressed) {
            if (keyCode == RIGHT) {
                sphericalAgent.stepForward();
            }
            if (keyCode == LEFT) {
                sphericalAgent.stepBackward();
            }
        }
        long start = millis();
        // update
        sphericalAgent.update(0.1f);
        long update = millis();
        // draw
        background(0);
        // obstacles
        if (DRAW_OBSTACLES) {
            for (SphericalObstacle sphericalObstacle : sphericalObstacles) {
                sphericalObstacle.drawBox();
            }
        }
        // agent
        sphericalAgent.draw();
        // configuration space
        bshConfigurationSpace.draw();
        // graph
        graph.draw();
        long draw = millis();

        surface.setTitle(
                "FPS: " + Math.round(frameRate)
                        + " " + DATA_STRUCTURE + " edge culling time: " + EDGE_CULLING_TIME + "ms"
                        + " #obs: " + sphericalObstacles.size()
        );
    }

    public void keyPressed() {
        if (key == 'b') {
            resetBSH();
            sphericalAgent.setPath(graph.weightedAStar(1.5f));
        }
        if (key == 'v') {
            resetPlain();
            sphericalAgent.setPath(graph.weightedAStar(1.5f));
        }
        if (key == 'g') {
            BSHConfigurationSpace.DRAW_BOUNDING_SPHERES = !BSHConfigurationSpace.DRAW_BOUNDING_SPHERES;
        }
        if (key == 'h') {
            DRAW_OBSTACLES = !DRAW_OBSTACLES;
        }
        if (key == 'k') {
            MultiAgentGraph.DRAW_VERTICES = !MultiAgentGraph.DRAW_VERTICES;
        }
        if (key == 'j') {
            MultiAgentGraph.DRAW_EDGES = !MultiAgentGraph.DRAW_EDGES;
        }
        if (key == 'p') {
            sphericalAgent.isPaused = !sphericalAgent.isPaused;
        }
        if (key == '1') {
            sphericalAgent.setPath(graph.dfs());
        }
        if (key == '2') {
            sphericalAgent.setPath(graph.bfs());
        }
        if (key == '3') {
            sphericalAgent.setPath(graph.ucs());
        }
        if (key == '4') {
            sphericalAgent.setPath(graph.aStar());
        }
        if (key == '5') {
            sphericalAgent.setPath(graph.weightedAStar(1.5f));
        }
    }

    static public void main(String[] passedArgs) {
        String[] appletArgs = new String[]{"demos.BSHSpeedUp"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
