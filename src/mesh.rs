// Add ability for vertex to have different properties like color, position, velocity, acceleration, etc
// TODO: What order is our convention? struct, trait impls, other impls?
// How to make sure that Face and Polyhedron are valid

#[derive(Debug)]
struct Mesh<'vertex: 'edge, 'edge: 'triangle, 'triangle, T>
{
    vertices: std::vec::Vec<Vertex<T>>,
    edges: std::vec::Vec<Edge<'vertex, T>>,
    triangles: std::vec::Vec<Triangle<'vertex, 'edge, T>>,
    tetrahedra: std::vec::Vec<Tetrahedron<'vertex, 'edge, 'triangle, T>>,
}

#[derive(Debug, PartialEq)]
struct Vertex<T>
{
    position: std::vec::Vec<T>,
}

impl<'a: 'b, 'b, T> Vertex<T>
{
    // Vertices can outlive an edge created with them, but not the reverse
    pub fn join(&'a self, other_vertex: &'a Vertex<T>) -> Edge<'b, T>
    {
        Edge {
            vertices: [self, other_vertex],
        }
    }
}

#[derive(Debug, PartialEq)]
// Edge lives only as long as its vertices do
struct Edge<'a, T>
{
    vertices: [&'a Vertex<T>; 2],
}

impl<'a, T> Edge<'a, T>
{
    pub fn new(vertex1: &'a Vertex<T>, vertex2: &'a Vertex<T>) -> Self
    {
        Edge {
            vertices: [vertex1, vertex2],
        }
    }
}

#[derive(Debug, PartialEq)]
struct Triangle<'a: 'b, 'b, T>
{
    edges: [&'b Edge<'a, T>; 3],
}

impl<'a: 'b, 'b, T> Triangle<'a, 'b, T>
{
    pub fn from_vertices(
        vertex1: &'a Vertex<T>,
        vertex2: &'a Vertex<T>,
        vertex3: &'a Vertex<T>,
    ) -> Self
    {
        Triangle {
            edges: [
                &Edge {
                    vertices: [vertex1, vertex2],
                },
                &Edge {
                    vertices: [vertex2, vertex3],
                },
                &Edge {
                    vertices: [vertex3, vertex1],
                },
            ],
        }
    }

    pub fn from_edges(
        edge1: &'b Edge<'a, T>,
        edge2: &'b Edge<'a, T>,
        edge3: &'b Edge<'a, T>,
    ) -> Self
    {
        Triangle {
            edges: [edge1, edge2, edge3],
        }
    }
}

#[derive(Debug, PartialEq)]
struct Tetrahedron<'a: 'b, 'b: 'c, 'c, T>
{
    faces: [&'c Triangle<'a, 'b, T>; 4],
}

#[test]
fn check_join()
{
    let v1 = Vertex::<f64> {
        position: std::vec![0., 1.],
    };
    let v2 = Vertex::<f64> {
        position: std::vec![2., 3.],
    };

    let test_edge1 = v1.join(&v2);
    let test_edge2 = Edge::new(&v1, &v2);

    println!(
        "Checking both ways of edge creation are the same {:?} {:?}",
        &test_edge1, &test_edge2
    );
    assert_eq!(test_edge1, test_edge2);
}
