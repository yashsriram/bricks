package demos.boids;

import camera.QueasyCam;
import fixed.Ground;
import fixed.SphericalObstacle;
import fixed.SpriteSphericalObstacle;
import math.Vec3;
import processing.core.PApplet;
import processing.core.PConstants;
import processing.core.PShape;
import robot.acting.MultiSphericalAgentSystem;
import robot.acting.SphericalAgent;
import robot.input.SphericalAgentDescription;
import robot.planning.multiagentgraph.MultiAgentGraph;
import robot.sensing.PlainConfigurationSpace;

import java.util.ArrayList;
import java.util.List;

public class CSCI5611 extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;
    public static final int SIDE = 1000;

    final Vec3 OFFSET = Vec3.of(100, 100, 0);
    final Vec3 finishPosition = Vec3.of(SIDE * 0.9f, 0, SIDE * 0.9f).plusInPlace(OFFSET);
    final Vec3 minCorner = Vec3.of(-SIDE, 0, -SIDE).plusInPlace(OFFSET);
    final Vec3 maxCorner = Vec3.of(SIDE, 0, SIDE).plusInPlace(OFFSET);

    MultiSphericalAgentSystem multiSphericalAgentSystem;
    List<SphericalAgentDescription> sphericalAgentDescriptions;
    Ground ground;
    List<SphericalObstacle> sphericalObstacles = new ArrayList<>();
    List<PShape> agentWalkCycleShapes = new ArrayList<>();
    List<PShape> obstacleShapes = new ArrayList<>();
    QueasyCam cam;
    List<List<Vec3>> finishPositions = new ArrayList<>();
    List<List<SphericalAgent>> flocks = new ArrayList<>();
    List<Vec3> C = new ArrayList<>();
    List<Vec3> S = new ArrayList<>();
    List<Vec3> C1 = new ArrayList<>();
    List<Vec3> I = new ArrayList<>();
    List<Vec3> C5 = new ArrayList<>();
    List<Vec3> C6 = new ArrayList<>();
    List<Vec3> C11 = new ArrayList<>();
    List<Vec3> C12 = new ArrayList<>();

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
        cam.speed = 10f;
        sphericalAgentDescriptions = new ArrayList<>();

        for (int i = 0; i < 7; i++) {
            if (i == 0) {
                continue;
            }
            PShape shape = loadShape("data/rocks/Rock_" + (i + 1) + ".obj");
            shape.rotateX(PConstants.PI);
            obstacleShapes.add(shape);
        }
        for (int i = 0; i < 5; i++) {
            if (i == 1 || i == 2) {
                continue;
            }
            PShape shape = loadShape("data/plants/Plant_" + (i + 1) + ".obj");
            shape.rotateX(PConstants.PI);
            obstacleShapes.add(shape);
        }
        for (int i = 0; i < 2; i++) {
            PShape shape = loadShape("data/bushes/Bush_" + (i + 1) + ".obj");
            shape.rotateX(PConstants.PI);
            obstacleShapes.add(shape);
        }
        for (int i = 0; i < 2; i++) {
            PShape shape = loadShape("data/bushes/snow/Bush_Snow_" + (i + 1) + ".obj");
            shape.rotateX(PConstants.PI);
            obstacleShapes.add(shape);
        }
        for (int i = 0; i < 2; i++) {
            PShape shape = loadShape("data/bushes/berries/BushBerries_" + (i + 1) + ".obj");
            shape.rotateX(PConstants.PI);
            obstacleShapes.add(shape);
        }

        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 5; j++) {
                PShape shape = obstacleShapes.get((int) random(obstacleShapes.size()));
                float normalizedSize = Math.max(Math.max(shape.getWidth(), shape.getHeight()), shape.getDepth()) + 0.3f;
                sphericalObstacles.add(new SpriteSphericalObstacle(
                        this,
                        Vec3.of(SIDE * (0.1f - (i / 10f)), 0, SIDE * (1.5f * j / 10f - 0.9f)).plus(OFFSET),
                        SIDE * 0.10f * random(0.5f, 1),
                        Vec3.of(1, 0, 0),
                        shape,
                        normalizedSize
                ));
            }
        }

        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 5; j++) {
                PShape shape = obstacleShapes.get((int) random(obstacleShapes.size()));
                float normalizedSize = Math.max(Math.max(shape.getWidth(), shape.getHeight()), shape.getDepth()) + 0.3f;
                sphericalObstacles.add(new SpriteSphericalObstacle(
                        this,
                        Vec3.of(SIDE * (0.1f - (i / 10f)), 0, SIDE * (-1.5f * j / 10f + 0.9f)).plus(OFFSET),
                        SIDE * 0.10f * random(0.5f, 1),
                        Vec3.of(1, 0, 0),
                        shape,
                        normalizedSize
                ));
            }
        }

        //5
        C5.add(Vec3.of(SIDE * -0.6f, 0, SIDE * -0.5f).plusInPlace(OFFSET));
        C5.add(Vec3.of(SIDE * -0.6f, 0, SIDE * -0.7f).plusInPlace(OFFSET));
        C5.add(Vec3.of(SIDE * -0.7f, 0, SIDE * -0.7f).plusInPlace(OFFSET));
        C5.add(Vec3.of(SIDE * -0.7f, 0, SIDE * -0.5f).plusInPlace(OFFSET));
        C5.add(Vec3.of(SIDE * -0.9f, 0, SIDE * -0.5f).plusInPlace(OFFSET));
        C5.add(Vec3.of(SIDE * -0.9f, 0, SIDE * -0.7f).plusInPlace(OFFSET));
        finishPositions.add(C5);

        //6
        C6.add(Vec3.of(SIDE * -0.6f, 0, SIDE * -0.0f).plusInPlace(OFFSET));
        C6.add(Vec3.of(SIDE * -0.6f, 0, SIDE * -0.2f).plusInPlace(OFFSET));
        C6.add(Vec3.of(SIDE * -0.7f, 0, SIDE * -0.2f).plusInPlace(OFFSET));
        C6.add(Vec3.of(SIDE * -0.7f, 0, SIDE * -0.0f).plusInPlace(OFFSET));
        C6.add(Vec3.of(SIDE * -0.9f, 0, SIDE * -0.0f).plusInPlace(OFFSET));
        C6.add(Vec3.of(SIDE * -0.9f, 0, SIDE * -0.2f).plusInPlace(OFFSET));
        C6.add(Vec3.of(SIDE * -0.7f, 0, SIDE * -0.2f).plusInPlace(OFFSET));
        finishPositions.add(C6);

        //1
        C11.add(Vec3.of(SIDE * -0.6f, 0, SIDE * 0.4f).plusInPlace(OFFSET));
        C11.add(Vec3.of(SIDE * -0.7f, 0, SIDE * 0.4f).plusInPlace(OFFSET));
        C11.add(Vec3.of(SIDE * -0.8f, 0, SIDE * 0.4f).plusInPlace(OFFSET));
        C11.add(Vec3.of(SIDE * -0.9f, 0, SIDE * 0.4f).plusInPlace(OFFSET));
        finishPositions.add(C11);

        //1
        C12.add(Vec3.of(SIDE * -0.6f, 0, SIDE * 0.8f).plusInPlace(OFFSET));
        C12.add(Vec3.of(SIDE * -0.7f, 0, SIDE * 0.8f).plusInPlace(OFFSET));
        C12.add(Vec3.of(SIDE * -0.8f, 0, SIDE * 0.8f).plusInPlace(OFFSET));
        C12.add(Vec3.of(SIDE * -0.9f, 0, SIDE * 0.8f).plusInPlace(OFFSET));
        finishPositions.add(C12);

        //C
        C.add(Vec3.of(SIDE * 0.9f, 0, SIDE * -0.5f).plusInPlace(OFFSET));
        C.add(Vec3.of(SIDE * 0.75f, 0, SIDE * -0.65f).plusInPlace(OFFSET));
        C.add(Vec3.of(SIDE * 0.6f, 0, SIDE * -0.5f).plusInPlace(OFFSET));
        finishPositions.add(C);

        //S
        S.add(Vec3.of(SIDE * 0.9f, 0, SIDE * -0.1f).plusInPlace(OFFSET));
        S.add(Vec3.of(SIDE * 0.8f, 0, SIDE * -0.2f).plusInPlace(OFFSET));
        S.add(Vec3.of(SIDE * 0.7f, 0, SIDE * -0.1f).plusInPlace(OFFSET));
        S.add(Vec3.of(SIDE * 0.6f, 0, SIDE * -0.2f).plusInPlace(OFFSET));
        finishPositions.add(S);

        //C
        C1.add(Vec3.of(SIDE * 0.9f, 0, SIDE * 0.3f).plusInPlace(OFFSET));
        C1.add(Vec3.of(SIDE * 0.75f, 0, SIDE * 0.15f).plusInPlace(OFFSET));
        C1.add(Vec3.of(SIDE * 0.6f, 0, SIDE * 0.3f).plusInPlace(OFFSET));
        finishPositions.add(C1);

        //I
        I.add(Vec3.of(SIDE * 0.9f, 0, SIDE * 0.5f).plusInPlace(OFFSET));
        I.add(Vec3.of(SIDE * 0.75f, 0, SIDE * 0.5f).plusInPlace(OFFSET));
        I.add(Vec3.of(SIDE * 0.6f, 0, SIDE * 0.5f).plusInPlace(OFFSET));
        finishPositions.add(I);


        for (int i = 0; i < finishPositions.size(); i++) {
            Vec3 center;
            float crowdRadius;
            if (i < 4) {
                center = Vec3.of(SIDE * 0.5f, 0, SIDE * (-0.5f + 4 * i / 10f));
            } else {
                center = Vec3.of(SIDE * -0.6f, 0, SIDE * (2f - 3f * i / 10f));
            }
            crowdRadius = SIDE * 0.15f;
            generateCrowd(center, finishPositions.get(i), crowdRadius);
        }

        PlainConfigurationSpace configurationSpace = new PlainConfigurationSpace(
                this,
                sphericalAgentDescriptions.get(0),
                sphericalObstacles
        );
        SphericalAgent.MILESTONE_REACHED_RADIUS = 20f;
        SphericalAgent.NEXT_MILESTONE_HINT_SIZE = 18f;
        ground = new Ground(this,
                OFFSET.plus(Vec3.of(0, SIDE * 0.08f, 0)),
                Vec3.of(0, 0, 1), Vec3.of(1, 0, 0),
                2 * SIDE, 2 * SIDE,
                loadImage("ground6.png"));
        MultiAgentGraph.END_POINT_SIZE = 20f;
        MultiAgentGraph.DRAW_VERTICES = false;
        MultiAgentGraph.DRAW_ENDS = false;
        MultiSphericalAgentSystem.MAX_EDGE_LEN = 50;
        multiSphericalAgentSystem = new MultiSphericalAgentSystem(this, sphericalAgentDescriptions, configurationSpace, minCorner, maxCorner);
        // tuning parameters
        SphericalAgent.IMPACT_RADIUS = 120f;
        SphericalAgent.SEPERATION_FORCE_BOID = 0.3f;
        SphericalAgent.SEPERATION_FORCE_OBSTACLE = 9f;
        SphericalAgent.ALIGNMENT_FORCE = 0.0f;
        SphericalAgent.CENTROID_FORCE = 0.0f;
        SphericalAgent.DRAW_PATH = false;
        SphericalAgent.DRAW_FUTURE_STATE = false;
        SphericalAgent.REPULSION = 0.000000001f;

        for (int i = 0; i < 8; i++) {
            PShape agentShape = loadShape("data/robot/" + (i + 1) + ".obj");
            agentShape.rotateX(PApplet.PI);
            agentShape.rotateY(PApplet.PI);
            agentWalkCycleShapes.add(agentShape);
        }

        buildFLock(finishPositions, multiSphericalAgentSystem);
    }


    private void buildFLock(List<List<Vec3>> finishPositions, MultiSphericalAgentSystem multiSphericalAgentSystem) {
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
//        multiSphericalAgentSystem.updateBoid(sphericalObstacles, 0.3f);
        multiSphericalAgentSystem.updateClan(flocks, sphericalObstacles, 0.3f);
        long update = millis();
        // draw
        background(0);
        directionalLight(0.6f, 0.6f, 0.6f, 0, 1, -1);
        directionalLight(0.6f, 0.6f, 0.6f, 0, 1, 1);
        pointLight(1, 1, 1, 0, -10, 0);
        // obstacles
        if (DRAW_OBSTACLES) {
            for (SphericalObstacle sphericalObstacle : sphericalObstacles) {
                sphericalObstacle.draw();
            }
        }
        // agent
        multiSphericalAgentSystem.draw(agentWalkCycleShapes, 5f);
//        checkFinish(multiSphericalAgentSystem);
        // ground
        ground.draw();
        // graph
//        graph.draw();
        long draw = millis();
        connectFinish(finishPositions);

        surface.setTitle("Processing - FPS: " + Math.round(frameRate) + " Update: " + (update - start) + "ms Draw " + (draw - update) + "ms" + " search: " + SEARCH_ALGORITHM + " smooth-path: " + SMOOTH_PATH);
    }

    private void generateCrowd(Vec3 center, List<Vec3> list, float crowdRadius) {
        for (int i = 0; i < list.size(); i++) {
            float theta = 0 + (i * PI) / list.size();
            float r = crowdRadius;
            Vec3 start = center.plus(Vec3.of(r * sin(theta), 0, r * cos(theta))).plusInPlace(OFFSET);
            SphericalAgentDescription description = new SphericalAgentDescription(
                    start,
                    list.get(i),
                    SIDE * 0.04f
            );
            sphericalAgentDescriptions.add(description);
        }

    }

    private void connectFinish(List<List<Vec3>> finishPositions) {
        for (List<Vec3> list : finishPositions) {
            int size = list.size();
            for (int i = 0; i < size - 1; i++) {
                Vec3 v1 = list.get(i);
                Vec3 v2 = list.get(i + 1);
                pushMatrix();
                stroke(0, 125, 0);
                strokeWeight(2);
                line(v1.x, v1.y, v1.z, v2.x, v2.y, v2.z);
                popMatrix();
            }
        }
    }


    private void checkFinish(MultiSphericalAgentSystem multiSphericalAgentSystem) {
        int i = 0;
        while (i < multiSphericalAgentSystem.sphericalAgents.size()) {
            SphericalAgent agent = multiSphericalAgentSystem.sphericalAgents.get(i);
            if (agent.hasReachedEnd()) {
                multiSphericalAgentSystem.sphericalAgents.remove(i);
            } else {
                i++;
            }
        }
    }

    public void keyPressed() {
        if (keyCode == RIGHT) {
            multiSphericalAgentSystem.stepForward();
        }
        if (keyCode == LEFT) {
            multiSphericalAgentSystem.stepBackward();
        }
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
            multiSphericalAgentSystem.togglePause();
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
        String[] appletArgs = new String[]{"demos.boids.CSCI5611"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
