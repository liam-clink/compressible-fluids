// Add ability for vertex to have different properties like color, position, velocity, acceleration, etc
// TODO: What order is our convention? struct, trait impls, other impls?
// How to make sure that Face and Polyhedron are valid

#[derive(Debug)]
pub struct Mesh<T>
{
    vertices: std::vec::Vec<Vertex<T>>,
    edges: std::vec::Vec<Edge>,
    triangles: std::vec::Vec<Triangle>,
    tetrahedra: std::vec::Vec<Tetrahedron>,
}

impl<T> Mesh<T>
{
    pub fn new() -> Self
    {
        Mesh {
            vertices: Default::default(),
            edges: Default::default(),
            triangles: Default::default(),
            tetrahedra: Default::default(),
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex<T>) -> usize
    {
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    pub fn edge_from_vertices(&mut self, vertex1: usize, vertex2: usize) -> usize
    {
        let new_edge = Edge {
            vertices: [vertex1, vertex2],
        };
        self.edges.push(new_edge);
        self.edges.len() - 1
    }

    pub fn triangle_from_vertices(&mut self, vertex1: usize, vertex2: usize, vertex3: usize)
    {
        let new_edges = [
            self.edge_from_vertices(vertex1, vertex2),
            self.edge_from_vertices(vertex2, vertex3),
            self.edge_from_vertices(vertex3, vertex1),
        ];
        self.triangles.push(Triangle {
            vertices: [vertex1, vertex2, vertex3],
            edges: new_edges,
        });
    }
}

#[derive(Debug)]
pub struct Vertex<T>
{
    position: std::vec::Vec<T>,
}
// TODO: perhaps make these 3 impls into a declarative macro, per Kevin's advice
impl<T> PartialEq for Vertex<T>
{
    fn eq(&self, other: &Self) -> bool
    {
        &self == &other
    }
}
impl<T> Eq for Vertex<T> {}

#[derive(Debug)]
pub struct Edge
{
    vertices: [usize; 2],
}
impl PartialEq for Edge
{
    fn eq(&self, other: &Self) -> bool
    {
        &self == &other
    }
}
impl Eq for Edge {}

#[derive(Debug)]
pub struct Triangle
{
    vertices: [usize; 3],
    edges: [usize; 3],
}
impl PartialEq for Triangle
{
    fn eq(&self, other: &Self) -> bool
    {
        &self == &other
    }
}
impl Eq for Triangle {}

#[derive(Debug)]
pub struct Tetrahedron
{
    vertices: [usize; 4],
}
impl PartialEq for Tetrahedron
{
    fn eq(&self, other: &Self) -> bool
    {
        &self == &other
    }
}
impl Eq for Tetrahedron {}

#[test]
fn check_join()
{
    let mut test_mesh = Mesh::new();

    let v1_id = test_mesh.add_vertex(Vertex::<f64> {
        position: std::vec![0., 1.],
    });
    let v2_id = test_mesh.add_vertex(Vertex::<f64> {
        position: std::vec![2., 3.],
    });

    test_mesh.edge_from_vertices(v1_id, v2_id);

    println!("Checking mesh state {:?}", test_mesh);
    //assert_eq!(test_edge1, test_edge2);
}
