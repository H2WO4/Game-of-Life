#![feature(is_some_with)]
#![allow(dead_code)]

use bevy::prelude::*;
use bevy::winit::WinitSettings;

mod universe {
    use std::ops::{Deref, DerefMut};

    use grid::Grid;

    use super::*;

    #[derive(Component, Debug, Clone)]
    pub struct Cell(pub bool);
    impl Deref for Cell {
        type Target = bool;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl DerefMut for Cell {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl Cell {
        pub const ALIVE: Self = Self(true);
        pub const DEAD: Self = Self(false);
    }

    #[derive(Component, Debug)]
    pub struct Universe {
        pub cells: Grid<Cell>,

        rules: rules::Rules,
    }
    impl Deref for Universe {
        type Target = Grid<Cell>;

        fn deref(&self) -> &Self::Target {
            &self.cells
        }
    }
    impl DerefMut for Universe {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.cells
        }
    }
    impl Universe {
        pub fn new(width: usize, height: usize, rules: rules::Rules) -> Self {
            Self { cells: Grid::init(width, height, Cell(false)),
                   rules }
        }

        fn get_neighbors(&self, x: usize, y: usize) -> [Option<&Cell>; 8] {
            [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].map(|(i, j)| {
                                                                                      self.cells
                                                                                          .get((x as isize + i) as usize, (y as isize + j) as usize)
                                                                                  })
        }

        pub fn do_tick(&mut self) {
            let (width, height) = self.size();

            // TODO: Multithreading
            let mut new_cells = Grid::init(width, height, Cell(false));
            for x in 0..width {
                for y in 0..height {
                    let cell = new_cells.get_mut(x, y).unwrap();
                    let neighbors = self.get_neighbors(x, y);

                    cell.0 = self.rules.rules_arr[neighbors.iter()
                                                           .fold(0u8, |v, c| (v << 1) + (c.is_some_and(|c| c.0) as u8)) as usize];
                }
            }

            self.cells = new_cells;
        }
    }
}

mod rules {
    use itertools::Itertools;
    use lazy_regex::{regex, Lazy, Regex};

    use super::*;

    static RULES_REGEX: &Lazy<Regex> = regex!(r"^B([0-8]+)/S([0-8]+)$");

    #[derive(Component, Debug, Clone)]
    pub struct Rules {
        pub name:     String,
        pub sys_name: String,

        pub survive: Box<[u8]>,
        pub birth:   Box<[u8]>,

        pub rules_arr: [bool; 256],
    }
    impl Rules {
        pub fn new(birth: &[u8], survive: &[u8]) -> Self {
            let fused = birth.iter()
                             .chain(survive.iter())
                             .copied()
                             .unique()
                             .collect::<Vec<_>>();

            let rules_arr: [bool; 256] = core::array::from_fn(|x| {
                let alive_neighbors = x.count_ones() as u8;

                fused.contains(&alive_neighbors)
            });

            Self { name: "".into(),
                   sys_name: "".into(),

                   birth: birth.into(),
                   survive: survive.into(),

                   rules_arr }
        }

        pub fn from_string(input: &str) -> Result<Self, &'static str> {
            let (birth, survive) = if let Some(captures) = RULES_REGEX.captures(input) {
                (captures[1].chars()
                            .into_iter()
                            .filter_map(|c| c.to_digit(10).map(|x| x as u8))
                            .collect::<Vec<_>>(),
                 captures[2].chars()
                            .into_iter()
                            .filter_map(|c| c.to_digit(10).map(|x| x as u8))
                            .collect::<Vec<_>>())
            } else {
                return Err("Invalid rules")
            };

            Ok(Self::new(&birth, &survive).with_sys_name(input))
        }

        pub fn with_name(self, name: &str) -> Self {
            Self { name: name.into(), ..self }
        }

        pub fn with_sys_name(self, sys_name: &str) -> Self {
            Self { sys_name: sys_name.into(), ..self }
        }
    }
}

mod graphics {
    use super::*;

    pub fn setup(mut commands: Commands/* , asset_server: Res<AssetServer> */) {
        commands.spawn_bundle(Camera2dBundle::default());

        // commands.spawn_bundle(ButtonBundle { style: Style { size: Size::new(Val::Px(150.0), Val::Px(65.0)),
        //                                                     // center button
        //                                                     margin: UiRect::all(Val::Auto),
        //                                                     // horizontally center child text
        //                                                     justify_content: JustifyContent::Center,
        //                                                     // vertically center child text
        //                                                     align_items: AlignItems::Center,
        //                                                     ..default() },
        //                                      color: Color::GRAY.into(),
        //                                      ..default() })
        //         .with_children(|parent| {
        //             parent.spawn_bundle(TextBundle::from_section("Button",
        //                                                          TextStyle { font:      asset_server.load("fonts/Calibri_Regular.ttf"),
        //                                                                      font_size: 40.0,
        //                                                                      color:     Color::BLACK, }));
        //         });

        for x in -10..=10 {
            for y in -10..=10 {
                let position = Vec2::new((x * 32) as f32, (y * 32) as f32);

                commands.spawn()
                        .insert_bundle(SpriteBundle { sprite: Sprite { color: Color::BLUE, ..default() },
                                                      transform: Transform { translation: position.extend(0.0),
                                                                             scale: Vec3::new(16., 16., 1.0),
                                                                             ..default() },
                                                      ..default() })
                        .insert(universe::Cell(false));
            }
        }
    }

    pub fn button_system(mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text>) {
        for (interaction, mut color, children) in &mut interaction_query {
            let mut text = text_query.get_mut(children[0]).unwrap();
            match *interaction {
                Interaction::Clicked => {
                    text.sections[0].value = "Press".to_string();
                    *color = Color::GREEN.into();
                },
                Interaction::Hovered => {
                    text.sections[0].value = "Hover".to_string();
                    *color = Color::WHITE.into();
                },
                Interaction::None => {
                    text.sections[0].value = "Button".to_string();
                    *color = Color::GRAY.into();
                },
            }
        }
    }
}

fn main() {
    App::new().add_plugins(DefaultPlugins)
              .insert_resource(WinitSettings::desktop_app())
              .add_startup_system(graphics::setup)
              .add_system(graphics::button_system)
              .run();
}
