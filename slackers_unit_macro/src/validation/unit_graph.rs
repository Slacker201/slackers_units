use std::collections::HashMap;

use proc_macro2::Span;
use syn::Error;

use crate::{
    data::{
        Decimal, Id,
        parser_data::{EqualityExpression, System, UnitDefinition},
    },
    validation::unit_graph::graph_context::GraphContext,
};

pub(crate) mod graph_context;

impl System {
    pub(crate) fn reduce_unit_equations(
        &self,
        gc: &GraphContext,
    ) -> syn::Result<(HashMap<String, Id>, HashMap<Id, (Decimal, Box<[i32]>)>)> {
        let mut alias_to_id = HashMap::new();
        let mut id_to_node = HashMap::new();
        for unit in &self.units {
            for alias in &unit.aliases.aliases {
                alias_to_id.insert(alias.to_string(), unit.id);
            }
            id_to_node.insert(unit.id, unit);
        }
        let graph = Graph::new(&self.units, gc)?;
        Ok((alias_to_id, graph.reduce()))
    }
}
#[derive(Debug, Clone)]
struct Graph<'a> {
    nodes: HashMap<Id, GraphNode<'a>>,
}

impl<'a> Graph<'a> {
    fn new(units: &'a [UnitDefinition<Id>], gc: &GraphContext) -> syn::Result<Self> {
        let mut nodes = HashMap::new();
        for unit in units {
            let mut node = GraphNode::new(unit);
            if let Some(v) = gc.base_to_vector_mapping.get(&node.id) {
                node.vmult = Some((1.0, v.clone()));
            }
            nodes.insert(node.id, node);
        }

        let mut s = Graph { nodes };
        s.assert_acylic()?;
        s.flatten_dependancies();
        Ok(s)
    }
    fn assert_acylic(&self) -> syn::Result<()> {
        for node in self.nodes.values() {
            let dependees = node.get_dependees(&self.nodes);
            if dependees.contains(&node.id) {
                return Err(Error::new(node.span, "Found cyclic dependancy"));
            }
        }

        Ok(())
    }
    fn flatten_dependancies(&mut self) {
        let mut ids_to_flatten: Vec<Id> = Vec::from_iter(self.nodes.keys())
            .into_iter()
            .copied()
            .collect();
        ids_to_flatten.sort();

        while let Some(self_id) = ids_to_flatten.pop() {
            match self
                .nodes
                .get(&self_id)
                .expect("This should always be a valid id")
                .try_flatten(&self.nodes)
            {
                Ok(vec) => {
                    self.nodes
                        .get_mut(&self_id)
                        .expect(&format!("line {} col {}", line!(), column!()))
                        .vmult = Some(vec);
                }
                Err(id) => {
                    ids_to_flatten.push(self_id);
                    ids_to_flatten.push(id);
                }
            }
        }
    }
    fn reduce(self) -> HashMap<Id, (Decimal, Box<[i32]>)> {
        let mut map = HashMap::new();
        for (id, node) in self.nodes {
            map.insert(id, node.vmult.expect("All vectors should be resolved"));
        }
        map
    }
}

#[derive(Debug, Clone)]
pub struct GraphNode<'a> {
    pub id: Id,
    pub eq: Option<&'a EqualityExpression<Id>>,
    pub edges: Box<[Id]>,
    pub vmult: Option<(Decimal, Box<[i32]>)>,

    pub span: Span,
}

impl<'a> GraphNode<'a> {
    fn new(ud: &'a UnitDefinition<Id>) -> Self {
        Self {
            id: ud.id,
            vmult: None,
            edges: ud
                .equality
                .as_ref()
                .map(|eq| eq.get_children())
                .unwrap_or(Box::default()),
            eq: ud.equality.as_ref(),
            span: ud.span,
        }
    }

    fn get_dependees(&self, nodes: &HashMap<Id, GraphNode>) -> Box<[Id]> {
        if self.eq.is_none() {
            return Box::default();
        };
        let mut dependees = vec![];
        for edge in &self.edges {
            nodes.get(&edge).map(|node| node.gd(nodes, &mut dependees));
        }

        dependees.into_boxed_slice()
    }

    fn gd(&self, nodes: &HashMap<Id, GraphNode>, dependees: &mut Vec<Id>) {
        if dependees.contains(&self.id) {
            return;
        }

        dependees.push(self.id);

        if self.eq.is_none() {
            return;
        }

        for edge in &self.edges {
            nodes.get(&edge).map(|node| node.gd(nodes, dependees));
        }
    }
    fn try_flatten(&self, nodes: &HashMap<Id, GraphNode>) -> Result<(Decimal, Box<[i32]>), Id> {
        let Some(eq) = self.eq else {
            return Ok(self.vmult.as_ref().cloned().expect("UNREACHABLE"));
        };

        eq.try_get_vmult(nodes)
    }
}
