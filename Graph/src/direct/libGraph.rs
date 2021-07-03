use std::collections::HashMap;
use std::hash::Hash;
use std::fmt;
use std::ops::Index;
use std::panic::panic_any;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error, BufWriter};

pub type Weight = usize;
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
            if find_key == node.key { return counter; }
            counter += 1;
        };
        0
    }

    // возвращает вес ребра
    fn get_weight_node(&self, begin: T, end: T) -> Weight {
        if self.check_rib(begin, end) {
            let index = self.get_index_node(begin);
            let weigth = self.list[index].map.get(&end).unwrap().values().last().unwrap();
            return *weigth;
        }
        0
    }

    // возвращает направление ребра
    fn get_direction_node(&self, begin: T, end: T) -> Direction {
        if self.check_rib(begin, end) {
            let index = self.get_index_node(begin);
            let direct = self.list[index].map.get(&end).unwrap().keys().last().unwrap();
            return *direct;
        }
        0
    }

    // удаляет вершину графа // нужно доработать связи с другими вершинами !!!!///////////////////////////////!!!!!!!!
    pub fn delete_node(&mut self, key: T) -> bool {
        if self.find_node(key) {
            let a = self.get_index_node(key);
            self.list.remove(a);
            return true;
        }
        false
    }

    // удаляет все врешины графа
    pub fn delete_graph(&mut self) -> bool {
        self.list.clear();
        self.list.is_empty()
    }

    // удаляет ориентированное ребро
    pub fn delete_oriented_rib(&mut self, begin: T, end: T) -> bool {
        if self.check_rib(begin, end) {
            let vertex_one = self.get_index_node(begin);
            let vertex_two = self.get_index_node(end);

            self.list[vertex_one].map.remove(&end);
            self.list[vertex_two].map.remove(&begin);

            if self.list[vertex_one].map.get(&end) == None &&
                self.list[vertex_two].map.get(&begin) == None {
                return true;
            }
        }
        false
    }

    // выводит на экран все вершины (ключ, содержимое)
    pub fn print_vertex(&self) {
        if self.list.len() != 0 {
            println!("Graph vertices:");
            for node in self.list.iter() {
                println!("Key vertex: {}, value: {}", node.key, node.value);
            }
        } else {
            println!("The graph contains no vertices.");
        }
        println!();
    }

    // выводит на экран ребра вершины и ее значения
    pub fn print_vertex_direction(&self, key: T) {
        if self.find_node(key) {
            let direct: Direction = 1;
            let node = &self.list[self.get_index_node(key)];
            println!("Key vertex: {}, value: {}", node.key, node.value);
            for rib in node.map.iter() {
                for g in rib.1 {
                    if (g.0 == &direct) {
                        println!("Key: {} -> key_next: {}, weigth: {}", node.key, rib.0, g.1);
                    } else {
                        println!("Key: {} <- key_next: {}, weigth: {}", node.key, rib.0, g.1);
                    }
                }
            }
        } else {
            println!("Such a peak does not exist.");
        }
        println!();
    }

    // выводит на экран все
    pub fn print_vertexs_direction(&self) {
        if self.list.len() != 0 {
            let direct: Direction = 1;
            for node in self.list.iter() {
                println!("\nKey vertex: {}, value: {}", node.key, node.value);
                for rib in node.map.iter() {
                    for g in rib.1 {
                        if g.0 == &direct {
                            println!("Key: {} -> key_next: {}, weigth: {}", node.key, rib.0, g.1);
                        } else {
                            println!("Key: {} <- key_next: {}, weigth: {}", node.key, rib.0, g.1);
                        }
                    }
                }
            }
        } else {
            println!("Such a peak does not exist.");
        }
        println!();
    }

    // указание нового веса для ребра
    pub fn set_rib_weight(&mut self, begin: T, end: T, new_weight: Weight) -> bool {
        if self.check_rib(begin, end) {
            let vertex_one = self.get_index_node(begin);
            let vertex_two = self.get_index_node(end);

            let old_direction = self.get_direction_node(begin, end);

            let mut new_map: HashMap<Direction, Weight> = HashMap::new();
            new_map.insert(old_direction, new_weight);
            &self.list[vertex_one].map.insert(end, new_map);

            let mut new_map: HashMap<Direction, Weight> = HashMap::new();
            new_map.insert(old_direction, new_weight);

            &self.list[vertex_two].map.insert(begin, new_map);

            return true;
        }
        false
    }

    // изменяет направление ребра
    pub fn change_direcrion_rib(&mut self, begin: T, end: T, new_direction: Direction) -> bool {
        if self.check_rib(begin, end) {
            let vertex_one = self.get_index_node(begin);
            let vertex_two = self.get_index_node(end);

            let old_weight = self.get_weight_node(begin, end);
            let mut new_map: HashMap<Direction, Weight> = HashMap::new();
            new_map.insert(new_direction, old_weight);

            &self.list[vertex_one].map.insert(end, new_map);

            let mut change_direction: Direction = new_direction;
            if change_direction == 0 {
                change_direction = 1;
            } else {
                change_direction = 0;
            }

            let mut new_map: HashMap<Direction, Weight> = HashMap::new();
            new_map.insert(change_direction, old_weight);

            &self.list[vertex_two].map.insert(begin, new_map);

            return true;
        }
        false
    }

    // проверяет существует ли такое ребро
    pub fn check_rib(&self, begin: T, end: T) -> bool {
        if self.find_node(begin) && self.find_node(end) {
            let vertex_one = self.get_index_node(begin);
            let vertex_two = self.get_index_node(end);

            if self.list[vertex_one].map.contains_key(&end) == true && self.list[vertex_two].map.contains_key(&begin) == true {
                return true;
            }
        }
        false
    }

    // создание оринтированного ребра
    pub fn add_oriented_rib(&mut self, begin: T, end: T, new_weigth: Weight) -> bool {
        if self.check_rib(begin, end) {
            return false;
        } else if self.find_node(begin) && self.find_node(end) {
            let vertex_one = self.get_index_node(begin);
            let vertex_two = self.get_index_node(end);
            let vertex_direct: Direction = 1;
            let mut new_map: HashMap<Direction, Weight> = HashMap::new();
            new_map.insert(vertex_direct, new_weigth);

            &self.list[vertex_one].map.insert(end, new_map);

            let vertex_direct: Direction = 0;
            let mut new_map: HashMap<Direction, Weight> = HashMap::new();
            new_map.insert(vertex_direct, new_weigth);

            &self.list[vertex_two].map.insert(begin, new_map);
            return true;
        }
        false
    }

    // обход графа в ширину
    pub fn width_graph_traversal(&self, begin: T, end: T) -> bool {
        if self.find_node(begin) && self.find_node(end) {
            let mut queue: HashMap<T, bool> = HashMap::new(); // наш список с пройденными вершинами

            let mut jump_flag: bool = false; // флаг прыжка, нужен для проверки если ли у вершины сл потомки
            let mut path = Vec::new(); // созда записывается пройденный путь (чтоб вернуться назад)

            let index = self.get_index_node(begin);
            let mut previous = &self.list[index]; // стартовый элемент
            queue.insert(previous.key, true); // добавляет в очередь пройденных
            path.push(previous);

            while queue.len() != self.list.len() {
                for neighbor in self.list.iter() { // проверяем все вершины в нашем векторе
                    if queue.get(&neighbor.key) != Some(&true)  // если его нет в списке
                        && previous.map.get(&neighbor.key) != None // если он сосед нынешней вершины
                        && self.get_direction_node(previous.key, neighbor.key) != 0 // если направление ребра ведет от текущей к сл а не наоборото
                    {
                        queue.insert(neighbor.key, true);  // добавляем в список что вершина была пройдена
                        path.push(previous); // добавляем вершину в путь
                        previous = neighbor; // теперь мы двигаемся с этой вершины
                        if neighbor.key == end { // если заданный ключ и ключ вершины совпали то выход
                            return true;
                        }
                        jump_flag = true;
                        break;
                    }
                }
                if jump_flag == false && path.len() != 0 { // если прыжка не было, значит потомком нет и нужно вернуться на прошлого потомка
                    if path.len() == 1 {
                        previous = path[0];
                    } else {
                        previous = path.pop().unwrap();
                    }
                } else {
                    jump_flag = false;
                }
            }
        }
        false
    }

    pub fn write_to_file(&self, path: &str) -> Result<(), Error> {
        let mut output = File::create(path)?;

        for node in self.list.iter() {
            write!(output, "Graph: {}\nValue: {}\n", node.key, node.value).expect("Unable to write to file (Graph, value)!\n");
            for rib in node.map.iter() {
                write!(output, "map:\n{}\n", rib.0).expect("Unable to write to file (Map)!\n");
                for (key, value) in rib.1 {
                    write!(output, "Key: {}\nValue:{}\n", key, value).expect("Unable to write to file (Key, value)!\n");;
                }
            }
            write!(output, "#\n");
        }
        Ok(())
    }

    pub fn read_from_file(&self, path: &str) -> Result<(), Error> {
        let input = File::open(path).expect("Unable to open file!");
        let buffer = BufReader::new(input);

        for line in buffer.lines() {
            if line.unwrap().find("Graph") {
                println!("Нашел!");
            } else {
                Err(())
            }
        }
        Ok(())
    }
}
