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

public class EightBatches extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;
    public static final int SIDE = 100;
    final Vec3 minCorner = Vec3.of(-SIDE, -SIDE, -SIDE);
    final Vec3 maxCorner = Vec3.of(SIDE, SIDE, SIDE);

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

        sphericalObstacles.add(new SphericalObstacle(
                this,
                Vec3.of(-30, 30, 30),
                10,
                Vec3.of(1)
        ));
        sphericalObstacles.add(new SphericalObstacle(
                this,
                Vec3.of(-30, -30, 30),
                10,
                Vec3.of(1)
        ));
        sphericalObstacles.add(new SphericalObstacle(
                this,
                Vec3.of(-30, -30, -30),
                10,
                Vec3.of(1)
        ));
        sphericalObstacles.add(new SphericalObstacle(
                this,
                Vec3.of(-30, 30, -30),
                10,
                Vec3.of(1)
        ));

        sphericalObstacles.add(new SphericalObstacle(
                this,
                Vec3.of(30, 30, 30),
                10,
                Vec3.of(1)
        ));
        sphericalObstacles.add(new SphericalObstacle(
                this,
                Vec3.of(30, -30, 30),
                10,
                Vec3.of(1)
        ));
        sphericalObstacles.add(new SphericalObstacle(
                this,
                Vec3.of(30, -30, -30),
                10,
                Vec3.of(1)
        ));
        sphericalObstacles.add(new SphericalObstacle(
                this,
                Vec3.of(30, 30, -30),
                10,
                Vec3.of(1)
        ));

        Vec3 e1 = Vec3.of(SIDE * -0.8f, SIDE * 0.8f, SIDE * -0.8f);
        Vec3 e2 = Vec3.of(SIDE * 0.8f, SIDE * -0.8f, SIDE * 0.8f);
        placeAgents(e1, e2);
        placeAgents(e2, e1);

        Vec3 e3 = Vec3.of(SIDE * -0.8f, SIDE * 0.8f, SIDE * 0.8f);
        Vec3 e4 = Vec3.of(SIDE * 0.8f, SIDE * -0.8f, SIDE * -0.8f);
        placeAgents(e3, e4);
        placeAgents(e4, e3);

        Vec3 e5 = Vec3.of(SIDE * 0.8f, SIDE * 0.8f, SIDE * -0.8f);
        Vec3 e6 = Vec3.of(SIDE * -0.8f, SIDE * -0.8f, SIDE * 0.8f);
        placeAgents(e5, e6);
        placeAgents(e6, e5);

        Vec3 e7 = Vec3.of(SIDE * 0.8f, SIDE * 0.8f, SIDE * 0.8f);
        Vec3 e8 = Vec3.of(SIDE * -0.8f, SIDE * -0.8f, SIDE * -0.8f);
        placeAgents(e7, e8);
        placeAgents(e8, e7);

        Vec3 up = Vec3.of(SIDE * 0.8f, 0, 0);
        Vec3 down = Vec3.of(SIDE * -0.8f, 0, 0);
        placeAgents(up, down);
        placeAgents(down, up);

        Vec3 left = Vec3.of(0, SIDE * 0.8f, 0);
        Vec3 right = Vec3.of(0, SIDE * -0.8f, 0);
        placeAgents(left, right);
        placeAgents(right, left);

        Vec3 front = Vec3.of(0, 0, SIDE * 0.8f);
        Vec3 back = Vec3.of(0, 0, SIDE * -0.8f);
        placeAgents(front, back);
        placeAgents(back, front);

        ConfigurationSpace configurationSpace = new PlainConfigurationSpace(this, sphericalAgentDescriptions.get(0), sphericalObstacles);
        MultiSphericalAgentSystem.INITIAL_AGENT_SPEED = 1f;
        MultiSphericalAgentSystem.MAX_EDGE_LEN = 20f;

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

        multiSphericalAgentSystem = new MultiSphericalAgentSystem(this, sphericalAgentDescriptions, configurationSpace, minCorner, maxCorner, 14);
    }

    private void placeAgents(Vec3 start, Vec3 finish) {
        float agentRadius = 2f;
        float slack = 2f;
        int gridSize = 4;
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
        multiSphericalAgentSystem.drawBox();
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
        String[] appletArgs = new String[]{"demos.ttc.EightBatches"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
