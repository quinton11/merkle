use bevy::{prelude::*, window::{PrimaryWindow, WindowPlugin}};
use tree::MerkleTree;
//use keccak_hash;

mod hasher;
mod tree;

//.insert_resource(ClearColor(Color::srgb(0.1216, 0.2039, 0.3451)))

fn main() {
    App::new()
    .init_resource::<GameState>()
    .insert_resource(ClearColor(Color::WHITE))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Merkle".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }))
        .add_systems(Startup, start_up)
        .add_systems(Update, (check_keyboards, sprite_update, text_bar_update, update_loop_text, update_loop_tree.after(button_system), button_system))
        .run();
}

#[derive(Resource)]
struct GameState{
    pub current_text: String,
    pub display_text: String,
    pub previous_text: String,
    pub handle: Handle<Font>,
    pub toggle_input: bool,
    pub mouse_position: (f32, f32),
    pub tree: MerkleTree,
    pub select_node: bool,
    pub selected_node: Option<tree::Node>,
    pub mode: MerkleMode,
    pub hovered_button: Option<Entity>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState{
            current_text: "Binary Merkle Tree Demo".to_string(),
            display_text: "Binary Merkle Tree Demo".to_string(),
            previous_text: "".to_string(),
            handle: Handle::default(),
            toggle_input: true,
            mouse_position: (0.0, 0.0),
            tree: MerkleTree { 
                ..Default::default()
             },
            select_node: false,
            selected_node: None,
            mode: MerkleMode::BuildTree,
            hovered_button: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum MerkleMode{ 
    InclusionProof,
    RebuildTree,
    BuildTree
}

const MERKLE_MODE_STRINGS: [&str; 3] = ["Proof", "Rebuild", "Build"];
const BUTTON_HOVER_COLOR: Color = Color::BLACK;

#[derive(Component)]
struct TextBarMarker;

#[derive(Component)]
struct TextBarTextMarker;



fn start_up(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<GameState>){
    let font_handle = asset_server.load("fonts/JetBrainsMono-Regular.ttf");
    let input_texture = asset_server.load("images/input_box.png");
    state.handle = font_handle.clone();

    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        ..Default::default()
    });


    // Spawn opaque layer
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::NONE, // Initially hidden
            custom_size: Some(Vec2::new(800.0, 600.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(TextBarMarker)
    .with_children(|parent| {
        parent.spawn(SpriteBundle {
            texture: input_texture.clone(),
            sprite: Sprite {
                color: Color::NONE,
                custom_size: Some(Vec2::new(700.0, 50.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 200.0, 3.0),
            ..Default::default()
        });
    });

    // Spawn input/display text
    commands.spawn(Text2dBundle {
        text: Text::from_section(state.display_text.clone(), TextStyle { font: font_handle.clone(), font_size: 18.0, color: Color::WHITE }),
        transform: Transform::from_translation(Vec3::new(0.0, 200.0, 4.0)),
        ..Default::default()
    })
    .insert(TextBarTextMarker);



    // Spawn mode buttons
    commands
    .spawn(NodeBundle {
        style: Style {
            width: Val::Auto,
            height: Val::Auto,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            top: Val::Px(50.0), 
            right: Val::Px(90.0),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        // Proof Button
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(40.0),
                margin: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..default()
                },
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            border_radius: BorderRadius{ top_left: Val::Px(5.0), top_right: Val::Px(5.0), bottom_left:Val::Px(5.0), bottom_right: Val::Px(5.0)},
            background_color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Proof",
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ));
        });

        // Rebuild Button
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(40.0),
                margin: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..default()
                },
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            border_radius: BorderRadius{ top_left: Val::Px(5.0), top_right: Val::Px(5.0), bottom_left:Val::Px(5.0), bottom_right: Val::Px(5.0)},
            background_color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Rebuild",
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ));
        });

        // Build Button
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(40.0),
                margin: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..default()
                },
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            border_radius: BorderRadius{ top_left: Val::Px(5.0), top_right: Val::Px(5.0), bottom_left:Val::Px(5.0), bottom_right: Val::Px(5.0)},
            background_color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Build",
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ));
        });
    });

}


