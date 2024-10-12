use bevy::{prelude::*, window::{PrimaryWindow, WindowPlugin}};
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
        .add_systems(Update, (check_keyboards, sprite_update, text_bar_update, update_loop_text, update_loop_tree))
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
        }
    }
}

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
}


// add system to track changes in button state based on text input
fn check_keyboards(mut state: ResMut<GameState>, input: Res<ButtonInput<KeyCode>>, commands: Commands){
    if input.just_pressed(KeyCode::Space) && state.toggle_input {
        state.current_text.push(' ');
    }

    if input.just_pressed(KeyCode::Backspace) && !state.current_text.is_empty() && state.toggle_input{
        state.current_text.pop();
    }

    // Set current text to previous text
    if input.just_pressed(KeyCode::Enter) && state.toggle_input {
        state.toggle_input = false;
        state.previous_text = state.display_text.clone();
        state.display_text = state.current_text.clone();


        let hash = hasher::keccak_256(&state.display_text.clone());
         println!("Hash: {}", hash);

        let clone = state.display_text.clone();
        let words = clone.split_whitespace().collect::<Vec<&str>>();
        println!("Words: {:?}", words);

        // let hashes = hasher::hash_words(words);
        // println!("Hashes: {:?}", hashes);

        let mut tree = tree::build_tree(words);

        println!("Tree: {:?}", tree);
        tree.graph(600.0, commands, &state.handle);

        // After building tree, generate the graph

        // After building tree, we move to displaying it.
        // Now the algorithm for displaying will be words first, then hashes, as we go down the levels
        // Then we get more narrower as we move towards the root.
        // Focus on building with 4 words first, then we expand

        // After, we move on ot editing the words in the trees and seeing which ones change
        // that will include som midly heave bevy use

        return;
    }

    if input.just_pressed(KeyCode::KeyI) && !state.toggle_input {
        state.toggle_input = true;
        // de spawn all tree nodes
        return;
    }

    if input.just_pressed(KeyCode::KeyL) {
        println!("Toggle State: {}, Current Text: {}, Display Text: {}, Previous Text: {}", state.toggle_input,state.current_text, state.display_text, state.previous_text);
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

    let shift_held = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);
    let caps_lock_on = input.pressed(KeyCode::CapsLock);

    for (key_code, letter) in letters.iter() {
        if input.just_pressed(*key_code) && state.toggle_input {
            let mut letter_to_add = *letter;

            if shift_held || caps_lock_on {
                letter_to_add = letter_to_add.to_ascii_uppercase();
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
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if state.toggle_input {
        for (entity, _transform, _sprite, _node) in query.iter_mut() {
            commands.entity(entity).despawn();
        }
    }

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
    //println!("Before entity check, Mouse Position: {:?}", state.mouse_position );


    // Loop through node entities and check if the mouse is within the node bounds
    for (_entity, transform,  sprite_option, node_option) in query.iter_mut() {
        if let Some(node) = node_option {
            if let Some(mut sprite) = sprite_option {

                if node.is_hash{
                    continue;
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

                if within_x_bounds && within_y_bounds {
                    println!("Mouse is hovering over node with text: {} at position: {:?}", node.hash,node_position);
                    sprite.color = Color::srgb(0.8, 0.8, 0.2);
                }
                else {
                    sprite.color = Color::BLACK;
                }
            } else {
                println!("Entity without a sprite, skipping bounds check");
            }
        }
    }
}



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
