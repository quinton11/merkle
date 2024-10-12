use std::collections::HashMap;

use bevy::{asset::Handle, color::Color, math::{Quat, Vec3}, prelude::{Commands, Component, Transform}, sprite::{Sprite, SpriteBundle}, text::{Font, Text, Text2dBundle, TextStyle}};

use crate::hasher;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Component)]
pub struct Node {
    pub hash: String,
    pub level: u32,
    pub parent_index: (u32, u32),
    pub index: usize,
    pub position: (f32, f32),
    pub start_position: (f32, f32),    // Start point of the node (for the line to the parent)
    pub end_position: (f32, f32),  
}

#[derive(Component)]
pub struct BranchMarker;

#[derive(Component)]
pub struct NodeTextMarker;

#[derive(Debug)]
pub struct MerkleTree{
    pub root: Option<Node>,
    pub nodes: HashMap<u32,Vec<Node>>,
    pub words: Vec<String>,
    pub levels: u32,
    pub size: (f32, f32)

}

impl MerkleTree {
    fn new(hashes: &Vec<String>, words: Vec<String>) -> MerkleTree {
        let mut nodes = Vec::new();
        let mut nodes_map = HashMap::new();
        for (i, hash) in hashes.iter().enumerate() {
            let node = Node{
                hash: hash.to_string(),
                level: 1,
                parent_index: (0, 0),
                index: i,
                ..Default::default()
            };
            nodes.push(node);
        }
        // Per my implementation, minimum words is 4 and max is 16, hence the levels will be 4 and 5 respectively
        // So we can calculate the levels by taking the log base 2 of the number of words
        println!("Words and Nodes");
        println!("{:?}", words);
        println!("{:?}", nodes);
        let level = ((words.len() as f64).log2()) as u32 +1;
        nodes_map.insert(1, nodes.clone());
        return MerkleTree{
            root: None,
            nodes: nodes_map,
            words: words.clone(),
            levels: level,
            size: (150.0, 50.0)
        };
    }

    
    /// Build the Merkle Tree
    /// combinations for each sub level
    fn build(&mut self) {


        for current_level in 0..self.levels {
            if let Some(nodes) = self.nodes.get(&current_level).cloned() {
                let mut new_nodes = Vec::new();
        
                for index in (0..nodes.len()).step_by(2) {
                    
                    if index == nodes.len() - 1 {
                        continue;
                    }
        
                    let left = &nodes[index];
                    let right = &nodes[index + 1];
        
                    let hash = hasher::hash_combination(&left.hash, &right.hash);
        
                    let new_node = Node {
                        hash,
                        level: current_level + 1,
                        parent_index: (left.index as u32, right.index as u32),
                        index: new_nodes.len(), // Index in the new level
                        ..Default::default()
                    };
        
                    new_nodes.push(new_node.clone());
        
                    if current_level + 1 == self.levels {
                        self.root = Some(new_node);
                    }
                }
        
                self.nodes.insert(current_level + 1, new_nodes.clone());
            }
        }
    }

    fn get_length_and_degree(current_depth: u32, max_depth: u32) -> (f32, f32) {
        // Handle up to max 5 levels
        match (max_depth, current_depth) {
            (3, 4) => (620.0, 85.0),
            (3, 3) => (300.0, 75.0),
            (3, 2) => (150.0, 60.0),
            (3, 1) => (60.0, 35.0),
            (3, 0) => (80.0, 30.0),
            (4, 5) => (450.0, 80.0),
            (4, 4) => (400.0, 85.0),
            (4, 3) => (180.0, 85.0),
            (4, 2) => (95.0, 75.0),
            (4, 1) => (40.0, 40.0),
            (4, 0) => (20.0, 10.0),
            _ => (0.0, 0.0),
        }
    }

    fn calculate_endpoints_tuple(start_point: &(f32, f32), length: f32, angle: f32) -> ((f32, f32), (f32, f32)) {
        let angle_radians = angle.to_radians();
        let symangle_radians = (360.0 - angle).to_radians();
    
        let end_point_zero = (
            start_point.0 - length * angle_radians.sin(),
            start_point.1 + length * angle_radians.cos(),
        );
    
        let end_point_one = (
            start_point.0 - length * symangle_radians.sin(),
            start_point.1 + length * symangle_radians.cos(),
        );
    
        (end_point_zero, end_point_one)
    }
    