fn button_system(
    mut query_set: ParamSet<(
        Query<(&mut BackgroundColor, &mut BorderColor, &Children, Entity), With<Button>>,  // For all buttons
        Query<(&Interaction, &mut BackgroundColor, &mut BorderColor, &Children, Entity), (Changed<Interaction>, With<Button>)>,  // For interaction changes
    )>,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<GameState>,
) {

    let mut interaction_happened = false;

    //println!("Interaction triggered!");
    for (interaction, mut background_color, mut border_color, children, entity) in query_set.p1().iter_mut() {
        if let Ok(mut text) = text_query.get_mut(children[0]) {
            //println!("Interaction text: {:?}",text);
            match *interaction {
                Interaction::Pressed => {
                    // Check which button was pressed based on the text
                    let proof = MERKLE_MODE_STRINGS[0];
                    let rebuild = MERKLE_MODE_STRINGS[1];
                    let build = MERKLE_MODE_STRINGS[2];

                    // Update state based on button text
                    if text.sections[0].value.as_str() == proof {
                        state.mode = MerkleMode::InclusionProof;
                        let selected_node_exists = state.selected_node.is_some();
                        if selected_node_exists {
                            let selected_node = state.selected_node.as_ref().map_or(0, |node| node.index);
                            state.tree.inclusion_proof(selected_node);
                        }

                        // call state.tree.inclusion_proof() which will set the proof nodes to be highlighted, i.e the path to the root
                        // So the level and the index of the nodes to be highlighted
                    } else if text.sections[0].value.as_str() == rebuild {
                        state.mode = MerkleMode::RebuildTree;
                        state.tree.proof = None;
                    } else if text.sections[0].value.as_str() == build {
                        state.mode = MerkleMode::BuildTree;
                    }


                    // Change the button border to indicate selection
                    border_color.0 = BUTTON_HOVER_COLOR;
                    *background_color = BUTTON_HOVER_COLOR.into();
                    text.sections[0].style.color = Color::WHITE;
                    interaction_happened = true;

                }
                Interaction::Hovered => {
                    // Highlight button on hover
                    println!("Hovered changing border colour");
                    border_color.0 = BUTTON_HOVER_COLOR;
                    *background_color = BUTTON_HOVER_COLOR.into();
                    text.sections[0].style.color = Color::WHITE;
                    interaction_happened = true;
                    state.hovered_button = Some(entity);

                }
                Interaction::None => {
                    if state.hovered_button == Some(entity) {
                        border_color.0 = Color::BLACK;
                        *background_color = Color::WHITE.into();
                        text.sections[0].style.color = Color::BLACK;
                        state.hovered_button = None;
                    };

                    println!("Mode: {:?}",state.mode);
                }
            }
        }
    }

    if !interaction_happened{
        for (mut background_color, mut border_color, children, entity) in query_set.p0().iter_mut() {
            if let Ok(mut text) = text_query.get_mut(children[0]) {
    
                border_color.0 = Color::BLACK;
                *background_color = Color::WHITE.into();
                text.sections[0].style.color = Color::BLACK;

                if let Some(hovered_button) = state.hovered_button {
                    if hovered_button == entity {
                        border_color.0 = BUTTON_HOVER_COLOR;
                        *background_color = BUTTON_HOVER_COLOR.into();
                        text.sections[0].style.color = Color::WHITE;
                        continue; 
                    }
                }

                if state.mode == MerkleMode::InclusionProof && text.sections[0].value.as_str() == MERKLE_MODE_STRINGS[0] {
                    border_color.0 = BUTTON_HOVER_COLOR;
                    *background_color = BUTTON_HOVER_COLOR.into();
                    text.sections[0].style.color = Color::WHITE;
                } else if state.mode == MerkleMode::RebuildTree && text.sections[0].value.as_str() == MERKLE_MODE_STRINGS[1] {
                    border_color.0 = BUTTON_HOVER_COLOR;
                    *background_color = BUTTON_HOVER_COLOR.into();
                    text.sections[0].style.color = Color::WHITE;
                } else if state.mode == MerkleMode::BuildTree && text.sections[0].value.as_str() == MERKLE_MODE_STRINGS[2] {
                    border_color.0 = BUTTON_HOVER_COLOR;
                    *background_color = BUTTON_HOVER_COLOR.into();
                    text.sections[0].style.color = Color::WHITE;
                }
            }
        }
    }

}




