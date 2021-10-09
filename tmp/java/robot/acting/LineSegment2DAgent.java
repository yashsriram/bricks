package robot.acting;

import math.Vec3;
import processing.core.PApplet;
import robot.input.LineSegment2DAgentDescription;
import robot.sensing.LineSegment2DConfigurationSpace;

import java.util.ArrayList;
import java.util.List;

public class LineSegment2DAgent {
    public static float NEXT_MILESTONE_HINT_SIZE = 2f;
    public static boolean DRAW_POSITION_ORIENTATION_SPACE_PATH = true;

    final PApplet parent;
    final LineSegment2DAgentDescription description;
    final LineSegment2DConfigurationSpace configurationSpace;
    final float speed;
    final Vec3 color;
    final Vec3 minCorner;
    final Vec3 maxCorner;

    Vec3 pose;
    List<Vec3> path = new ArrayList<>();
    int currentMilestone = 0;
    public boolean isPaused = false;

    public LineSegment2DAgent(final PApplet parent, final LineSegment2DAgentDescription description, final LineSegment2DConfigurationSpace configurationSpace, Vec3 minCorner, Vec3 maxCorner, float speed, Vec3 color) {
        this.parent = parent;
        this.description = description;
        this.configurationSpace = configurationSpace;
        this.speed = speed;
        this.color = color;
        this.minCorner = minCorner;
        this.maxCorner = maxCorner;

        this.pose = Vec3.of(description.startPose);
    }

    public void update(float dt) {
        if (isPaused) {
            return;
        }
        if (currentMilestone < path.size() - 1) {
            // reached next milestone
            if (path.get(currentMilestone + 1).minus(pose).norm() < 2) {
                currentMilestone++;
                pose.set(path.get(currentMilestone));
                return;
            }
            // move towards next milestone
            Vec3 velocityDir =
                    path.get(currentMilestone + 1)
                            .minus(pose)
                            .normalizeInPlace();
            pose.plusInPlace(velocityDir.scale(speed * dt));
        }
    }

    public void smoothUpdate(float dt) {
        if (isPaused) {
            return;
        }
        if (currentMilestone < path.size() - 1) {
            // reached next milestone
            if (path.get(currentMilestone + 1).minus(pose).norm() < 2) {
                currentMilestone++;
                pose.set(path.get(currentMilestone));
                return;
            }
            // next next milestone lookup
            if (currentMilestone < path.size() - 2) {
                boolean blocked = configurationSpace.doesEdgeIntersectSomeObstacleStricter(path.get(currentMilestone + 2), pose);
                if (!blocked) {
                    currentMilestone++;
                }
            }
            // move towards next milestone
            Vec3 velocityDir =
                    path.get(currentMilestone + 1)
                            .minus(pose)
                            .normalizeInPlace();
            pose.plusInPlace(velocityDir.scale(speed * dt));
        }
    }

    private void drawPose(Vec3 pose, Vec3 color) {
        parent.pushMatrix();
        parent.stroke(color.x, color.y, color.z);
        parent.fill(color.x, color.y, color.z);
        Vec3 halfLength = Vec3.of(0, (float) (Math.sin(pose.x / configurationSpace.orientationScale) * description.length / 2), (float) (Math.cos(pose.x / configurationSpace.orientationScale) * description.length / 2));
        Vec3 e1 = pose.minus(halfLength);
        Vec3 e2 = pose.plus(halfLength);
        parent.line(0, e1.y, e1.z, 0, e2.y, e2.z);
        parent.translate(0, pose.y, pose.z);
        parent.box(1f);
        parent.translate(0, halfLength.y, halfLength.z);
        parent.box(1f);
        parent.popMatrix();
    }

    public void draw() {
        // start pose
        drawPose(description.startPose, Vec3.of(0, 0, 1));
        // finish pose
        drawPose(description.finishPose, Vec3.of(0, 1, 0));
        // path
        for (int i = 0; i < path.size() - 1; i++) {
            Vec3 v1 = path.get(i);
            Vec3 v2 = path.get(i + 1);
            parent.stroke(color.x, color.y, color.z);
            parent.line(0, v1.y, v1.z, 0, v2.y, v2.z);
        }
        if (DRAW_POSITION_ORIENTATION_SPACE_PATH) {
            // position-orientation space path
            for (int i = 0; i < path.size() - 1; i++) {
                Vec3 v1 = path.get(i);
                Vec3 v2 = path.get(i + 1);
                parent.stroke(0, 0, 1);
                parent.line(v1.x, v1.y, v1.z, v2.x, v2.y, v2.z);
            }
        }
        parent.noStroke();
        // agent
        drawPose(pose, color);
        // next milestone
        if (currentMilestone < path.size() - 1) {
            Vec3 nextMilestonePosition = path.get(currentMilestone + 1);
            Vec3 nextMilestoneColor = Vec3.of(1, 0, 0);
            drawPose(nextMilestonePosition, nextMilestoneColor);
            if (DRAW_POSITION_ORIENTATION_SPACE_PATH) {
                parent.pushMatrix();
                parent.fill(nextMilestoneColor.x, nextMilestoneColor.y, nextMilestoneColor.z);
                parent.noStroke();
                parent.translate(nextMilestonePosition.x, nextMilestonePosition.y, nextMilestonePosition.z);
                parent.sphere(NEXT_MILESTONE_HINT_SIZE);
                parent.popMatrix();
            }
        }
    }

    public void setPath(List<Vec3> path) {
        this.path = path;
        currentMilestone = 0;
        pose.set(description.startPose);
    }

    public void stepForward() {
        if (path.size() == 0) {
            return;
        }
        pose.set(path.get(currentMilestone));
        if (currentMilestone < path.size() - 1) {
            currentMilestone++;
        }
    }

    public void stepBackward() {
        if (path.size() == 0) {
            return;
        }
        pose.set(path.get(currentMilestone));
        if (currentMilestone > 0) {
            currentMilestone--;
        }
    }

    public List<Vec3> samplePoints(int numberOfPoints) {
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
