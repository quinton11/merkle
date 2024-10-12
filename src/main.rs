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
}

impl Default for GameState {
    fn default() -> Self {
        GameState{
            current_text: "Binary Merkle Tree Demo".to_string(),
            display_text: "Binary Merkle Tree Demo".to_string(),
            previous_text: "".to_string(),
            handle: Handle::default(),
            toggle_input: true,
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
    mut query: Query<(&mut Transform, &mut Text), With<TextBarTextMarker>>  // Only target entities with `TextBarTextMarker`
) {
    for (_transform, mut text) in query.iter_mut() {
        if state.toggle_input {
            // Update the text and color when toggle_input is true
            text.sections[0].value = state.current_text.clone(); // Update text content
            text.sections[0].style.color = Color::WHITE;         // Set text color to white
        } else {
            // Update the text and color when toggle_input is false
            text.sections[0].value = state.display_text.clone(); // Update text content
            text.sections[0].style.color = Color::BLACK;         // Set text color to black
        }
    }
}


fn update_loop_tree(
    state: Res<GameState>, 
    mut query: Query<(Entity, &Transform), Or<(With<tree::Node>, With<tree::BranchMarker>, With<tree::NodeTextMarker>)>>,
    mut commands: Commands,
) {
    if !state.toggle_input {
        return;
    }

    for (entity, _transform) in query.iter_mut() {
        commands.entity(entity).despawn();
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