// add system to track changes in button state based on text input
fn check_keyboards(mut state: ResMut<GameState>, input: Res<ButtonInput<KeyCode>>, commands: Commands){
    if input.just_pressed(KeyCode::Space) && state.toggle_input {
        state.current_text.push(' ');
    }

    let backspace_allowed = match state.mode {
        MerkleMode::RebuildTree => {
            if let Some(selected_node) = &state.selected_node {
                !selected_node.hash.is_empty()
            } else {
                false
            }
        },
        _ => {
            !state.current_text.is_empty()
        }
    };

    if input.just_pressed(KeyCode::Backspace) && backspace_allowed && state.toggle_input{
        if state.mode == MerkleMode::RebuildTree {
            if let Some(selected_node) = &mut state.selected_node {
                selected_node.hash.pop();
            }
        } else {
            state.current_text.pop();

        }
    }

    let shift_held = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);
    let caps_lock_on = input.pressed(KeyCode::CapsLock);

    // Set current text to previous text
    if input.just_pressed(KeyCode::Enter) && state.toggle_input {

        match state.mode {
            MerkleMode::BuildTree => {
                state.toggle_input = false;
                state.previous_text = state.display_text.clone();
                state.display_text = state.current_text.clone();
        
        
                let hash = hasher::keccak_256(&state.display_text.clone());
                 println!("Hash: {}", hash);
        
                let clone = state.display_text.clone();
                let words = clone.split_whitespace().collect::<Vec<&str>>();
                //println!("Words: {:?}", words);
        
                // let hashes = hasher::hash_words(words);
                // println!("Hashes: {:?}", hashes);
        
                let mut tree = tree::build_tree(words);
        
                println!("Tree: {:?}", tree);
        
                tree.graph(600.0, commands, &state.handle);
        
                state.tree = tree;
        
                // After building tree, generate the graph
        
                // After building tree, we move to displaying it.
                // Now the algorithm for displaying will be words first, then hashes, as we go down the levels
                // Then we get more narrower as we move towards the root.
                // Focus on building with 4 words first, then we expand
            },
            MerkleMode::RebuildTree => {
                println!("Rebuilding Tree");
                if let Some(selected_node) = &state.selected_node {

                    // if shift is held, instead of rebuilding the tree, we store the new word in the tree object
                    // and generate the proof, then we indicate if the word belongs in the tree or not
                    // then when we're rendering we can show the word in red if it doesn't belong in the tree
                    // else we show it in green if it does belong
                    // then when we render the nodes, we render the edited word in that node
                    // so the difference is that we don't rebuild the tree with the new word, we just store the new word
                    // either way we store the new word in the tree object and generate the proof

                    let index = selected_node.index;
                    let new_word = selected_node.hash.clone();

                    let words = state.tree.words.clone();
                    
                    // if shift is held
                    if shift_held {
                        
                        state.mode = MerkleMode::RebuildTree;
                        state.toggle_input = false;
                        let words_vec_str =  words.iter().map(|s| s.as_str()).collect();
                        let mut tree = tree::build_tree(words_vec_str);
                        tree.graph(600.0, commands, &state.handle);
                        state.tree = tree;
                        let hash = hasher::keccak_256(&new_word);
                        state.tree.word_to_prove = Some(tree::WordToProve{
                            index,
                            word: new_word.clone(),
                            hash: hash.clone(),
                            display_hash: tree::MerkleTree::format_hash(&hash, 6, 6, "...")
                        });
                        return;
                    }



                    if let Some(word) = state.tree.words.get_mut(index) {
                        *word = new_word.clone();
                    }

                    let words = state.tree.words.clone();
                    

                    state.current_text = words.join(" ");
                    state.display_text = state.current_text.clone();
                    let words_vec_str =  words.iter().map(|s| s.as_str()).collect();
                    let mut tree = tree::build_tree(words_vec_str);
                    tree.graph(600.0, commands, &state.handle);
                    state.tree = tree;

                    state.mode = MerkleMode::BuildTree;
                    state.toggle_input = false;
                    println!("Tree: {:?}", state.tree);
                }
            },
            MerkleMode::InclusionProof => {
                state.mode = MerkleMode::BuildTree;
                // with the inclusion proof its similar to rebuilding the tree, this time, we don't
                // change the current text nor the display text, we just rebuild the tree
                // and highlight the path to the root of the tree, i.e the nodes that are included in the proof
                // we highlight the nodes included with green and the node with red if its proof failed
                // Basically we want to show if that node's hash is included in the tree
            }
        }


        return;
    }

    if input.just_pressed(KeyCode::KeyI) && !state.toggle_input && state.mode != MerkleMode::InclusionProof {
        state.toggle_input = true;
        //state.mode = MerkleMode::BuildTree;
        // de spawn all tree nodes
        return;
    }

    if input.just_pressed(KeyCode::KeyL) {
        println!("Toggle State: {}, Current Text: {}, Display Text: {}, Previous Text: {}", state.toggle_input,state.current_text, state.display_text, state.previous_text);
        println!("Word to Prove: {:?}", state.tree.word_to_prove);
    }

    let letters = [
        (KeyCode::KeyA, 'a'), (KeyCode::KeyB, 'b'), (KeyCode::KeyC, 'c'), (KeyCode::KeyD, 'd'),
        (KeyCode::KeyE, 'e'), (KeyCode::KeyF, 'f'), (KeyCode::KeyG, 'g'), (KeyCode::KeyH, 'h'),
        (KeyCode::KeyI, 'i'), (KeyCode::KeyJ, 'j'), (KeyCode::KeyK, 'k'), (KeyCode::KeyL, 'l'),
        (KeyCode::KeyM, 'm'), (KeyCode::KeyN, 'n'), (KeyCode::KeyO, 'o'), (KeyCode::KeyP, 'p'),
        (KeyCode::KeyQ, 'q'), (KeyCode::KeyR, 'r'), (KeyCode::KeyS, 's'), (KeyCode::KeyT, 't'),
        (KeyCode::KeyU, 'u'), (KeyCode::KeyV, 'v'), (KeyCode::KeyW, 'w'), (KeyCode::KeyX, 'x'),
        (KeyCode::KeyY, 'y'), (KeyCode::KeyZ, 'z'),
    ];



    for (key_code, letter) in letters.iter() {
        if input.just_pressed(*key_code) && state.toggle_input {
            let mut letter_to_add = *letter;

            if shift_held || caps_lock_on {
                letter_to_add = letter_to_add.to_ascii_uppercase();
            }

            // If we are in rebuild tree mode, we add the letter to the selected node
            if state.mode == MerkleMode::RebuildTree {
                if let Some(selected_node) = &mut state.selected_node {
                    selected_node.hash.push(letter_to_add);
                }
                continue;
            }
            state.current_text.push(letter_to_add);
        }
    }


}

