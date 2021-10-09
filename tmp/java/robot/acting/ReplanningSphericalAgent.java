package robot.acting;

import math.Vec3;
import processing.core.PApplet;
import robot.input.SphericalAgentDescription;
import robot.planning.replanninggraph.ReplanningGraph;
import robot.planning.replanninggraph.Vertex;
import robot.sensing.ConfigurationSpace;

import java.util.ArrayList;
import java.util.List;

public class ReplanningSphericalAgent {
    public enum Algorithm {
        DFS, BFS, UCS, AStar, WeightedAStar
    }

    public static float MILESTONE_REACHED_RADIUS = 2f;
    // Should be big enough such that next milestone lies inside sense radius and is therefore sensed
    public static float SENSE_RADIUS = 20f;

    final PApplet parent;
    final SphericalAgentDescription description;
    final ConfigurationSpace configurationSpace;
    final float speed;
    final Vec3 color;
    final Vec3 minCorner;
    final Vec3 maxCorner;
    public final ReplanningGraph replanningGraph;
    public Algorithm algorithm;

    Vec3 center;
    List<Vertex> path = new ArrayList<>();
    int currentMilestone = 0;
    float distanceCovered = 0;
    public boolean isPaused = false;

    public ReplanningSphericalAgent(final PApplet parent,
                                    final SphericalAgentDescription description,
                                    final ConfigurationSpace configurationSpace,
                                    Vec3 minCorner, Vec3 maxCorner,
                                    float speed,
                                    Vec3 color,
                                    int numSamples,
                                    float maxEdgeLen,
                                    Algorithm algorithm) {
        this.parent = parent;
        this.description = description;
        this.configurationSpace = configurationSpace;
        this.speed = speed;
        this.color = color;
        this.minCorner = minCorner;
        this.maxCorner = maxCorner;
        this.replanningGraph = new ReplanningGraph(parent, description.startPosition, description.finishPosition);
        replanningGraph.generateGraph(samplePoints(numSamples), maxEdgeLen);
        this.algorithm = algorithm;

        this.center = Vec3.of(description.startPosition);
        this.path.add(replanningGraph.start);
    }

    public void update(float dt) {
        if (isPaused) {
            return;
        }
        if (!path.get(currentMilestone).isSensed) {
            replan();
            return;
        }
        if (currentMilestone < path.size() - 1) {
            // proceed only if next milestone sensed
            if (!path.get(currentMilestone + 1).isSensed) {
                replan();
                return;
            }
            // reached next milestone
            if (path.get(currentMilestone + 1).position.minus(center).norm() < MILESTONE_REACHED_RADIUS) {
                currentMilestone++;
                return;
            }
            // move towards next milestone
            Vec3 velocityDir =
                    path.get(currentMilestone + 1)
                            .position
                            .minus(center)
                            .normalizeInPlace();
            Vec3 displacement = velocityDir.scaleInPlace(speed * dt);
            center.plusInPlace(displacement);
            distanceCovered += displacement.norm();
        }
    }

    public void smoothUpdate(float dt) {
        if (isPaused) {
            return;
        }
        if (!path.get(currentMilestone).isSensed) {
            replan();
            return;
        }
        if (currentMilestone < path.size() - 1) {
            // proceed only if next milestone sensed
            if (!path.get(currentMilestone + 1).isSensed) {
                replan();
                return;
            }
            // reached next milestone
            if (path.get(currentMilestone + 1).position.minus(center).norm() < MILESTONE_REACHED_RADIUS) {
                currentMilestone++;
                return;
            }
            // next next milestone lookup
            if (currentMilestone < path.size() - 2) {
                if (path.get(currentMilestone + 2).isSensed) {
                    boolean blocked = configurationSpace.doesEdgeIntersectSomeObstacle(path.get(currentMilestone + 2).position, center);
                    if (!blocked) {
                        currentMilestone++;
                    }
                }
            }
            // move towards next milestone
            Vec3 velocityDir =
                    path.get(currentMilestone + 1)
                            .position
                            .minus(center)
                            .normalizeInPlace();
            Vec3 displacement = velocityDir.scaleInPlace(speed * dt);
            center.plusInPlace(displacement);
            distanceCovered += displacement.norm();
        }
    }

    public void draw() {
        // graph
        replanningGraph.draw();
        // path
        parent.stroke(color.x, color.y, color.z);
        for (int i = 0; i < path.size() - 1; i++) {
            Vec3 v1 = path.get(i).position;
            Vec3 v2 = path.get(i + 1).position;
            parent.line(v1.x, v1.y, v1.z, v2.x, v2.y, v2.z);
        }
        parent.noStroke();
        // agent
        parent.pushMatrix();
        parent.fill(color.x, color.y, color.z);
        parent.translate(center.x, center.y, center.z);
        parent.sphere(description.radius);
        parent.popMatrix();
        // next milestone
        if (currentMilestone < path.size() - 1) {
            Vec3 nextMilestonePosition = path.get(currentMilestone + 1).position;
            parent.pushMatrix();
            parent.fill(1, 0, 0);
            parent.translate(nextMilestonePosition.x, nextMilestonePosition.y, nextMilestonePosition.z);
            parent.sphere(description.radius);
            parent.popMatrix();
        }
    }

    private void replan() {
        boolean obstaclesDetected = replanningGraph.senseAndUpdate(center, SENSE_RADIUS, configurationSpace);
        if (!obstaclesDetected && path.size() > 1) {
            return;
        }
        switch (algorithm) {
            case DFS:
                path = replanningGraph.dfs(path.get(currentMilestone));
                break;
            case BFS:
                path = replanningGraph.bfs(path.get(currentMilestone));
                break;
            case UCS:
                path = replanningGraph.ucs(path.get(currentMilestone));
                break;
            case AStar:
                path = replanningGraph.aStar(path.get(currentMilestone));
                break;
            case WeightedAStar:
                path = replanningGraph.weightedAStar(path.get(currentMilestone), 1.5f);
                break;
        }
        currentMilestone = 0;
    }

    public void stepForward() {
        if (path.size() == 0) {
            return;
        }
        center.set(path.get(currentMilestone).position);
        if (currentMilestone < path.size() - 1) {
            currentMilestone++;
        }
    }

    public void stepBackward() {
        if (path.size() == 0) {
            return;
        }
        center.set(path.get(currentMilestone).position);
        if (currentMilestone > 0) {
            currentMilestone--;
        }
    }

    private List<Vec3> samplePoints(int numberOfPoints) {
        List<Vec3> samples = new ArrayList<>();
        for (int i = 0; i < numberOfPoints; i++) {
            samples.add(Vec3.of(
                    parent.random(minCorner.x, maxCorner.x),
                    parent.random(minCorner.y, maxCorner.y),
                    parent.random(minCorner.z, maxCorner.z)
            ));
        }
        return samples;
    }

}
