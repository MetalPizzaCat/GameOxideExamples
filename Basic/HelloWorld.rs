/**
 * This program creates a simple window of 800x600 size that displays "Hello world!" in white text
 * This is meant to showcase most basic setup for displaying anything
 * This needs a font named Roboto-Regular located in work folder
 * This targets https://github.com/MetalPizzaCat/GameOxideFramework/commit/347598408ed76b57a6cc829fcc68cbc1b6e73b37 build of framework
 */
use game_oxide_framework::{components::*, render::*, texture_manager::*};
//this framework relies on nalgebra to allow access to math
use nalgebra::Vector2;
//for handling events
use sdl2::{event::Event, EventPump};
//For creating ECS world
use specs::{Builder, WorldExt};

//Main returns empty object or error string
fn main() -> Result<(), String> {
    //This function creates a tuple that contains all of the items that you need for working with this framework
    //
    let (mut world, sdl, video, ttf_context, mut canvas, mut game) =
        game_oxide_framework::setup::setup("Hello world!".to_owned(), Some(Vector2::new(800, 600)))
            .unwrap_or_else(|e| panic!("{}", e.to_string()));

    //Get the event pump to get
    let mut event_pump: EventPump = sdl.event_pump().unwrap();
    //Texture creator and texture manager are used to handle and load textures
    let mut texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext> =
        canvas.texture_creator();
    let mut texture_manager: TextureManager = TextureManager::new(&texture_creator)?;

    //The actual font used for displaying text
    //Load it from file and set size to 50
    let font = ttf_context.load_font("./Roboto-Regular.ttf", 50)?;

    //Create text object
    //For text object to be recognised it has to have *at least*
    //Position component
    //Text component
    //Renderable component
    let hello_world_text = world
        .create_entity()
        .with(Position { x: 10, y: 10 })
        .with(Text {
            text: "Hello world!".to_owned(),
            color: sdl2::pixels::Color::RGB(255, 255, 255),
            visible: true,
            offset: Vector2::new(0, 0),
        })
        .with(Renderable {
            visible: true,
            layer: game_oxide_framework::layers::RenderLayers::Gameplay as u32,
        })
        .build();
    //Simple look that breaks when player exist(like pressing x on the window title or doing alt+f4)
    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'game;
                }
                _ => {}
            }
        }
        //Render everything to screen
        //This is a helper function
        render_game(&world, &mut canvas, &texture_manager, &mut game, &font)?;
    }
    Ok(())
}
