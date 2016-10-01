
pub struct Game {
    components : GameComponentCollection,
    services   : GameServiceContainer,
    content    : ContentManager,
    platform   : GamePlatform
}

impl Game {
    pub fn new() -> Self {
        let services = GameServiceContainer::new();
        
        let mut game_builder = Game {
            components : GameComponentCollection::new(),
            services   : services,
            content    : ContentManager::new( services )
        }

        let mut platform = GamePlatform::create( game_builder );

        game_builder.platform = platform;

        let game = game_builder;

        game
    }
}
