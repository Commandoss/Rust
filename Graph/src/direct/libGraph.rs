use std::collections::HashMap;
use std::hash::Hash;
// use std::ptr::swap_nonoverlapping_one;


// pub type vertex = usize; // ключ вершины
pub type Weight = usize;
// вес вершины / растояние до сл
// pub type key_next = usize; // ключ следующей вершины

pub type Direction = usize; // направление вершины к вершине


// Структура определения вершины
pub struct Node<T, U> {
    map: HashMap<T, HashMap<Direction, Weight>>,
    key: T,
    value: U,
}

// создание структуры
impl<T, U> Node<T, U> {
    pub fn new_node(new_key: T, new_value: U) -> Node<T, U> {
        Node {
            map: HashMap::new(),
            key: new_key,
            value: new_value,
        }
    }

    pub fn set_node(new_key: T, new_value: U, new_map: HashMap<T, HashMap<Direction, Weight>>) -> Node<T, U> {
        Node {
            map: new_map,
            key: new_key,
            value: new_value,
        }
    }
}

pub struct Graph<T, U> {
    list: Vec<Node<T, U>>,
}

impl<T: Hash + Eq + PartialOrd + Copy + std::fmt::Display, U: std::fmt::Display + Copy + PartialOrd> Graph<T, U> {
    // создание пустого графа
    pub fn new() -> Self {
        Graph {
            list: Vec::new()
        }
    }

    // создание нескольких вершин графа
    pub fn create_graph(value: HashMap<T, U>) -> Self {
        let mut new_graph: Graph<T, U> = Graph::new();
        new_graph.list.clear();

        for (key, val) in value {
            let new_node: Node<T, U> = Node::new_node(key, val);
            new_graph.list.push(new_node);
        }
        new_graph
    }

    // добавление вершины
    pub fn add_node(&mut self, key: T, value: U) -> bool {
        if !self.find_node(key) {
            let new_node = Node::new_node(key, value);
            self.list.push(new_node);
            return true;
        }
        false
    }

    // проверяет есть ли такая вершина
    fn find_node(&self, find_key: T) -> bool {
        for node in self.list.iter() {
            if find_key == node.key { return true; }
        };
        false
    }

    // возвращает номер элемента в векторе graph.list
    fn get_index_node(&self, find_key: T) -> usize {
        let mut counter = 0;
        for node in self.list.iter() {
            if find_key == node.key { return counter }
            counter += 1;
        };
        0
    }

    // удаляет вершину графа
    pub fn delete_node(&mut self, key: T) -> bool {
        if self.find_node(key) {
            let a = self.get_index_node(key);
            self.list.remove(a);
            return true
        }
        false
    }

    // удаляет все врешины графа
    pub fn delete_graph(&mut self) {
        self.list.clear();
    }

    // выводит на экран все вершины (ключ, содержимое)
    pub fn print_vertex(&self) {
        if self.list.len() != 0 {
            println!("Graph vertices:");
            for node in self.list.iter() {
                println!("Key vertex: {}. value: {}", node.key, node.value);
            }
        } else {
            println!("The graph contains no vertices.");
        }
    }

    // так же нужно проверять есть ли такое ребро
    // указание веса для ребра
    pub fn set_rib_weight(&self, new_weigth: Weight) {

    }

    // изменяет направление ребра
    pub fn change_direcrion_rib(&self, begin: T, end: T) {
        if self.check_rib(begin, end) {

        }
    }

    // проверяет существует ли такое ребро
    pub fn check_rib(&self, begin: T, end: T) -> bool {
        if self.find_node(begin) && self.find_node(end) {
            let vertex_one = self.get_index_node(begin);
            let vertex_two = self.get_index_node(end);

            let a = &self.list[vertex_one].map.get(&begin);


            true
        }
        false
    }

    // создание оринтированного ребра
    pub fn add_oriented_rib(&mut self, begin: T, end: T, new_weigth: Weight) -> bool {
        if self.check_rib(begin, end) {
            return false
        }
        else if self.find_node(begin) && self.find_node(end) {
            let vertex_one = self.get_index_node(begin);
            let vertex_two = self.get_index_node(end);

            let vertex_direct: Direction = 1;
            let mut new_map: HashMap<Direction, Weight> = HashMap::new();
            new_map.insert(vertex_direct, new_weigth);

            &self.list[vertex_one].map.insert(end, new_map);

            let vertex_direct: Direction = 0;
            let mut new_map: HashMap<Direction, Weight> = HashMap::new();
            new_map.insert(vertex_direct, new_weigth);

            &self.list[vertex_one].map.insert(begin, new_map);
            return true
        }
        false
    }
}
