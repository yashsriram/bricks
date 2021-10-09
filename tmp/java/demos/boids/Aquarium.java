package demos.boids;

import camera.QueasyCam;
import fixed.SphericalObstacle;
import math.Vec3;
import processing.core.PApplet;
import robot.acting.FreeAgent;
import robot.acting.MultiSphericalAgentSystem;

import java.util.ArrayList;
import java.util.List;

public class Aquarium extends PApplet {
    public static final int WIDTH = 800;
    public static final int HEIGHT = 800;
    public static final int SIDE = 100;
    public static boolean PAUSE = true ;
    final Vec3 minCorner = Vec3.of(-SIDE, -SIDE, -SIDE);
    final Vec3 maxCorner = Vec3.of(SIDE, SIDE, SIDE);

    List<SphericalObstacle> sphericalObstacles = new ArrayList<>();
    List<FreeAgent> freeAgents = new ArrayList<>() ;
    QueasyCam cam;

    static boolean DRAW_OBSTACLES = true;

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

        Vec3 center = Vec3.of(SIDE*0.0f, SIDE * 0.0f, SIDE * 0.0f);

        float agentRadius = SIDE * 0.020f;
        for(int i = 0 ; i < 25; i++){
            freeAgents.add(new FreeAgent(
                    Vec3.of(random(-SIDE, -SIDE*0.75f), random(-SIDE, -SIDE*0.75f), random(-SIDE, -SIDE*0.75f)),
                    Vec3.of(random(0, 1f), random(0, 1f), random(0, 1f)),
                    Vec3.of(random(0,1),random(0,1),random(0,1)),
                    agentRadius,
                    this,
                    minCorner,
                    maxCorner
            ));
        }
        for(int i = 0 ; i < 25; i++){
            freeAgents.add(new FreeAgent(
                    Vec3.of(random(SIDE*0.75f, SIDE), random(SIDE*0.75f, SIDE), random(SIDE*0.75f, SIDE)),
                    Vec3.of(random(-1, 0f), random(-1, 0f), random(-1, 0f)),
                    Vec3.of(random(0,1),random(0,1),random(0,1)),
                    agentRadius,
                    this,
                    minCorner,
                    maxCorner
            ));
        }

        // tuning parameters
        FreeAgent.IMPACT_RADIUS = 20f;
        FreeAgent.SEPARATION_FORCE_BOID = 0.01f;
        FreeAgent.SEPARATION_FORCE_OBSTACLE = 2f;
        FreeAgent.ALIGNMENT_FORCE = 0.3f;
        FreeAgent.CENTROID_FORCE = 0.05f;

    }

    public void draw() {
        long start = millis();
        background(255);
        pushMatrix();
        stroke(0);
        noFill();
        translate(0,0,0);
        box(2*(SIDE+10f),2*(SIDE+10f),2*(SIDE+10f));
        popMatrix();
        // update
        for(int i = 0 ; i < 10 ; i++){
            for(FreeAgent agent : freeAgents){
                agent.getForce(freeAgents, sphericalObstacles);
            }
            for(FreeAgent agent : freeAgents){
                agent.update(0.1f);
            }
        }

        // multiagent system

        long update = millis();
        // draw

        // obstacles
        if (DRAW_OBSTACLES) {
            for (SphericalObstacle sphericalObstacle : sphericalObstacles) {
                sphericalObstacle.draw();
            }
        }


        for(FreeAgent agent : freeAgents){
            agent.draw() ;
        }
//        multiSphericalAgentSystem.draw(wings);
        long draw = millis();

        surface.setTitle("Processing - FPS: " + Math.round(frameRate) + " Update: " + (update - start) + "ms Draw " + (draw - update) + "ms" );
    }

    public void keyPressed() {
        if(key == 'p'){
            PAUSE = !PAUSE ;
        }


    }



    static public void main(String[] passedArgs) {
        String[] appletArgs = new String[]{"demos.boids.Aquarium"};
        if (passedArgs != null) {
            PApplet.main(concat(appletArgs, passedArgs));
        } else {
            PApplet.main(appletArgs);
        }
    }
}
