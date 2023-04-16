# picture

## The storage structure of the graph

In addition to storing the information of each vertex in the graph, the storage structure of the graph also needs to store all the relationships between vertices (edge information). Therefore, the structure of the graph is relatively complicated, and it is difficult to store data elements in the storage area The physical position of the element can be used to represent the relationship between elements, but it is precisely because of its arbitrary characteristics that there are many physical representation methods. Common graph storage structures include adjacency matrix, adjacency list, etc.

## Adjacency Matrix Notation

For a graph with n nodes, an n*n matrix (two-dimensional array) can be used to represent the adjacency relationship between them. The matrix A(i,j) = 1 indicates that there is an edge (Vi,Vj) in the graph, and A(i,j)=0 indicates that there is no edge (Vi,Vj) in the graph.
In actual programming, when the graph is an unweighted graph, bool values can be stored in a two-dimensional array.

* A(i,j) = true means there is an edge (Vi,Vj),
* A(i,j) = false means there is no edge (Vi,Vj);


When the graph has weights, the weights can be directly stored in two-dimensional values, and A(i,j) = null means that there is no edge (Vi,Vj).

Let's take a look at the graph structure we implemented using the adjacency matrix:

```rust
#[derive(Debug)]
struct Node {
    nodeid: usize,
    nodename: String,
}

#[derive(Debug,Clone)]
struct Edge {
    edge: bool,
}

#[derive(Debug)]
struct Graphadj {
    nodenums: usize,
    graphadj: Vec<Vec<Edge>>,
}

impl Node {
    fn new(nodeid: usize, nodename: String) -> Node {
        Node{
            nodeid: nodeid,
            nodename: nodename,
        }
    }
}
impl Edge {
    fn new() -> Edge {
        Edge{
            edge: false,
        }
    }
    fn have_edge() -> Edge {
        Edge{
            edge: true,
        }
    }
}

impl Graphadj {
    fn new(nums:usize) -> Graphadj {
        Graphadj {
            nodenums: nums,
            graphadj: vec![vec![Edge::new();nums]; nums],
        }
    }

    fn insert_edge(&mut self, v1: Node, v2:Node) {
        match v1.nodeid < self.nodenums && v2.nodeid<self.nodenums {
            true => {
                self.graphadj[v1.nodeid][v2.nodeid] = Edge::have_edge();
                //Removing the following comment is equivalent to treating the graph as an undirected graph
                //self.graphadj[v2.nodeid][v1.nodeid] = Edge::have_edge();
            }
            false => {
                panic!("your nodeid is bigger than nodenums!");
            }
        }
    }
}

fn main() {
    let mut g = Graphadj::new(2);
    let v1 = Node::new(0, "v1".to_string());
    let v2 = Node::new(1, "v2".to_string());
    g.insert_edge(v1,v2);
    println!("{:?}", g);
}
```

## Adjacency list notation

The adjacency list is the most important storage structure of the graph, which is used to describe each point on the graph.

>**Implementation method:** Create a container for each vertex of the graph (n vertices create n containers), and the nodes in the i-th container include all adjacent vertices of the vertex Vi. In fact, the adjacency matrix we commonly use is an adjacency list that does not discretize the edge set of each point.

* In a directed graph, describe the edges that each point connects to other nodes (point a-> point b in this case).
* In an undirected graph, describe all the edges of each point (point a-> point b in this case)

The storage method corresponding to the adjacency list is called an edge set table. This method uses a container to store all the edges.

## **practise:**
A graph structure that implements linked-list notation.
