use std::collections::HashMap;
use std::hash::Hash;
use std::fmt;
use std::ops::Index;
use std::panic::panic_any;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error, BufWriter};
use std::fmt::Debug;
use core::marker::Sized;
use std::collections::hash_map::Keys;
use std::convert::TryFrom;

pub type Weight = usize;
pub type Direction = usize;

// Косяк #1
// Была допущена ошибка и выяснилась она только в конце
// Я использовал в struct Node<T, U> в параметре map: Hashmap<T, ..> в качестве ключа
// обобщенное T и на тот момент я не знал что в языке rust нельзя использовать f32/f64 в качестве ключа в hashmap))))))
// Как-то так) Как пишут на форумах исправить это никак нельзя поэтому тут под замену половина кода.


/// Эта структура представляет вершину графа
/// map: принимает параметры T - ключ следующей вершины с которой есть ребро
///      HashMap<Direction, Weight> - контейнер описывающий связь
///      Direction - направление ориетированного ребра
///      Weight - вес ребра
///
/// key: ключ данной вершины
/// value: то что хранит вершины
// #[derive(Hash, Eq, PartialEq)]
pub struct Node<T, U> {
    map: HashMap<T, HashMap<Direction, Weight>>,
    key: T,
    value: U,
}

/// Функции для работы с Node<T, U>
impl<T, U> Node<T, U> {

    /// fn new_node - данная функция зодает новую вершину с заданными праметрами
    /// Принимаемые значения:
    ///     new_key - новый ключ вершины
    ///     new_value - новое значение вершины
    /// Возврашаемое значение это созданная структура
    pub fn new_node(new_key: T, new_value: U) -> Node<T, U> {
        Node {
            map: HashMap::new(),
            key: new_key,
            value: new_value,
        }
    }

    /// fn set_node - данная функция служит для создания вершины с ориентированными ребрами
    /// Принимаемые значения:
    ///     new_key - новый ключ вершины
    ///     new_value - новое значение вершины
    ///     new_map - контейнер с описанием для создания ориентированных ребер
    /// Возврашаемое значение это созданная структура
    pub fn set_node(new_key: T, new_value: U, new_map: HashMap<T, HashMap<Direction, Weight>>) -> Node<T, U> {
        Node {
            map: new_map,
            key: new_key,
            value: new_value,
        }
    }
}

/// Эта структура представляет граф
/// list: это вектор вершин графа
///     Node - структура описывающая вершину
///     T - обощенный тип представляюший ключ верщины
///     U - обобщенный тип представляющий хранимое значение вершины
pub struct Graph<T, U> {
    list: Vec<Node<T, U>>,
}


/// Функции для работы с Graph<T, U>
impl<T: Hash + Eq + PartialEq + PartialOrd + Copy + std::fmt::Display, U: Hash + Eq + PartialEq  + PartialOrd + Copy + std::fmt::Display> Graph<T, U> {
    /// fn new() - данная функция создает струтуру с пустыми значениями
    /// Возвращаемое значения струтура Graph<T, U>
    pub fn new() -> Self {
        Graph {
            list: Vec::new()
        }
    }

    /// fn create - данная функия создает граф с заданными значениями
    /// Принимаемые значения:
    ///     value - контейнер для создания новых вершин
    /// Возвращаемое значение струтура Graph<T, U>
    pub fn create_graph(value: HashMap<T, U>) -> Self {
        let mut new_graph: Graph<T, U> = Graph::new();
        new_graph.list.clear();

        for (key, val) in value {
            let new_node: Node<T, U> = Node::new_node(key, val);
            new_graph.list.push(new_node);
        }
        new_graph
    }

