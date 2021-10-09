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

public class TwoBatches extends PApplet {
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

    public void settings() {
        size(WIDTH, HEIGHT, P3D);
    }

    public void setup() {
        surface.setTitle("Processing");
        colorMode(RGB, 1.0f);
        rectMode(CENTER);
        noStroke();

        cam = new QueasyCam(this);

        Vec3 bottomLeft = Vec3.of(0, SIDE * 0.5f, SIDE * -0.9f);
        Vec3 topRight = Vec3.of(0, SIDE * -0.9f, SIDE * 0.5f);
        placeAgents(bottomLeft, topRight);
        placeAgents(topRight, bottomLeft);

        ConfigurationSpace configurationSpace = new PlainConfigurationSpace(this, sphericalAgentDescriptions.get(0), sphericalObstacles);
        MultiSphericalAgentSystem.INITIAL_AGENT_SPEED = 1f;

        MultiSphericalAgentSystem.TTC_K = 2000f;
        MultiSphericalAgentSystem.TTC_MAX_FORCE = 100;
        MultiSphericalAgentSystem.TTC_POWER = 4f;
        MultiSphericalAgentSystem.TTC_PERSONAL_SPACE = 4;
        MultiSphericalAgentSystem.TTC_SEPARATION_FORCE_K = 40f;

        MultiSphericalAgentSystem.TTC_COLLISION_CORRECTION_FORCE_K = 400;
        MultiAgentGraph.DRAW_VERTICES = false;

        SphericalAgent.DRAW_FUTURE_STATE = false;
        SphericalAgent.DRAW_PATH = false;

        multiSphericalAgentSystem = new MultiSphericalAgentSystem(this, sphericalAgentDescriptions, configurationSpace, minCorner, maxCorner, 2);
    }

    private void placeAgents(Vec3 start, Vec3 finish) {
        float agentRadius = 2f;
        float slack = 2f;
        int gridSize = 6;
        for (int i = 0; i < gridSize; i++) {
            for (int j = 0; j < gridSize; j++) {
                sphericalAgentDescriptions.add(new SphericalAgentDescription(
                        start.plus(Vec3.of(0, (2f + slack) * agentRadius * j, (2f + slack) * agentRadius * i)),
                        finish.plus(Vec3.of(0, (2f + slack) * agentRadius * (gridSize / 2), (2f + slack) * agentRadius * (gridSize / 2))),
                        agentRadius
                ));
            }
        }
    }

    public void draw() {
        long start = millis();
        // update
        for (int i = 0; i < 20; i++) {
            multiSphericalAgentSystem.updateTTC(sphericalObstacles, 0.05f);
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
        // multiagent system
        multiSphericalAgentSystem.draw();
        long draw = millis();

        surface.setTitle("Processing - FPS: " + Math.round(frameRate) + " Update: " + (update - start) + "ms Draw " + (draw - update) + "ms" + " search: " + SEARCH_ALGORITHM + " speed " + MultiSphericalAgentSystem.INITIAL_AGENT_SPEED);
    }

    public void keyPressed() {
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
        String[] appletArgs = new String[]{"demos.ttc.TwoBatches"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
