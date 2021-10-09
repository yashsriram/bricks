package robot.acting;

import fixed.SphericalObstacle;
import math.Vec3;
import processing.core.PApplet;
import processing.core.PConstants;

import java.util.List;

public class FreeAgent {
    public static float SEPARATION_FORCE_BOID = 0.5f;
    public static float SEPARATION_FORCE_OBSTACLE = 1f;
    public static float ALIGNMENT_FORCE = 0.02f;
    public static float CENTROID_FORCE = 0.05f;
    public static float IMPACT_RADIUS = 10f;

    Vec3 center ;
    Vec3 velocity ;
    Vec3 color;
    float radius;
    Vec3 force;
    PApplet parent;
    final Vec3 minCorner;
    final Vec3 maxCorner;

    public FreeAgent(Vec3 center, Vec3 velocity, Vec3 color, float radius, PApplet parent, Vec3 minCorner, Vec3 maxCorner) {
        this.center = center;
        this.velocity = velocity;
        this.color = color;
        this.radius = radius;
        this.parent = parent;
        this.minCorner = minCorner;
        this.maxCorner = maxCorner;
        this.force = Vec3.zero();
    }

    public void update(float dt){
        this.velocity.plusInPlace(force) ;
        this.center.plusInPlace(this.velocity.scale(dt));
        if(this.center.x < minCorner.x){
            this.center.x = minCorner.x ;
            this.velocity.x *= -1f ;
        }
        if(this.center.x > maxCorner.x){
            this.center.x = maxCorner.x ;
            this.velocity.x *= -1f ;
        }
        if(this.center.y < minCorner.y){
            this.center.y = minCorner.y ;
            this.velocity.y *= -1f ;
        }
        if(this.center.y > maxCorner.y){
            this.center.y = maxCorner.y ;
            this.velocity.y *= -1f ;
        }
        if(this.center.z < minCorner.z){
            this.center.z = minCorner.z ;
            this.velocity.z *= -1f ;
        }
        if(this.center.z > maxCorner.z){
            this.center.z = maxCorner.z ;
            this.velocity.z *= -1f ;
        }


    }

    public void getForce(List<FreeAgent> flock, List<SphericalObstacle> obstacles){
        Vec3 boidForce = getBoidForce(flock, obstacles) ;
        force = boidForce ;
    }

    private Vec3 getBoidForce(List<FreeAgent> flock, List<SphericalObstacle> obstacles) {
        Vec3 separationForce = Vec3.zero();
        Vec3 centroid = Vec3.zero();
        Vec3 alignment = Vec3.zero();
        Vec3 alignmentForce = Vec3.zero();
        Vec3 centroidForce = Vec3.zero();
        float neighbors = 0 ;
        for (FreeAgent agent : flock) {
            Vec3 force = this.center.minus(agent.center);
            float distance = force.norm();
            if (distance < IMPACT_RADIUS && distance > 0) {
                neighbors += 1 ;
                force.normalizeInPlace();
                separationForce.plusInPlace(force.scaleInPlace(SEPARATION_FORCE_BOID * (IMPACT_RADIUS - distance)));
                centroid.plusInPlace(agent.center);
                Vec3 udir = agent.velocity;
                alignment.plusInPlace(udir);
            }
        }

        if (neighbors > 0){
            centroid.scaleInPlace(1f / neighbors);
            alignment.normalizeInPlace();
            Vec3 mydir = this.velocity.normalize() ;
            alignmentForce = alignment.minus(mydir);
            alignmentForce.scaleInPlace(ALIGNMENT_FORCE);
            centroidForce = centroid.minus(this.center);
            centroidForce.normalizeInPlace().scaleInPlace(CENTROID_FORCE);
        }

        Vec3 finalForce = separationForce.plus(centroidForce.plus(alignmentForce));
        Vec3 obstacleAvoidanceForce = Vec3.zero();
        for (SphericalObstacle obstacle : obstacles) {
            Vec3 force = this.center.minus(obstacle.center);
            float distance = force.norm();
            if (distance < IMPACT_RADIUS + obstacle.radius) {
                force.normalizeInPlace();
                obstacleAvoidanceForce.plusInPlace(force.scale(SEPARATION_FORCE_OBSTACLE));
            }
        }

        finalForce.plusInPlace(obstacleAvoidanceForce);
        return finalForce;
    }

    public void draw(){
        parent.pushMatrix();
        parent.translate(center.x, center.y, center.z);
        parent.stroke(125,0,0);
        parent.stroke(color.x, color.y, color.z);
        parent.fill(color.x, color.y, color.z);
        parent.sphere(radius);
        parent.popMatrix();
    }
}