    pub fn graph(&mut self, screen_height: f32, mut commands: Commands,handle: &Handle<Font>) {
        let root_point = (0.0, -(screen_height / 2.0) + 50.0); // Origin is at the center

        println!("Root Point: {:?}", root_point);
    
        for current_level in (0..self.levels + 1).rev() {
            if current_level == 0 {
                break;
            }
            println!("Current Level: {:?}", current_level);
    
            let (length, angle) = MerkleTree::get_length_and_degree(current_level, self.levels);
    

            let mut calculated_start_positions: HashMap<u32,(f32,f32)> = HashMap::new();
    
            if let Some(nodes) = self.nodes.get_mut(&current_level) {
                for (i, node) in nodes.iter_mut().enumerate() {
                    let mut parent_position = if current_level == self.levels {
                        root_point 
                    }  else {
                        node.start_position
                    };

                    parent_position.1 = parent_position.1 + self.size.1 / 2.0;

                    println!("Node: {:?}", node);
                    println!("Length: {:?}", length);
                    println!("Angle: {:?}", angle);
    
                    // Store the parent node position
                    node.position = parent_position;
    
                    // Calculate the end points for the current node's children
                    let (end_point_zero, end_point_one) = MerkleTree::calculate_endpoints_tuple(&parent_position, length, angle);
    
                    calculated_start_positions.insert(node.parent_index.0, (end_point_zero.0, end_point_zero.1 + self.size.1/2.0));
                    calculated_start_positions.insert(node.parent_index.1, (end_point_one.0, end_point_one.1 + self.size.1/2.0));

                    // Store these end points to be used in the next iteration
                    node.start_position = parent_position;
                    node.position = (parent_position.0, parent_position.1 - self.size.1 / 2.0);
    
                    // Draw the lines to the child nodes
                    // draw node, then draw lines, lines starting point should be the mid point of the node
                    Self::draw_node(&mut commands, node,self.size, handle,true);

                    if current_level ==1 {
                        continue;
                    }

                    Self::draw_line(&mut commands, parent_position, end_point_zero, 1.0);
                    Self::draw_line(&mut commands, parent_position, end_point_one, 1.0);
                }

            }

            if current_level == 1 {
                break;
            }
                // update the start positions of the nodes in the next level with the end positions of the current level
                if let Some(parent_nodes) = self.nodes.get_mut(&(current_level - 1)) {
                    for node in parent_nodes.iter_mut() {
                        node.start_position = calculated_start_positions.get(&(node.index as u32)).unwrap().clone();
                    }
                }

        }

        // draw words
        if let Some(prime_nodes) = self.nodes.get_mut(&1) {
            for (i, node) in prime_nodes.iter_mut().enumerate() {

                // Draw a straight short line from node to + 30 on y axis
                let start_point = (node.position.0, node.position.1 + self.size.1 / 2.0);
                let size_y = self.size.1;
                let end_point = (start_point.0, start_point.1 + 30.0 + size_y /2.0);
                Self::draw_line(&mut commands, start_point, end_point, 1.0);
                let word_node = Node{
                    hash: self.words[i].clone(),
                    level: 0,
                    parent_index: (0, 0),
                    index: i,
                    position: (start_point.0, start_point.1 + size_y + 30.0),
                    start_position: (start_point.0, start_point.1 + size_y + 30.0),
                    end_position: (start_point.0, start_point.1 + size_y + 30.0)
                };
                // Then draw a node with the word in it
                Self::draw_node(&mut commands, &word_node, self.size, handle,false);
            }
        }

        //println!("Nodes: {:?}", self.nodes);
    }

    fn format_hash(hash: &str, first_len: usize, last_len: usize, dots: &str) -> String {
        if hash.len() <= first_len + last_len {
            return hash.to_string();
        }
        format!(
            "{}{}{}",
            &hash[0..first_len],
            dots,               
            &hash[hash.len() - last_len..]
        )
    }
    
    

    fn draw_line(commands: &mut Commands, point1: (f32, f32), point5: (f32, f32), thickness: f32) {
        
        let mid_point = (
            (point1.0 + point5.0) / 2.0,
            (point1.1 + point5.1) / 2.0,
        );
    
        // Calculate the angle of rotation
        let angle = (point5.1 - point1.1).atan2(point5.0 - point1.0);
    
        // Calculate the distance between the two points
        let length = ((point5.0 - point1.0).powi(2) + (point5.1 - point1.1).powi(2)).sqrt();
    
        // Spawn the line (as a scaled and rotated sprite)
        commands.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(mid_point.0, mid_point.1, 0.0),
                rotation: Quat::from_rotation_z(angle),           
                scale: Vec3::new(length, thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BranchMarker);

        println!("Drawing Line from {:?} to {:?}", point1, point5);
    }

    fn draw_node(commands: &mut Commands, node: &Node, size: (f32, f32),handle: &Handle<Font>, is_hash: bool) {
        let (x, y) = node.position;
        let (size_x,size_y) = size;

        let display_text = if is_hash {
            Self::format_hash(&node.hash, 6, 6, "...")
        } else {
            node.hash.clone()
        };

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                ..Default::default()
            },
            //color: Color::srgb(0.5, 0.5, 1.0)
            transform: Transform {
                translation: Vec3::new(x, y, 0.0),
                scale: Vec3::new(size_x,size_y, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(node.clone());

         // Adjust text size based on content length
        let calculated_font_size = (size_y * 0.5).min(size_x / (display_text.len() as f32 * 0.5));

        // Draw the text
        commands.spawn(Text2dBundle {
            text: Text::from_section(display_text, 
            TextStyle { 
                font: handle.clone(),
                 font_size: calculated_font_size, color: Color::WHITE }),
            transform: Transform::from_translation(Vec3::new(x, y, 10.0)),
            ..Default::default()
        })
        .insert(NodeTextMarker); 
    }
    
}

impl Default for MerkleTree {
    fn default() -> Self {
        MerkleTree{
            root: None,
            nodes: HashMap::new(),
            words: Vec::new(),
            levels: 3,
            size: (0.0,0.0)
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node{
            hash: "".to_string(),
            level: 0,
            parent_index: (0, 0),
            index: 0,
            position: (0.0, 0.0),
            start_position: (0.0, 0.0),
            end_position: (0.0, 0.0)
        }
    }
}

fn pad_words(words: Vec<&str>) -> Vec<&str> {
    let mut padded_words = words.clone();
    let len = padded_words.len();

    if len > 4 && len < 8 {
        padded_words.resize(8, "<pad>");
    } 
    else if len > 8 && len < 16 {
        padded_words.resize(16, "<pad>");
    }

    padded_words
}

pub fn build_tree(words: Vec<&str>) -> MerkleTree {
    let padded_words = pad_words(words);

    let hashes = hasher::hash_words(padded_words.clone());
    let mut tree = MerkleTree::new(&hashes,
        padded_words.iter().map(|s| s.to_string()).collect());
    tree.build();
    return tree;
}
