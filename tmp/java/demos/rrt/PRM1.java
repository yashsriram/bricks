package demos.rrt;

import camera.QueasyCam;
import fixed.SphericalObstacle;
import math.Vec3;
import processing.core.PApplet;
import robot.acting.SphericalAgent;
import robot.input.SphericalAgentDescription;
import robot.planning.multiagentgraph.MultiAgentGraph;
import robot.sensing.PlainConfigurationSpace;

import java.util.ArrayList;
import java.util.List;

public class PRM1 extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;
    public static final int SIDE = 100;
    final Vec3 minCorner = Vec3.of(0, -SIDE, -SIDE);
    final Vec3 maxCorner = Vec3.of(0, SIDE, SIDE);

    final Vec3 startPosition = Vec3.of(0, SIDE * 0.9f, -SIDE * 0.9f);
    final Vec3 finishPosition = Vec3.of(0, -SIDE * 0.9f, -SIDE * 0.9f);
    SphericalAgentDescription sphericalAgentDescription;
    SphericalAgent sphericalAgent;
    List<SphericalObstacle> sphericalObstacles = new ArrayList<>();
    MultiAgentGraph graph;

    QueasyCam cam;

    static boolean DRAW_OBSTACLES = true;
    static String SEARCH_ALGORITHM = "";
    static boolean SMOOTH_PATH = false;

    public void settings() {
        size(WIDTH, HEIGHT, P3D);
    }

    public void setup() {
        surface.setTitle("Processing");
        colorMode(RGB, 1.0f);
        rectMode(CENTER);
        noStroke();

        cam = new QueasyCam(this);
        for (int i = 0; i < 5; i++) {
            sphericalObstacles.add(new SphericalObstacle(
                    this,
                    Vec3.of(0, SIDE * 0.9f - 20, SIDE * -0.9f + 20 * i - 10),
                    SIDE * 0.1f,
                    Vec3.of(1, 0, 1)
            ));
        }
        for (int i = 0; i < 5; i++) {
            sphericalObstacles.add(new SphericalObstacle(
                    this,
                    Vec3.of(0, SIDE * -0.9f + 20, SIDE * -0.9f + 20 * i - 10),
                    SIDE * 0.1f,
                    Vec3.of(1, 0, 1)
            ));
        }
        sphericalObstacles.add(new SphericalObstacle(
                this,
                Vec3.of(0, 0, 0),
                SIDE * 0.35f,
                Vec3.of(1, 0, 1)
        ));
        sphericalAgentDescription = new SphericalAgentDescription(
                startPosition,
                finishPosition,
                SIDE * (0.5f / 20)
        );
        PlainConfigurationSpace configurationSpace = new PlainConfigurationSpace(this, sphericalAgentDescription, sphericalObstacles);
        sphericalAgent = new SphericalAgent(this, sphericalAgentDescription, configurationSpace, minCorner, maxCorner, 20f, Vec3.of(1));
        graph = new MultiAgentGraph(this, startPosition, finishPosition);
        graph.generateVertices(sphericalAgent.samplePoints(10000), configurationSpace);
        graph.generateAdjacencies(10, configurationSpace);
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
        if (SMOOTH_PATH) {
            sphericalAgent.smoothUpdate(0.1f);
        } else {
            sphericalAgent.update(0.1f);
        }
        long update = millis();
        // draw
        background(0);
        // obstacles
        if (DRAW_OBSTACLES) {
            for (SphericalObstacle sphericalObstacle : sphericalObstacles) {
                sphericalObstacle.draw();
            }
        }
        // agent
        sphericalAgent.draw();
        // graph
        graph.draw();
        long draw = millis();

        surface.setTitle("Processing - FPS: " + Math.round(frameRate) + " Update: " + (update - start) + "ms Draw " + (draw - update) + "ms" + " search: " + SEARCH_ALGORITHM + " smooth-path: " + SMOOTH_PATH);
    }

    public void keyPressed() {
        if (key == 'x') {
            SMOOTH_PATH = !SMOOTH_PATH;
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
            SEARCH_ALGORITHM = "DFS";
        }
        if (key == '2') {
            sphericalAgent.setPath(graph.bfs());
            SEARCH_ALGORITHM = "BFS";
        }
        if (key == '3') {
            sphericalAgent.setPath(graph.ucs());
            SEARCH_ALGORITHM = "UCS";
        }
        if (key == '4') {
            sphericalAgent.setPath(graph.aStar());
            SEARCH_ALGORITHM = "A*";
        }
        if (key == '5') {
            float weight = 1.5f;
            sphericalAgent.setPath(graph.weightedAStar(weight));
            SEARCH_ALGORITHM = weight + "A*";
        }
    }

    static public void main(String[] passedArgs) {
        String[] appletArgs = new String[]{"demos.rrt.PRM1"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
