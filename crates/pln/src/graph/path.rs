use crate::graph::search::SpanningTreeView;
use crate::StateSpace;

#[derive(Debug)]
pub struct Path<SS: StateSpace> {
    pub(crate) vertices: Vec<SS::State>,
}

impl<S: StateSpace> Path<S> {
    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}

impl<'a, SS: StateSpace> From<&SpanningTreeView<'a, SS>> for Path<SS> {
    fn from(ts: &SpanningTreeView<'a, SS>) -> Path<SS> {
        let vertices = match ts.path_to_stop() {
            None => vec![],
            Some(path) => path
                .into_iter()
                .map(|idx| ts.graph.vertices[idx].state)
                .collect(),
        };
        Path { vertices }
    }
}