    /// fn add_node - данная функция добавляет вершину в граф создавая ее
    /// Принимаемые значения:
    ///     key - ключ добавляемое вершины
    ///     value - хранимое значения в верщине
    ///     &mut self - ссылка на структуру которая вызывает данную функцию
    ///
    /// Условия корректной работы:
    ///     ключ передаваемый в функцию должен быть уникальным, т.е. создаваемая
    /// веришна не может быть с одинаковыми ключами которые уже были созданны
    ///
    /// Возвращаемое значение тип bool, при корректной работе вернет true,
    /// при невыполнении условий вернет false
    pub fn add_node(&mut self, key: T, value: U) -> bool {
        if !self.find_node(key) {
            let new_node = Node::new_node(key, value);
            self.list.push(new_node);
            return true;
        }
        false
    }

    /// fn find_node - данная функция проверяет есть ли вершина с данным ключом в графе
    /// Принимаемые значения:
    ///     find_key - ключ который нужно найти
    /// Возвращаемое значение типа bool если ключ был найден вернется true
    /// если нет то false
    fn find_node(&self, find_key: T) -> bool {
        for node in self.list.iter() {
            if find_key == node.key { return true; }
        };
        false
    }

    /// fn get_index_node - данная функция находит вершину в списке
    /// Принимаемые значения:
    ///     find_key - ключ который нужно найти
    /// Возвращаемое значение типа usize, если ключ был найден вернется его номер в векторе
    /// если нет, то 0
    fn get_index_node(&self, find_key: T) -> usize {
        let mut counter = 0;
        for node in self.list.iter() {
            if find_key == node.key { return counter; }
            counter += 1;
        };
        0
    }

    /// fn get_weight_node - данная функция возвращает вес ребра
    /// Принимаемые значения:
    ///     begin - вершина с которой исходит ребро
    ///     end - вершина к которой направлено ребро
    /// Условия выполнения:
    ///     Вершины должны существовать.
    ///     Ребро должно существоать.
    /// Врзвращаемое значение:
    ///     вес ребра, при невыполнении условий 0
    fn get_weight_node(&self, begin: T, end: T) -> Weight {
        if self.check_rib(begin, end) {
            let index = self.get_index_node(begin);
            let weigth = self.list[index].map.get(&end).unwrap().values().last().unwrap();
            return *weigth;
        }
        0
    }

    /// fn get_direction_node - данная функция возвращает направление ребра
    /// Принимаемые значения:
    ///     begin - вершина с которой исходит ребро
    ///     end - вершина к которой направлено ребро
    /// Условия выполнения:
    ///     Вершины должны существовать.
    ///     Ребро должно существоать.
    /// Врзвращаемое значение:
    ///     направление ребра, при невыполнении условий 0
    fn get_direction_node(&self, begin: T, end: T) -> Direction {
        if self.check_rib(begin, end) {
            let index = self.get_index_node(begin);
            let direct = self.list[index].map.get(&end).unwrap().keys().last().unwrap();
            return *direct;
        }
        0
    }

    /// fn delete_node - данная функция удаляет вершину в графе
    /// Принимаемые значения:
    ///     key - ключ вершины
    /// Условия выполнения:
    ///     Вершина должна существовать.
    /// Возвращаемое значение типа bool
    ///     true - если вершина была удалена
    ///     false - условия не выполнены
    pub fn delete_node(&mut self, key: T) -> bool {
        if self.find_node(key) {
            let vertex = self.get_index_node(key);
            let keys = self.list[vertex].map.clone();
            for (i, j) in keys {
                self.delete_oriented_rib(key, i);
            }
            self.list.remove(vertex);
            return true;
        }
        false
    }

    /// fn delete_graph - удаляет все вершины в графе
    /// Возвращаемое значение типа bool
    ///     true - граф был очищен
    ///     false - неудалось очистить граф
    pub fn delete_graph(&mut self) -> bool {
        self.list.clear();
        self.list.is_empty()
    }

    /// fn delete_oriented_rib - удаляет ориентированное ребро
    /// Принимаемые значения:
    ///     begin - вершина с которой исходит ребро
    ///     end - вершина к которой направлено ребро
    /// Условия выполнения:
    ///     Вершины должны существовать.
    ///     Ребро должно существоать.
    /// Возвращаемое значение типа bool:
    ///     true - ребро было удалено
    ///     false - условия небыли выполнены
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

