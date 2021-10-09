package robot.planning.replanninggraph;

import math.Vec3;
import processing.core.PApplet;
import processing.core.PConstants;

import java.util.ArrayList;
import java.util.List;

public class Vertex {
    private static final int START_ID = -1;
    private static final int FINISH_ID = 0;

    private static int nextId = 1;

    private static int getNextId() {
        int currId = nextId;
        nextId++;
        return currId;
    }

    private final PApplet parent;
    private final int id;
    public final Vec3 position;
    final float heuristicDistanceToFinish;

    boolean isOutsideObstacle = true;
    public boolean isSensed = false;
    final List<Vertex> neighbours = new ArrayList<>();
    final List<Vec3> edgeColors = new ArrayList<>();

    class SearchState {
        boolean isExplored = false;
        float distanceFromStart = 0;
        List<Vertex> pathFromStart = new ArrayList<>();
        private Vec3 color = Vec3.of(1);

        void reset() {
            isExplored = false;
            distanceFromStart = 0;
            pathFromStart.clear();
            if (Vertex.this.isOutsideObstacle) {
                if (Vertex.this.isSensed) {
                    color.set(1, 1, 0);
                } else {
                    color.set(1, 1, 1);
                }
            } else {
                color.set(1, 0, 1);
            }
        }

        void addToFringeFrom(Vertex parent) {
            isExplored = true;
            pathFromStart.addAll(parent.searchState.pathFromStart);
            pathFromStart.add(Vertex.this);
            color.set(0, 1, 0);
        }

        void setExplored() {
            color.set(1, 0, 0);
        }

        void setSensed() {
            color.set(1, 1, 0);
        }

        void setInsideObstacle() {
            color.set(1, 0, 1);
        }
    }

    final SearchState searchState;

    static Vertex start(PApplet parent, Vec3 position, float distanceToFinish) {
        return new Vertex(parent, START_ID, position, distanceToFinish);
    }

    static Vertex finish(PApplet parent, Vec3 position, float distanceToFinish) {
        return new Vertex(parent, FINISH_ID, position, distanceToFinish);
    }

    static Vertex of(PApplet parent, Vec3 position, float distanceToFinish) {
        return new Vertex(parent, getNextId(), position, distanceToFinish);
    }

    private Vertex(PApplet parent, int id, Vec3 position, float heuristicDistanceToFinish) {
        this.parent = parent;
        this.id = id;
        this.position = Vec3.of(position);
        this.heuristicDistanceToFinish = heuristicDistanceToFinish;
        this.searchState = new SearchState();
    }

    void draw() {
        parent.pushMatrix();
        parent.fill(searchState.color.x, searchState.color.y, searchState.color.z);
        parent.stroke(searchState.color.x, searchState.color.y, searchState.color.z);
        parent.beginShape(PConstants.TRIANGLE);
        parent.vertex(position.x, position.y, position.z);
        parent.vertex(position.x, position.y + 1, position.z + 1);
        parent.vertex(position.x, position.y, position.z + 1);
        parent.endShape();
        parent.popMatrix();
        if (ReplanningGraph.DRAW_EDGES) {
            for (int i = 0; i < neighbours.size(); ++i) {
                Vertex neighbour = neighbours.get(i);
                Vec3 color = edgeColors.get(i);
                parent.stroke(color.x, color.y, color.z);
                parent.line(this.position.x, this.position.y, this.position.z,
                        neighbour.position.x, neighbour.position.y, neighbour.position.z);
            }
        }
    }

    void addNeighbour(Vertex other, Vec3 color) {
        neighbours.add(other);
        edgeColors.add(color);
    }

    void removeNeighbour(Vertex neighbour) {
        int indexToRemove = -1;
        for (int i = 0; i < neighbours.size(); i++) {
            if (neighbours.get(i).id == neighbour.id) {
                indexToRemove = i;
                break;
            }
        }
        // remove this from the neighbour's neighbour list
        int indexToRemoveInNeighbour = -1;
        List<Vertex> neighboursOfToBeRemovedNeighbour = neighbours.get(indexToRemove).neighbours;
        for (int i = 0; i < neighboursOfToBeRemovedNeighbour.size(); i++) {
            if (neighboursOfToBeRemovedNeighbour.get(i).id == this.id) {
                indexToRemoveInNeighbour = i;
                break;
            }
        }
        neighbours.get(indexToRemove).neighbours.remove(indexToRemoveInNeighbour);
        neighbours.remove(indexToRemove);
    }

    void setSensed() {
        isSensed = true;
        searchState.setSensed();
    }

    void setInsideObstacle() {
        isOutsideObstacle = false;
        searchState.setInsideObstacle();
    }

    boolean isFinishVertex() {
        return id == FINISH_ID;
    }

    @Override
    public String toString() {
        return "Vertex{" +
                "id=" + id +
                '}';
    }
}