fn update_loop_text(
    state: ResMut<GameState>, 
    mut query: Query<(&mut Transform, &mut Text), With<TextBarTextMarker>> 
) {
    for (_transform, mut text) in query.iter_mut() {
        if state.toggle_input {
            // if mode is rebuild tree, then display the selected node text
            // else display the current text

            //println!("Mode in update loop text: {:?}", state.mode);
            if state.mode == MerkleMode::RebuildTree || state.mode == MerkleMode::InclusionProof {
                text.sections[0].value = state.selected_node.as_ref().map_or("".to_string(), |node| node.hash.clone());
                text.sections[0].style.color = Color::WHITE;
                continue;
            }

            text.sections[0].value = state.current_text.clone(); 
            text.sections[0].style.color = Color::WHITE;       
        } else {
            text.sections[0].value = state.display_text.clone();
            text.sections[0].style.color = Color::BLACK;      
        }
    }
}


fn update_loop_tree(
    mut state: ResMut<GameState>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &Transform, Option<&mut Sprite>, Option<&tree::Node>), Or<(With<tree::Node>, With<tree::BranchMarker>, With<tree::NodeTextMarker>)>>, // Add optional Sprite
    mut commands: Commands,
    mut query_text: Query<(Entity, &mut Text, &tree::NodeTextMarker)>, 
    q_camera: Query<(&Camera, &GlobalTransform)>,
    input: Res<ButtonInput<MouseButton>>
) {
    if state.toggle_input {
        for (entity, _transform, _sprite,_node) in query.iter_mut() {
            commands.entity(entity).despawn();
        }
    }

    let mut mouse_clicked = false;

    if let Some(position) = q_windows.single().cursor_position() {
        let (camera, camera_transform) = q_camera.single();

        let window_size = Vec2::new(q_windows.single().width() as f32, q_windows.single().height() as f32);

        // Convert screen position (origin is top-left) to normalized device coordinates (NDC) (-1 to +1 range)
        let mut ndc = (position / window_size) * 2.0 - Vec2::ONE;

        ndc.y = -ndc.y;

        //  NDC to world space coordinates
        if let Some(world_position) = camera.ndc_to_world(camera_transform, ndc.extend(-1.0)) {
            state.mouse_position = (world_position.x, world_position.y);
        }
    }

    if input.just_pressed(MouseButton::Left){
        println!("Mouse Clicked at: {:?}", state.mouse_position);
        mouse_clicked = true;
    }

    let mut clicked_on_node = false;


    // Loop through node entities and check if the mouse is within the node bounds
    for (_entity, transform,  sprite_option, node_option) in query.iter_mut() {
        if let Some(node) = node_option {
            if let Some(mut sprite) = sprite_option {

                if node.is_hash{
                    // based on the mode, we can highlight the nodes that are part of the inclusion proof
                    // so if mode is inclusion proof, we highlight the nodes that are part of the proof
                    if state.mode == MerkleMode::InclusionProof {
                        if state.tree.proof.is_some() {
                            if let Some(proof) = &state.tree.proof {
                                for proof_link in &proof.proof_link {
                                    // Safely check if the level exists in the proof_link map
                                    if let Some(&sibling_index) = proof_link.get(&(node.level as u32)) {
                                        // Compare the sibling index to the node index
                                        if sibling_index == node.index as u32 {
                                            sprite.color = Color::srgb(0.2, 0.8, 0.2); // Green highlight for nodes in the inclusion proof
                                        }
                                    }
                                }
                            }
                            // Now based on proof_links, we can change the color of the nodes that are part of the proof to green and their text to black
                            // and if the proof is valid, we change the color of the word node to green and the text to black else we change the color to red
                        }
                    }else {
                        sprite.color = Color::BLACK;
                    }

                    if let Some(word_to_prove) = &state.tree.word_to_prove {
                        if word_to_prove.index == node.index && node.level == 1 {
                            if let Some((_, mut text, _)) = query_text.iter_mut().find(|(_, _, marker)| marker.node_index == node.index && marker.node_level == node.level) {
                                text.sections[0].value = word_to_prove.display_hash.clone();
                            }
                        }
                    }
                
                    continue;
                }

                if let Some(word_to_prove) = &state.tree.word_to_prove {
                    if word_to_prove.index == node.index && node.level == 0 {
                        // Find the corresponding text entity by using NodeTextMarker
                        if let Some((_, mut text, _)) = query_text.iter_mut().find(|(_, _, marker)| marker.node_index == node.index && marker.node_level == node.level) {
                            //println!("Updating Text to Word to Prove: {:?}, Node: {:?}", text.sections[0].value, node);
                            text.sections[0].value = word_to_prove.word.clone(); // Update the text to the word to prove
                        }
                    }
                }

                let node_position = transform.translation;
                let node_size = transform.scale;

                let tolerance = 5.0;
                let half_width = (node_size.x / 2.0) + tolerance;
                let half_height = (node_size.y / 2.0) + tolerance;

                let within_x_bounds = state.mouse_position.0 >= node_position.x - half_width &&
                state.mouse_position.0 <= node_position.x + half_width;
                let within_y_bounds = state.mouse_position.1 >= node_position.y - half_height &&
                state.mouse_position.1 <= node_position.y + half_height;

                let within_bounds = within_x_bounds && within_y_bounds;


                if within_bounds {
                    //println!("Mouse is hovering over node with text: {} at position: {:?}", node.hash, node_position);
                    
                    // If no node is selected or this node is not the selected one, highlight on hover
                    if state.selected_node.is_none() || 
                    state.selected_node.as_ref().map_or(true, |selected_node| selected_node.hash != node.hash) {
                        sprite.color = Color::srgb(0.8, 0.8, 0.2); // Yellow highlight color on 
                    }

                    if mouse_clicked {
                        state.selected_node = Some(node.clone());
                        state.select_node = true;
                        sprite.color = Color::srgb(0.2, 0.2, 0.2); // Dark gray color on click
                        clicked_on_node = true;
                        state.mode = MerkleMode::RebuildTree;
                        state.tree.proof = None;
                        println!("Mode: {:?}", state.mode);

                    }
                } else if mouse_clicked && !clicked_on_node {
                    // Clicked outside, deselect the node
                    if state.mode == MerkleMode::InclusionProof{
                        continue;
                    }
                    state.select_node = false;
                    if let Some(selected_node) = &state.selected_node {
                        if selected_node.hash == node.hash {
                            sprite.color = Color::BLACK;
                            state.selected_node = None;
                        }
                    }
                    state.mode = MerkleMode::BuildTree;

                } else if !state.select_node || 
                state.selected_node.as_ref().map_or(true, |selected_node| selected_node.hash != node.hash) {
                   
                    sprite.color = Color::BLACK;
                }

                if let Some(word_to_prove) = &state.tree.word_to_prove {
                    if word_to_prove.index == node.index && node.level == 0 {
                        if let Some(proof) = &state.tree.proof  {
                            if proof.is_valid {
                                sprite.color = Color::srgb(0.2, 0.8, 0.2);
                            } else {
                                sprite.color = Color::srgb(0.8, 0.2, 0.2);

                            }
                        }
                    }
                }



                // highlight selected node
            } else {
                println!("Entity without a sprite, skipping bounds check");
            }


        }
    }
}

                // if state.mode == MerkleMode::InclusionProof {
                //     if state.tree.proof.is_some() {
                //         if let Some(proof) = &state.tree.proof {

                //             if let Some(node_sel) = &state.selected_node {
                //                 if node.index == node_sel.index {
                //                     if proof.is_valid {
                //                         sprite.color = Color::srgb(0.2, 0.8, 0.2); // Green highlight for nodes in the inclusion proof
                //                     } else {
                //                         sprite.color = Color::srgb(0.8, 0.2, 0.2); // Red highlight for nodes in the inclusion proof
                //                     }
                //                 }
                //             }
                //             sprite.color = Color::srgb(0.2, 0.8, 0.2); // Green highlight for nodes in the inclusion proof

                //         }
                //         // Now based on proof_links, we can change the color of the nodes that are part of the proof to green and their text to black
                //         // and if the proof is valid, we change the color of the word node to green and the text to black else we change the color to red
                //     }
                // }else {
                //     sprite.color = Color::BLACK;
                // }



