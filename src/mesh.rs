// Add ability for vertex to have different properties like color, position, velocity, acceleration, etc
// TODO: What order is our convention? struct, trait impls, other impls?
// How to make sure that Face and Polyhedron are valid

#[derive(Debug)]
pub struct Mesh<T>
{
    vertices: bimap::BiHashMap<uuid::Uuid, Vertex<T>>,
    edges: bimap::BiHashMap<uuid::Uuid, Edge>,
    triangles: bimap::BiHashMap<uuid::Uuid, Triangle>,
    tetrahedra: bimap::BiHashMap<uuid::Uuid, Tetrahedron>,
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

    pub fn add_vertex(&mut self, vertex: Vertex<T>) -> uuid::Uuid
    {
        let id = uuid::Uuid::new_v4();
        self.vertices.insert(id, vertex);
        return id;
    }

    pub fn join_vertices(&mut self, vertex1: uuid::Uuid, vertex2: uuid::Uuid)
    {
        let new_edge = Edge {
            vertex_ids: [vertex1, vertex2],
        };
        self.edges.insert(uuid::Uuid::new_v4(), new_edge);
    }

    pub fn triangle_from_vertices(
        &mut self,
        vertex1: &Vertex<T>,
        vertex2: &Vertex<T>,
        vertex3: &Vertex<T>,
    ) -> Triangle
    {
        let edge_ids = [
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
        ];

        self.edges.insert(
            edge_ids[0],
            Edge {
                vertex_ids: [
                    *self.vertices.get_by_right(vertex1).unwrap(),
                    *self.vertices.get_by_right(vertex2).unwrap(),
                ],
            },
        );

        self.edges.insert(
            edge_ids[1],
            Edge {
                vertex_ids: [
                    *self.vertices.get_by_right(vertex2).unwrap(),
                    *self.vertices.get_by_right(vertex3).unwrap(),
                ],
            },
        );

        self.edges.insert(
            edge_ids[2],
            Edge {
                vertex_ids: [
                    *self.vertices.get_by_right(vertex3).unwrap(),
                    *self.vertices.get_by_right(vertex1).unwrap(),
                ],
            },
        );

        self.triangle_from_edges(
            self.edges.get_by_left(&edge_ids[0]).unwrap(),
            self.edges.get_by_left(&edge_ids[0]).unwrap(),
            self.edges.get_by_left(&edge_ids[0]).unwrap(),
        )
    }

    pub fn triangle_from_edges(&self, edge1: &Edge, edge2: &Edge, edge3: &Edge) -> Triangle
    {
        Triangle {
            edges: [
                *self.edges.get_by_right(edge1).unwrap(),
                *self.edges.get_by_right(edge2).unwrap(),
                *self.edges.get_by_right(edge3).unwrap(),
            ],
        }
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

impl<T> std::hash::Hash for Vertex<T>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H)
    {
        let address = format!("{:p}", &self);
        address.hash(state); // Hash the address instead of the contents
    }
}

#[derive(Debug)]
pub struct Edge
{
    vertex_ids: [uuid::Uuid; 2],
}
impl PartialEq for Edge
{
    fn eq(&self, other: &Self) -> bool
    {
        &self == &other
    }
}
impl Eq for Edge {}

impl std::hash::Hash for Edge
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H)
    {
        let address = format!("{:p}", &self);
        address.hash(state); // Hash the address instead of the contents
    }
}

#[derive(Debug)]
pub struct Triangle
{
    edges: [uuid::Uuid; 3],
}
impl PartialEq for Triangle
{
    fn eq(&self, other: &Self) -> bool
    {
        &self == &other
    }
}
impl Eq for Triangle {}

impl std::hash::Hash for Triangle
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H)
    {
        let address = format!("{:p}", &self);
        address.hash(state); // Hash the address instead of the contents
    }
}

#[derive(Debug)]
pub struct Tetrahedron
{
    faces: [uuid::Uuid; 4],
}
impl PartialEq for Tetrahedron
{
    fn eq(&self, other: &Self) -> bool
    {
        &self == &other
    }
}
impl Eq for Tetrahedron {}

impl std::hash::Hash for Tetrahedron
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H)
    {
        let address = format!("{:p}", &self);
        address.hash(state); // Hash the address instead of the contents
    }
}

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

    test_mesh.join_vertices(v1_id, v2_id);

    println!("Checking mesh state {:?}", test_mesh);
    //assert_eq!(test_edge1, test_edge2);
}
