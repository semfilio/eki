use eki::node::Node;
use eki::nodes::{ pressure::Pressure, flow::Flow, connection::Connection };
use eki::edge::Edge;
use eki::edges::{ pipe::Pipe, valve::Valve };
use eki::graph::Graph;
use ohsl::Mat64;

#[test]
fn new() {
    let graph = Graph::new();
    assert_eq!( graph.num_nodes(), 0 );
    assert_eq!( graph.num_edges(), 0 );
}

#[test]
fn add_node_edge() {
    let mut graph = Graph::new();
    let node_from = Node::Pressure( Pressure::new( 0 ) );
    graph.add_node( node_from.clone() );
    let node_to = Node::Flow( Flow::new( 1 ) );
    graph.add_node( node_to.clone() );
    assert_eq!( graph.num_nodes(), 2 );
    let edge = Edge::Pipe( Pipe::new( node_from, node_to ) );
    graph.add_edge( edge );
    assert_eq!( graph.num_edges(), 1 );
    graph.create_id_to_index();
    let mut k_matrix = Mat64::new( 1, 2, 0.0 );
    k_matrix[0][0] = 1.0;
    k_matrix[0][1] = -1.0;
    assert_eq!( k_matrix.rows(), graph.k_matrix().rows() );
    assert_eq!( k_matrix.cols(), graph.k_matrix().cols() );
    for i in 0..1 {
        for j in 0..2 {
            assert_eq!( k_matrix[i][j], graph.k_matrix()[i][j] );
        }
    }
}