    /// fn print_vertex - функция выводит на экран все вершины графа
    /// Условия выполнения:
    ///     Граф должен существовать.
    /// Возвращаемое значение отсутствует.
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
    /// fn print_vertex_direction - данная функция выводит на экран ребра вершины и все ребра
    /// Принимаемые значения:
    ///     key - ключ вершины
    /// Условия выполнения:
    ///     Вершина должна существовать.
    /// Возвращаемое значение отсутствует.
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

    /// fn print_vertex_direction - данная функция выводит на экран
    ///     Ключ вершины.
    ///     Ориентированные ребра для каждой вершины.
    ///     Вес каждого ребра.
    ///     Направления ребер.
    /// Условия выполнения:
    ///     Хоть одна вершина должна существовать.
    /// Возвращаемое значение отсутствует.
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

    /// fn set_rib_weight - данная функция устанавливает новый вес ребра.
    /// Принимаемые значения:
    ///     begin - вершина с которой исходит ребро
    ///     end - вершина к которой направлено ребро
    ///     new_weight - новый вес ребра
    /// Условия выполнения:
    ///     Вершины должны существовать.
    ///     Ребро должно существоать.
    /// Возвращаемое значение типа bool:
    ///     true - функция успешно выполнена
    ///     false - условия небыли выполнены
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

    /// fn set_rib_weight - данная функция устанавливает новое направление ребра.
    /// Принимаемые значения:
    ///     begin - вершина с которой исходит ребро
    ///     end - вершина к которой направлено ребро
    ///     new_direction - новое направление ребра
    /// Условия выполнения:
    ///     Вершины должны существовать.
    ///     Ребро должно существоать.
    /// Возвращаемое значение типа bool:
    ///     true - функция успешно выполнена
    ///     false - условия небыли выполнены
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

    /// fn check_rib - данная функция проверят существует ли ребро
    /// Принимаемые значения:
    ///     begin - вершина с которой исходит ребро
    ///     end - вершина к которой направлено ребро
    /// Условия выполнения:
    ///     Вершины должны существовать.
    /// Возвращаемое значение типа bool:
    ///     true - ребро существует
    ///     false - условия не соблодаются
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

    /// fn add_oriented_rib - данная функция создает ориентированное ребро
    /// Принимаемые значения:
    ///     begin - вершина с которой исходит ребро
    ///     end - вершина к которой направлено ребро
    ///     new_weight - вес будующего ребра
    /// Условия выполнения:
    ///     Вершины должны существовать.
    ///     Ребра с заданными значениями не должно быть.
    ///     Направление ребра выствлается в соответсвии с значениями begin -> end
    /// Возвращемое значение типа bool:
    ///     true - ребро было добавлено
    ///     false - условия не были выполнены
    pub fn add_oriented_rib(&mut self, begin: T, end: T, new_weight: Weight) -> bool {
        if self.check_rib(begin, end) {
            return false;
        } else if self.find_node(begin) && self.find_node(end) {
            let vertex_one = self.get_index_node(begin);
            let vertex_two = self.get_index_node(end);
            let vertex_direct: Direction = 1;
            let mut new_map: HashMap<Direction, Weight> = HashMap::new();
            new_map.insert(vertex_direct, new_weight);

            &self.list[vertex_one].map.insert(end, new_map);

            let vertex_direct: Direction = 0;
            let mut new_map: HashMap<Direction, Weight> = HashMap::new();
            new_map.insert(vertex_direct, new_weight);

            &self.list[vertex_two].map.insert(begin, new_map);
            return true;
        }
        false
    }

