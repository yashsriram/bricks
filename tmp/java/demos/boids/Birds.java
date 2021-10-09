package demos.boids;

import camera.QueasyCam;
import fixed.SphericalObstacle;
import math.Vec3;
import processing.core.PApplet;
import processing.core.PShape;
import robot.acting.MultiSphericalAgentSystem;
import robot.acting.SphericalAgent;
import robot.input.SphericalAgentDescription;
import robot.planning.multiagentgraph.MultiAgentGraph;
import robot.sensing.ConfigurationSpace;
import robot.sensing.PlainConfigurationSpace;

import java.util.ArrayList;
import java.util.List;

public class Birds extends PApplet {
    public static final int WIDTH = 1200;
    public static final int HEIGHT = 1200;
    public static final int SIDE = 100;
    final Vec3 minCorner = Vec3.of(0, -SIDE, -SIDE);
    final Vec3 maxCorner = Vec3.of(0, SIDE, SIDE);

    List<SphericalObstacle> sphericalObstacles = new ArrayList<>();
    MultiSphericalAgentSystem multiSphericalAgentSystem;
    QueasyCam cam;
    List<PShape> wings;
    boolean reset = false ;
    final int numFlocks = 2 ;

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
        float radiusFactor = 0.04f;
        float obstacleRadius = SIDE * radiusFactor;
        Vec3 center = Vec3.zero();
        float extent = 0.9f;
        float radii = SIDE * extent;
        float theta = 2 * PApplet.PI;
        int counter = 1;
        float divider = 60f;
        int openings = 3;
        int sideToggler = 4;
        float depth = 0;
        sphericalObstacles.add(new SphericalObstacle(
                this,
                Vec3.of(0, center.y + radii * sin(theta*(58.0f/60f)), center.z + radii * cos(theta*(58.0f/60f))),
                obstacleRadius,
                Vec3.of(1, 0, 1)
        ));
        sphericalObstacles.add(new SphericalObstacle(
                this,
                Vec3.of(0, center.y + radii * sin(theta*(57.0f/60f)), center.z + radii * cos(theta*(57.0f/60f))),
                obstacleRadius,
                Vec3.of(1, 0, 1)
        ));
        while (radii > SIDE * 0.2f) {
            sphericalObstacles.add(new SphericalObstacle(
                    this,
                    Vec3.of(0, center.y + radii * sin(theta), center.z + radii * cos(theta)),
                    obstacleRadius,
                    Vec3.of(1, 0, 1)
            ));

            theta += (2 * PApplet.PI) / divider;
            counter += 1;
            if (divider - counter == openings) {
                if (extent > 0.6) {
                    extent -= 0.3;
                } else {
                    extent -= 0.2;
                }
                radii = SIDE * extent;
                counter = 0;
                divider -= 4;
                depth += 0.3f;
                if (sideToggler % 2 == 0) {
                    theta = PApplet.PI;
                    sideToggler -= 1;
//                    openings -= 1 ;
                } else {
                    theta = 0f;
                    sideToggler -= 1;
                }
            }
        }
        List<SphericalAgentDescription> sphericalAgentDescriptions = new ArrayList<>();
        center = Vec3.of(0, SIDE * 0.95f, SIDE * -0.7f);
        for (int k = 0; k < numFlocks; k++) {
            float agentRadius = SIDE * 0.015f;
            float slack = 8;
            int numAgentsRadially = 3;
            int numCircleDivisions = 10;
            Vec3 finishPosition = Vec3.of(0.0f * SIDE, 0, 0);
            for (int i = 0; i < numCircleDivisions; i++) {
                theta = 2 * PI / numCircleDivisions * i;
                for (int j = 0; j < numAgentsRadially; j++) {
                    float radialDistance = j * 2 * agentRadius + slack;
                    sphericalAgentDescriptions.add(new SphericalAgentDescription(
                            center.plus(Vec3.of(0, (float) Math.sin(theta), (float) Math.cos(theta)).scaleInPlace(radialDistance)),
                            finishPosition,
                            agentRadius
                    ));
                }
            }
            center.scaleInPlace(-1);

        }


        wings = new ArrayList<>();
        PShape model = loadShape("data/3d-model.obj");
        model.rotateX(PApplet.PI);
        model.rotateY(PApplet.PI);
        model.scale(0.01f);
        wings.add(model);

        ConfigurationSpace configurationSpace = new PlainConfigurationSpace(this, sphericalAgentDescriptions.get(0), sphericalObstacles);
        multiSphericalAgentSystem = new MultiSphericalAgentSystem(this, sphericalAgentDescriptions, configurationSpace, minCorner, maxCorner, 2);
        MultiSphericalAgentSystem.INITIAL_AGENT_SPEED = 10f;

        MultiAgentGraph.DRAW_ENDS = false;
        MultiAgentGraph.DRAW_VERTICES = false;

        // tuning parameters
        SphericalAgent.IMPACT_RADIUS = 10f;
        SphericalAgent.SEPERATION_FORCE_BOID = 4f;
        SphericalAgent.SEPERATION_FORCE_OBSTACLE = 6f;
        SphericalAgent.ALIGNMENT_FORCE = 0.2f;
        SphericalAgent.CENTROID_FORCE = 20f;
        SphericalAgent.DRAW_FUTURE_STATE = false;
        SphericalAgent.DRAW_PATH = false;
    }

    public void draw() {
        long start = millis();
        // update
        for(int i = 0 ; i < 10 ; i++){
            multiSphericalAgentSystem.updateBoid(sphericalObstacles, 0.01f);
        }

        // multiagent system
        for (int i = 0; i < multiSphericalAgentSystem.sphericalAgents.size(); i++) {
            SphericalAgent agent = multiSphericalAgentSystem.sphericalAgents.get(i);
            agent.draw();
        }
        checkFlockStatus() ;
        long update = millis();
        // draw
        background(0);
        // obstacles
        if (DRAW_OBSTACLES) {
            for (SphericalObstacle sphericalObstacle : sphericalObstacles) {
                sphericalObstacle.draw();
            }
        }

        multiSphericalAgentSystem.drawBox();
//        multiSphericalAgentSystem.draw(wings);
        long draw = millis();

        surface.setTitle("Processing - FPS: " + Math.round(frameRate) + " Update: " + (update - start) + "ms Draw " + (draw - update) + "ms" + " search: " + SEARCH_ALGORITHM);
    }

    private void checkFlockStatus() {
        float reached = 0 ;
        float div = (float)multiSphericalAgentSystem.sphericalAgents.size()/(float)numFlocks;
        float start = 0 ;
        float end = div-1 ;
        for(int i = 0 ; i < multiSphericalAgentSystem.sphericalAgents.size(); i++){
            SphericalAgent agent = multiSphericalAgentSystem.sphericalAgents.get(i) ;
            if(agent.hasReachedEnd()){
                reached += 1 ;
            }
            if(i == end){
                if(reached/div > 0.6){
                    resetFlock(start, end) ;
                }
                start = end + 1 ;
                end += div ;
                reached = 0 ;
            }

        }

    }

    private void resetFlock(float start, float end) {
        for(float i = start; i <= end; i++){
            SphericalAgent agent = multiSphericalAgentSystem.sphericalAgents.get((int) i) ;
            agent.reset();
        }
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

        if (key == '4') {
            multiSphericalAgentSystem.aStar();
            SEARCH_ALGORITHM = "A*";
        }

    }

    static public void main(String[] passedArgs) {
        String[] appletArgs = new String[]{"demos.boids.Birds"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }

}