fn sprite_update(state: ResMut<GameState>, mut query: Query<&mut Sprite, With<TextBarMarker>>, window_query: Query<&Window, With<PrimaryWindow>>){
    let window = window_query.iter().next().unwrap();
    for mut sprite in query.iter_mut(){
        if state.toggle_input {
            sprite.color = Color::srgba(0.12, 0.12, 0.12, 0.77);
            sprite.custom_size = Some(Vec2::new(window.width(), window.height()));
        } else {
            sprite.color = Color::NONE;
        }
    }
}

fn text_bar_update(state: Res<GameState>, mut param_set: ParamSet<(
    Query<(&Children, &Sprite), With<TextBarMarker>>,
    Query<&mut Sprite>,
)>){

    let mut children_to_update = vec![];

    {
        let parent_query = param_set.p0();
        for (children, _) in parent_query.iter() {
            for &child in children.iter() {
                children_to_update.push(child);
            }
        }
    }

    let mut child_query = param_set.p1();
    for child in children_to_update {
        if let Ok(mut child_sprite) = child_query.get_mut(child) {
            if state.toggle_input {
                child_sprite.color = Color::srgba(1.0, 0.12, 0.12, 1.0);
            } else {
                child_sprite.color = Color::NONE;
            }
        }
    }
}



// How to build the merkle tree? First understand how the process works.
// What makes up the tree, what do we need to break down, word by word, character by character?
// How do we build the tree from the ground up?  https://en.wikipedia.org/wiki/Merkle_tree
// Then do select on click and highlight the path to the root of the tree
// Then when we click on a node, we can see the hash of the node, and the hash of the children
// When a node is highlighted, we can edit the text of the node and see the hash update in real time
// We can also do inclusion proofs, where we can see the path to the root of the tree, if the edited node
// belongs to the tree or not
// Basically verify a node, with specific hashes we can see from its path to the root of the tree


// Now based on proof_links, we can change the color of the nodes that are part of the proof to green and their text to black
// and if the proof is valid, we change the color of the word node to green and the text to black else we change the color to red



// For the proof out of place, we don't rebuild the new word with the tree, we simply store the new word in the tree object along
// with the index of the word it is to replace, then we generate the proof as always and indicate if the word belongs in that tree
// Then when we're rendering we can show the word in red if it doesn't belong in the tree, else we show it in green if it does belong
// Then whe we render the nodes, we render the edited word in that node