    /// fn width_graph_traversal - данная функция обходит граф в ширину
    /// Принимаемые значения:
    ///     begin - вершина с которой исходит ребро
    ///     end - вершина к которой направлено ребро
    /// Условия выполнения:
    ///     Вершины должны существовать.
    /// Возвращаемое значение типо bool:
    ///     true - путь до заданной вершины был найден
    ///     false - условия небыли соблюдены или пути не существет
    /// Описание:
    ///     Данная функция обходит граф соблюдая направления ребер.
    ///     Если у вершины не существует исходяших ребер, то функция возвращается на предыдущее ребро
    pub fn width_graph_traversal(&self, begin: T, end: T) -> bool { // тут нужна доработка
        if self.find_node(begin) && self.find_node(end) {
            let mut list: HashMap<T, bool> = HashMap::new(); // наш список с пройденными вершинами
            let mut path = Vec::new(); // сюда записывается пройденный путь (чтоб вернуться назад)

            let mut jump_flag:bool = false;

            let index = self.get_index_node(begin);
            let mut current = &self.list[index]; // стартовый элемент
            list.insert(current.key, true); // добавляет в очередь пройденных

            while list.len() != self.list.len() {
                jump_flag = false;
                for next in self.list.iter() { // проверяем все вершины в нашем векторе
                    if list.get(&next.key) != Some(&true)  // если его нет в списке
                        && current.map.get(&next.key) != None // если он сосед нынешней вершины
                        && self.get_direction_node(current.key, next.key) != 0 // если направление ребра ведет от текущей к сл а не наоборото
                    {
                        list.insert(next.key, true);  // добавляем в список что вершина была пройдена
                        path.push(current); // добавляем вершину в путь
                        current = next; // теперь мы двигаемся с этой вершины
                        if next.key == end { // если заданный ключ и ключ вершины совпали то выход
                            return true;
                        }
                        jump_flag = true;
                        break;
                    }
                }
                // если прыжка не было, значит потомком нет и нужно вернуться на прошлого потомка
                if (jump_flag == true) {
                    continue
                }
                if (path.len() == 0) {
                    return false
                }
                current = path.pop().unwrap();
            }
        }
        false
    }

    /// fn write_to_file - данная функция записывает содержимое графа в файл в формате Trivial Graph Form
    /// Принимаемы значения:
    ///     path - путь/имя создаваемого файла
    /// Возвращаемое значение:
    ///     Ok() - функция успешно отработала
    ///     Error() - ошибка с описанием
    pub fn write_to_file(&self, path: &str) -> Result<(), Error> {
        let mut output = File::create(path)?;

        for node in self.list.iter() {
            write!(output, "#\n");
            write!(output, "Graph: {}\nValue: {}\n", node.key, node.value).expect("Unable to write to file (Graph, value)!\n");
            for rib in node.map.iter() {
                write!(output, "Map: {}\n", rib.0).expect("Unable to write to file (Map)!\n");
                for (key, value) in rib.1 {
                    write!(output, "Direction: {}\nWeight:{}\n", key, value).expect("Unable to write to file (Key, value)!\n");
                    ;
                }
            }
        }
        write!(output, "End.");
        Ok(())
    }

    /// fn read_from_file - данная функция читает содержимое файла и записывает в список вершин
    /// Принимаемы значения:
    ///     path - путь/имя создаваемого файла
    /// Возвращаемое значение:
    ///     Ok() - функция успешно отработала
    ///     Error() - ошибка с описанием
    pub fn read_from_file(&mut self, path: &str) -> Result<(), Error> {
        let input = File::open(path).expect("Unable to open file!");
        let mut buffer = BufReader::new(input);

        // let mut new_node;
        // let mut key: T;
        // let mut value: U;

        // for line in buffer.lines() {
            // let unwarp_line = line.as_ref().unwrap().split(' ').collect::<Vec<&str>>();
            //     if unwarp_line[0] == "Graph:" {
            //         key = unwarp_line[1];
            //         break;
            //     } else if unwarp_line[0] == "Value:" {
            //         value = unwarp_line[1] as U;
            //         new_node = Node::new_node(key, value);
            //         self.list.push(new_node);
            //         break;
            //     } else if unwarp_line[0] == "Map:" {
            //         break;
            //     } else if unwarp_line[0] == "Direction:"{
            //         break;
            //     } else if unwarp_line[0] =="Weight:" {
            //         break;
            //     }
            // }
        // }
        Ok(())
    }
}
