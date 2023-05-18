use eki::Connection;
use eki::node::Node;
use eki::nodes::{ pressure::Pressure, flow::Flow };
use eki::edge::Edge;
use eki::edges::{ pipe::Pipe };
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


/* TRIAL TESTS - SEM */

#[test] //Trial-Sem
fn add_node_edge_2() {
    let mut graph = Graph::new();

    let node_from = Node::Flow( Flow::new( 1 ) );
    graph.add_node( node_from.clone() );

    let node_mid = Node::Connection( Connection::new( 2 ) );
    graph.add_node( node_mid.clone() );

    let node_to = Node::Pressure( Pressure::new( 3 ) );
    graph.add_node( node_to.clone() );

    assert_eq!( graph.num_nodes(), 3 );
    
    let edge_pipe = Edge::Pipe( Pipe::new( node_from, node_mid.clone() ) );
    graph.add_edge( edge_pipe );

    let edge_valve = Edge::Pipe( Pipe::new( node_mid, node_to ) );
    graph.add_edge( edge_valve );

    assert_eq!( graph.num_edges(), 2 );

    graph.create_id_to_index();

    let mut k_matrix = Mat64::new( 2, 3, 0.0 );
    k_matrix[0][0] = 1.0;
    k_matrix[0][1] = -1.0;
    k_matrix[1][1] = 1.0;
    k_matrix[1][2] = -1.0;
    assert_eq!( k_matrix.rows(), graph.k_matrix().rows() );
    assert_eq!( k_matrix.cols(), graph.k_matrix().cols() );
    for i in 0..1 {
        for j in 0..2 {
            assert_eq!( k_matrix[i][j], graph.k_matrix()[i][j] );
        }
    }
}