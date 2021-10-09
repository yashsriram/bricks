package demos.boids;

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

public class ZigZag extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;
    public static final int SIDE = 100;
    final Vec3 minCorner = Vec3.of(0, -SIDE, -SIDE);
    final Vec3 maxCorner = Vec3.of(0, SIDE, SIDE);

    List<SphericalObstacle> sphericalObstacles = new ArrayList<>();
    MultiSphericalAgentSystem multiSphericalAgentSystem;
    QueasyCam cam;
    List<List<Vec3>> finishPositions = new ArrayList<>();
    List<List<SphericalAgent>> flocks = new ArrayList<>();
    List<Vec3> finish1 = new ArrayList<>();
    List<Vec3> finish2 = new ArrayList<>();
    List<SphericalAgentDescription> sphericalAgentDescriptions = new ArrayList<>();

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
        float radiusFactor = 0.06f;
        float obstacleRadius = SIDE * radiusFactor;
        int numRows = 4;
        int rowLength = 12;
        float a = 30;
        float b = 50;
        for (int i = 0; i < rowLength; i++) {
            for (int j = 0; j < numRows; j++) {
                float zCoordinate = (SIDE - 2 * obstacleRadius * i) * (j % 2 == 1 ? -1 : 1);
                sphericalObstacles.add(new SphericalObstacle(
                        this,
                        Vec3.of(0, -SIDE + a * j + b, zCoordinate),
                        obstacleRadius,
                        Vec3.of(1, 0, 1)
                ));
            }
        }

        Vec3 bottomLeft = Vec3.of(0, SIDE * 0.7f, SIDE * -0.9f);
        Vec3 topRight = Vec3.of(0, SIDE * -0.9f, SIDE * 0.7f);
        placeAgents(bottomLeft, topRight, finish1);
        placeAgents(topRight, bottomLeft, finish2);
        finishPositions.add(finish1);
        finishPositions.add(finish2);

        ConfigurationSpace configurationSpace = new PlainConfigurationSpace(this, sphericalAgentDescriptions.get(0), sphericalObstacles);
        multiSphericalAgentSystem = new MultiSphericalAgentSystem(this, sphericalAgentDescriptions, configurationSpace, minCorner, maxCorner, 2);

        buildFLock(finishPositions, multiSphericalAgentSystem);

        MultiSphericalAgentSystem.INITIAL_AGENT_SPEED = 2f;
        MultiAgentGraph.DRAW_ENDS = false;
        MultiAgentGraph.DRAW_VERTICES = false;

        SphericalAgent.DRAW_FUTURE_STATE = false;
        SphericalAgent.DRAW_PATH = false;
        // tuning parameters
        SphericalAgent.IMPACT_RADIUS = 15f;
        SphericalAgent.SEPERATION_FORCE_BOID = 1f;
        SphericalAgent.SEPERATION_FORCE_OBSTACLE = 7.5f;
        SphericalAgent.ALIGNMENT_FORCE = 5f;
        SphericalAgent.CENTROID_FORCE = 10f;
//        SphericalAgent.REPULSION = 0.000000001f ;
    }

    private void placeAgents(Vec3 start, Vec3 finish, List<Vec3> fin) {
        float agentRadius = SIDE * 0.01f;
        int gridSize = 4;
        for (int i = 0; i < gridSize; i++) {
            for (int j = 0; j < gridSize; j++) {
                sphericalAgentDescriptions.add(new SphericalAgentDescription(
                        start.plus(Vec3.of(0, 3f * agentRadius * j, 3f * agentRadius * i)),
                        finish,
                        agentRadius
                ));
                fin.add(finish);
            }
        }
    }

    public void buildFLock(List<List<Vec3>> finishPositions, MultiSphericalAgentSystem multiSphericalAgentSystem) {
        int i = 0;
        for (List<Vec3> list : finishPositions) {
            List<SphericalAgent> flock = new ArrayList<>();
            for (Vec3 pos : list) {
                flock.add(multiSphericalAgentSystem.sphericalAgents.get(i));
                i += 1;
            }
            flocks.add(flock);
        }
    }

    public void draw() {
        long start = millis();
        // update
        for (int i = 0; i < 10; i++) {
            multiSphericalAgentSystem.updateClan(flocks, sphericalObstacles, 0.01f);
        }
//        multiSphericalAgentSystem.updateClan(flocks, sphericalObstacles, 0.01f);
//        multiSphericalAgentSystem.updateBoid(sphericalObstacles, 0.1f);
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

        surface.setTitle("Processing - FPS: " + Math.round(frameRate) + " Update: " + (update - start) + "ms Draw " + (draw - update) + "ms" + " search: " + SEARCH_ALGORITHM);
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
        String[] appletArgs = new String[]{"demos.boids.ZigZag"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
