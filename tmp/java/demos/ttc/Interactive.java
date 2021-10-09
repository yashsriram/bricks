package demos.ttc;

import camera.QueasyCam;
import fixed.SphericalObstacle;
import math.Vec3;
import processing.core.PApplet;
import robot.acting.MultiSphericalAgentSystem;
import robot.acting.SphericalAgent;
import robot.input.SphericalAgentDescription;
import robot.planning.multiagentgraph.MultiAgentGraph;
import robot.sensing.ConfigurationSpace;
import robot.sensing.PlainConfigurationSpace;

import java.util.ArrayList;
import java.util.List;

public class Interactive extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;
    public static final int SIDE = 100;
    final Vec3 minCorner = Vec3.of(0, -SIDE, -SIDE);
    final Vec3 maxCorner = Vec3.of(0, SIDE, SIDE);

    List<SphericalObstacle> sphericalObstacles = new ArrayList<>();
    MultiSphericalAgentSystem multiSphericalAgentSystem;
    List<SphericalAgentDescription> sphericalAgentDescriptions = new ArrayList<>();
    QueasyCam cam;

    static boolean DRAW_OBSTACLES = true;
    static String SEARCH_ALGORITHM = "";

    private Vec3 cursor = Vec3.zero();
    private List<Vec3> startPoints = new ArrayList<>();
    private List<Vec3> finishPoints = new ArrayList<>();
    private boolean placingObjects = true;
    private final int NUM_OBSTACLES = 2;
    private final int NUM_BATCHES = 2;

    public void settings() {
        size(WIDTH, HEIGHT, P3D);
    }

    public void setup() {
        surface.setTitle("Processing");
        colorMode(RGB, 1.0f);
        rectMode(CENTER);
        noStroke();

        cam = new QueasyCam(this);
        MultiSphericalAgentSystem.INITIAL_AGENT_SPEED = 1f;

        MultiSphericalAgentSystem.TTC_K = 4000f;
        MultiSphericalAgentSystem.TTC_MAX_FORCE = 300;
        MultiSphericalAgentSystem.TTC_POWER = 4f;
        MultiSphericalAgentSystem.TTC_PERSONAL_SPACE = 0;
        MultiSphericalAgentSystem.TTC_SEPARATION_FORCE_K = 0;

        MultiSphericalAgentSystem.TTC_COLLISION_CORRECTION_FORCE_K = 20;
        MultiAgentGraph.DRAW_VERTICES = false;
        MultiAgentGraph.DRAW_ENDS = false;

        SphericalAgent.DRAW_FUTURE_STATE = false;
        SphericalAgent.DRAW_PATH = false;
        reset();
    }

    private void reset() {
        sphericalObstacles.clear();
        sphericalAgentDescriptions.clear();
        placingObjects = true;
        cursor.set(0, 0, 0);
    }

    private void finishRemainingSetup() {
        for (int i = 0; i < NUM_BATCHES; i++) {
            placeAgents(startPoints.get(i), finishPoints.get(i));
        }
        ConfigurationSpace configurationSpace = new PlainConfigurationSpace(this, sphericalAgentDescriptions.get(0), sphericalObstacles);
        multiSphericalAgentSystem = new MultiSphericalAgentSystem(this, sphericalAgentDescriptions, configurationSpace, minCorner, maxCorner, NUM_BATCHES);
    }

    private void placeAgents(Vec3 start, Vec3 finish) {
        float agentRadius = 1f;
        float slack = 2f;
        int gridSize = 7;
        for (int i = 0; i < gridSize; i++) {
            for (int j = 0; j < gridSize; j++) {
                sphericalAgentDescriptions.add(new SphericalAgentDescription(
                        start.plus(Vec3.of(0, (2f + slack) * agentRadius * (j - gridSize / 2 + 1), (2f + slack) * agentRadius * (i - gridSize / 2 + 1))),
                        finish.plus(Vec3.of(0, (2f + slack) * agentRadius, (2f + slack) * agentRadius)),
                        agentRadius
                ));
            }
        }
    }

    public void draw() {
        // draw
        background(0);
        // starts and finishes
        for (Vec3 start : startPoints) {
            pushMatrix();
            fill(0, 0, 1);
            translate(start.x, start.y, start.z);
            box(2);
            popMatrix();
        }
        for (Vec3 finish : finishPoints) {
            pushMatrix();
            fill(0, 1, 0);
            translate(finish.x, finish.y, finish.z);
            box(2);
            popMatrix();
        }
        // obstacles
        if (DRAW_OBSTACLES) {
            for (SphericalObstacle sphericalObstacle : sphericalObstacles) {
                sphericalObstacle.draw();
            }
        }

        surface.setTitle("Processing - FPS: " + Math.round(frameRate) + " search: " + SEARCH_ALGORITHM);
        if (placingObjects) {
            if (keyPressed) {
                if (keyCode == RIGHT) {
                    cursor.plusInPlace(Vec3.of(0, 0, 1));
                } else if (keyCode == LEFT) {
                    cursor.plusInPlace(Vec3.of(0, 0, -1));
                } else if (keyCode == UP) {
                    cursor.plusInPlace(Vec3.of(0, -1, 0));
                } else if (keyCode == DOWN) {
                    cursor.plusInPlace(Vec3.of(0, 1, 0));
                }
            }
            // cursor
            pushMatrix();
            fill(1, 0, 0);
            translate(cursor.x, cursor.y, cursor.z);
            sphere(2);
            popMatrix();
            return;
        }
        // update
        for (int i = 0; i < 20; i++) {
            multiSphericalAgentSystem.updateTTC(sphericalObstacles, 0.05f);
        }
        // multiagent system
        multiSphericalAgentSystem.drawBox();
    }

    public void keyPressed() {
        if (key == 'r') {
            reset();
        }
        if (placingObjects) {
            if (key == 'o') {
                if (sphericalObstacles.size() < NUM_OBSTACLES) {
                    sphericalObstacles.add(new SphericalObstacle(
                            this,
                            Vec3.of(cursor),
                            10,
                            Vec3.of(1, 0, 1)
                    ));
                }
                return;
            }

            if (key == 'n') {
                if (startPoints.size() < NUM_BATCHES) {
                    startPoints.add(Vec3.of(cursor));
                }
            }
            if (key == 'm') {
                if (finishPoints.size() < NUM_BATCHES) {
                    finishPoints.add(Vec3.of(cursor));
                }
            }

            if (sphericalObstacles.size() == NUM_OBSTACLES && startPoints.size() == NUM_BATCHES && finishPoints.size() == NUM_BATCHES) {
                placingObjects = false;
                finishRemainingSetup();
                return;
            }

            return;
        }
        if (keyCode == RIGHT) {
            multiSphericalAgentSystem.stepForward();
        }
        if (keyCode == LEFT) {
            multiSphericalAgentSystem.stepBackward();
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
            multiSphericalAgentSystem.togglePause();
        }
        if (key == 'b') {
            SphericalAgent.DRAW_FUTURE_STATE = !SphericalAgent.DRAW_FUTURE_STATE;
        }
        if (key == 'v') {
            SphericalAgent.DRAW_PATH = !SphericalAgent.DRAW_PATH;
        }
        if (key == '1') {
            multiSphericalAgentSystem.dfs();
            SEARCH_ALGORITHM = "DFS";
        }
        if (key == '2') {
            multiSphericalAgentSystem.bfs();
            SEARCH_ALGORITHM = "BFS";
        }
        if (key == '3') {
            multiSphericalAgentSystem.ucs();
            SEARCH_ALGORITHM = "UCS";
        }
        if (key == '4') {
            multiSphericalAgentSystem.aStar();
            SEARCH_ALGORITHM = "A*";
        }
        if (key == '5') {
            float weight = 1.5f;
            multiSphericalAgentSystem.weightedAStar(weight);
            SEARCH_ALGORITHM = weight + "A*";
        }
    }

    static public void main(String[] passedArgs) {
        String[] appletArgs = new String[]{"demos.ttc.Interactive"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
